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
use std::path::Path;
use std::ops::Deref;
use getopts::Options;
use rpassword::read_password;

mod macros;
mod aes;
mod commands;
mod ffi;
mod password;
mod color;
mod safe_string;

const ROOSTER_FILE_DEFAULT: &'static str = ".passwords.rooster";

struct Command {
    name: &'static str,
    callback_exec: fn(&getopts::Matches, &mut password::v2::PasswordStore, &str) -> Result<(), i32>,
    callback_help: fn(),
}

static COMMANDS: &'static [Command] = &[
    Command { name: "get", callback_exec: commands::get::callback_exec, callback_help: commands::get::callback_help },
    Command { name: "add", callback_exec: commands::add::callback_exec, callback_help: commands::add::callback_help },
    Command { name: "delete", callback_exec: commands::delete::callback_exec, callback_help: commands::delete::callback_help },
    Command { name: "generate", callback_exec: commands::generate::callback_exec, callback_help: commands::generate::callback_help },
    Command { name: "regenerate", callback_exec: commands::regenerate::callback_exec, callback_help: commands::regenerate::callback_help },
    Command { name: "list", callback_exec: commands::list::callback_exec, callback_help: commands::list::callback_help },
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
                    println_err!("I could not find the password file \"{}\". Would you like to create it now? [y/n]", filename);
                    let msg = format!("I did not understand that. Would you like to create the password file \"{}\" now? [y/n]", filename);
                    loop {
                        let mut line = String::new();
                        match stdin().read_line(&mut line) {
                            Ok(_) => {
                                if line.starts_with("y") {
                                    return open_password_file(filename, true);
                                } else if line.starts_with("n") {
                                    return Err(IoError::new(IoErrorKind::Other, "no password file available"));
                                } else {
                                    println_err!("{}", msg);
                                }
                            },
                            Err(_) => {
                                println_err!("{}", msg);
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
                write!(::std::io::stderr(), "Type your master password: ").unwrap();
                ::std::io::stderr().flush().unwrap();
                match read_password() {
                    Ok(ref mut master_password) => {
						// Upgrade the rooster file to the newest format supported.
						try!(password::upgrade(master_password.deref(), file).map_err(|_| 1));

                        let store = password::v2::PasswordStore::new(master_password.deref(), file);
                        let ret = (command.callback_exec)(matches, store);
                        master_password.scrub_memory();
                        return ret;
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

fn execute_command(matches: &getopts::Matches, command: &Command) -> Result<(), i32> {
    match env::var("ROOSTER_FILE") {
        Ok(filename) => {
            return execute_command_from_filename(matches, command, filename.as_ref());
        },
        Err(env::VarError::NotPresent) => {
            match env::home_dir() {
                Some(path) => {
                    match path.as_os_str().to_os_string().into_string() {
                        Ok(ref mut filename) => {
                            filename.push(PATH_SEP);
                            filename.push_str(ROOSTER_FILE_DEFAULT);
                            return execute_command_from_filename(matches, command, filename.as_ref());
                        },
                        Err(oss) => {
                            println_err!("The password filename, {:?}, is invalid. It must be valid UTF8.", oss);
                            return Err(1);
                        }
                    }
                },
                None => {
                    println_err!("I couldn't figure out what file to use for the passwords.");
                    return Err(1);
                }
            }
        },
        Err(env::VarError::NotUnicode(oss)) => {
            println_err!("The password filename, {:?}, is invalid. It must be valid UTF8.", oss);
            return Err(1);
        }
    };
}

fn usage() {
    println!("Welcome to Rooster, the simple password manager for geeks :-)");
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
    println!("    add               Add a new password.");
    println!("    delete            Delete a password");
    println!("    generate          Generate a password");
    println!("    regenerate        Re-generate a previously existing password");
    println!("    get               Retrieve a password");
    println!("    list              List all passwords");
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

    // Global help was requested.
    if matches.opt_present("h") && matches.free.is_empty() {
        usage();
        std::process::exit(0);
    }

    // No command was given, this is abnormal.
    if matches.free.is_empty() {
        usage();
        std::process::exit(1);
    }

    let command_name = matches.free.get(0).unwrap();
    match command_from_name(command_name.as_ref()) {
        Some(command) => {
            match execute_command(&matches, command) {
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
