// Copyright 2014 The Rooster Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate libc;
extern crate getopts;
extern crate rustc_serialize;
extern crate crypto;
extern crate rpassword;
extern crate rand;
extern crate byteorder;

use std::fs::File;
use std::env;
use std::path::MAIN_SEPARATOR as PATH_SEP;
use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::stdin;
use std::io::Write;
use std::io::Read;
use std::path::Path;
use getopts::Options;
use rpassword::read_password;
use safe_string::SafeString;
use safe_vec::SafeVec;
use std::ops::Deref;

mod macros;
mod aes;
mod commands;
mod ffi;
mod password;
mod color;
mod safe_string;
mod safe_vec;
mod generate;

const ROOSTER_FILE_ENV_VAR: &'static str = "ROOSTER_FILE";
const ROOSTER_FILE_DEFAULT: &'static str = ".passwords.rooster";

struct Command {
    name: &'static str,
    callback_exec: fn(&getopts::Matches, &mut password::v2::PasswordStore) -> Result<(), i32>,
    callback_help: fn(),
}

static COMMANDS: &'static [Command] = &[
    Command { name: "get", callback_exec: commands::get::callback_exec, callback_help: commands::get::callback_help },
    Command { name: "add", callback_exec: commands::add::callback_exec, callback_help: commands::add::callback_help },
    Command { name: "delete", callback_exec: commands::delete::callback_exec, callback_help: commands::delete::callback_help },
    Command { name: "generate", callback_exec: commands::generate::callback_exec, callback_help: commands::generate::callback_help },
    Command { name: "regenerate", callback_exec: commands::regenerate::callback_exec, callback_help: commands::regenerate::callback_help },
    Command { name: "list", callback_exec: commands::list::callback_exec, callback_help: commands::list::callback_help },
    Command { name: "export", callback_exec: commands::export::callback_exec, callback_help: commands::export::callback_help },
    Command { name: "change-master-password", callback_exec: commands::change_master_password::callback_exec, callback_help: commands::change_master_password::callback_help },
];

fn command_from_name(name: &str) -> Option<&'static Command> {
    for c in COMMANDS.iter() {
        if c.name == name {
            return Some(&c);
        }
    }
    None
}

fn open_password_file(filename: &str, create: bool) -> IoResult<File> {
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    options.write(true);
    options.create(create);
    options.open(&Path::new(filename))
}

fn get_password_file(filename: &str) -> IoResult<File> {
    match open_password_file(filename, false) {
        Ok(file) => Ok(file),
        Err(err) => {
            match err.kind() {
                IoErrorKind::NotFound => {
                    println_err!("I cannot find a password file at \"{}\". Would you like to create one now? [y/n]", filename);
                    loop {
                        let mut line = String::new();
                        match stdin().read_line(&mut line) {
                            Ok(_) => {
                                if line.starts_with("y") {
                                    return open_password_file(filename, true);
                                } else if line.starts_with("n") {
                                    return Err(IoError::new(IoErrorKind::Other, "no password file available"));
                                } else {
                                    println_stderr!("I did not get that. Create a password file now? [y/n]");
                                }
                            },
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    }
                },
                _ => Err(err)
            }
        }
    }
}

fn execute_command_from_filename(matches: &getopts::Matches, command: &Command, filename: &str) -> Result<(), i32> {
    match get_password_file(filename) {
        Ok(ref mut file) => {
            if matches.opt_present("help") {
                (command.callback_help)();
                return Ok(());
            } else {
                print_stderr!("Type your master password: ");
                match read_password() {
                    Ok(master_password) => {
                        let master_password = SafeString::new(master_password);
                        let mut input: Vec<u8> = Vec::new();
                        try!(file.read_to_end(&mut input).map_err(|_| 1));

                        // Try to open the file as is.
                        let mut store = match password::v2::PasswordStore::from_input(master_password.clone(), SafeVec::new(input.clone())) {
                            Ok(store) => store,
                            Err(_) => {
                                // If we can't open the file, we may need to upgrade its format first.
                                match password::upgrade(master_password.clone(), SafeVec::new(input.clone())) {
                                    Ok(store) => store,
                                    Err(_) => {
                                        // If we can't upgrade its format either, we show a helpful
                                        // error message.
                                        println_err!("I could not upgrade the Rooster file. This could be because:");
                                        println_err!("- your version of Rooster is outdated,");
                                        println_err!("- your Rooster file is corrupted,");
                                        println_err!("- your master password is wrong.");
                                        println_err!("Try upgrading to the latest version of Rooster.");
                                        return Err(1);
                                    }
                                }
                            }
                        };

                        // Execute the command and save the new password list
                        try!((command.callback_exec)(matches, &mut store));

                        match store.sync(file) {
                            Ok(()) => { Ok(()) },
                            Err(err) => {
                                println_err!("I could not save the password file ({:?}).", err);
                                return Err(1);
                            }
                        }
                    },
                    Err(err) => {
                        println_err!("I could not read your master password ({})", err);
                        return Err(1);
                    }
                }
            }
        },
        Err(err) => {
            println_err!("I could not open the password file \"{}\" :( ({})", filename, err);
            return Err(1);
        }
    }
}

fn get_password_file_path() -> Result<String, i32> {
    match env::var(ROOSTER_FILE_ENV_VAR) {
        Ok(filename) => {
            Ok(filename)
        },
        Err(env::VarError::NotPresent) => {
            let mut filename = match env::home_dir() {
                Some(home) => {
                    try!(home.as_os_str().to_os_string().into_string().map_err(|_| 1))
                }
                None => {
                    return Err(1);
                }
            };
            filename.push(PATH_SEP);
            filename.push_str(ROOSTER_FILE_DEFAULT);
            Ok(filename)
        },
        Err(env::VarError::NotUnicode(_)) => {
            Err(1)
        }
    }
}

fn usage(password_file: &str) {
    println!("Welcome to Rooster, the simple password manager for geeks :-)");
    println!("");
    println!("The current password file is: {}", password_file);
    println!("You may override this path in the $ROOSTER_FILE environment variable.");
    println!("");
    println!("Usage:");
    println!("    rooster -h");
    println!("    rooster [options] <command> [<args> ...]");
    println!("    rooster <command> -h");
    println!("");
    println!("Options:");
    println!("    -h, --help        Display a help message");
    println!("    -a, --alnum       Only use alpha numeric (a-z, A-Z, 0-9) in generated passwords");
    println!("    -l, --length      Set a custom length for the generated password, default is 32");
    println!("");
    println!("Commands:");
    println!("    add                        Add a new password");
    println!("    delete                     Delete a password");
    println!("    generate                   Generate a password");
    println!("    regenerate                 Re-generate a previously existing password");
    println!("    get                        Retrieve a password");
    println!("    list                       List all apps and usernames");
    println!("    export                     List all passwords in unencrypted JSON");
    println!("    change-master-password     Change your master password");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Display a help message");
    opts.optflag("a", "alnum", "Only use alpha numeric (a-z, A-Z, 0-9) in generated passwords");
    opts.optopt("l", "length", "Set a custom length for the generated password", "32");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(err) => {
            println_err!("{}", err);
             std::process::exit(1);
        }
    };

    // Fetch the Rooster file path now, so we can display it in help messages.
    let password_file_path = match get_password_file_path() {
        Ok(path) => path,
        Err(_) => {
            println_err!("Woops, I could not determine where your password file is.");
            println_err!("I recommend you try setting the $ROOSTER_FILE environment");
            println_err!("variable with the absolute path to your password file.");
            std::process::exit(1);
        }
    };

    // Global help was requested.
    if matches.opt_present("h") && matches.free.is_empty() {
        usage(password_file_path.deref());
        std::process::exit(0);
    }

    // No command was given, this is abnormal, so we'll show the docs.
    let command_name = match matches.free.get(0) {
        Some(command_name) => command_name,
        None => {
            usage(password_file_path.deref());
            std::process::exit(1);
        }
    };

    match command_from_name(command_name.as_ref()) {
        Some(command) => {
            match execute_command_from_filename(&matches, command, password_file_path.deref()) {
                Err(i) => std::process::exit(i),
                _ => std::process::exit(0)
            }
        },
        None => {
            println_err!(
                "Woops, the command `{}` does not exist. Try the --help option for more info.",
                command_name
            );
            std::process::exit(1);
        }
    }
}
