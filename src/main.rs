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

use color::Color;
use std::fs::File;
use std::env;
use std::path::MAIN_SEPARATOR as PATH_SEP;
use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::stdin;
use std::io::Write;
use std::path::Path;
use getopts::Options;

mod macros;
mod aes;
mod commands;
mod ffi;
mod password;
mod color;

const ROOSTER_FILE_DEFAULT: &'static str = ".passwords.rooster";

static mut ROOSTER_EXIT: i32 = 0;

struct Command {
    name: &'static str,
    callback: fn(&getopts::Matches, &mut File) -> ()
}

static COMMANDS: &'static [Command] = &[
    Command { name: "get", callback: commands::get::callback },
    Command { name: "add", callback: commands::add::callback },
    Command { name: "delete", callback: commands::delete::callback },
    Command { name: "generate", callback: commands::generate::callback },
    Command { name: "regenerate", callback: commands::regenerate::callback },
    Command { name: "list", callback: commands::list::callback }
];

fn set_exit_status(status: i32) {
    unsafe { ROOSTER_EXIT = status; }
}

fn get_exit_status() -> i32 {
    unsafe { ROOSTER_EXIT }
}

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

fn get_password_file_from_input(filename: &str) -> IoResult<File> {
    println_stderr!(
        "I could not find the password file \"{}\". Would you like to create it now? [yes/no]",
        filename
    );
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => {
                if line.starts_with("yes") {
                    return open_password_file(filename, true);
                } else if line.starts_with("no") {
                    return Err(IoError::last_os_error());
                } else {
                    errln!(
                        "I did not understand that. Would you like to create \
                        the password file \"{}\" now? [yes/no]",
                        filename
                    );
                }
            },
            Err(_) => {
                errln!(
                    "I was not able to read your answer. Would you like to create \
                    the password file \"{}\" now? [yes/no]",
                    filename
                );
            }
        }
    }
}

fn get_password_file(filename: &str) -> IoResult<File> {
    match open_password_file(filename, false) {
        Ok(file) => Ok(file),
        Err(err) => {
            match err.kind() {
                IoErrorKind::NotFound => get_password_file_from_input(filename),
                _ => Err(err)
            }
        }
    }
}

fn execute_command_from_filename(matches: &getopts::Matches, command: &Command, filename: &str) {
    match get_password_file(filename) {
        Ok(ref mut file) => {
            (command.callback)(matches, file);
        },
        Err(err) => {
            match err.kind() {
                // This was already handled before.
                IoErrorKind::NotFound => {},
                _ => {
                    errln!("I could not open the password file \"{}\" :( ({})", filename, err);
                }
            }
            set_exit_status(1);
        }
    }
}

fn execute_command(matches: &getopts::Matches, command: &Command) {
    match env::var("ROOSTER_FILE") {
        Ok(filename) => {
            execute_command_from_filename(matches, command, filename.as_ref());
        },
        Err(env::VarError::NotPresent) => {
            match env::home_dir() {
                Some(path) => {
                    match path.as_os_str().to_os_string().into_string() {
                        Ok(ref mut filename) => {
                            filename.push(PATH_SEP);
                            filename.push_str(ROOSTER_FILE_DEFAULT);
                            execute_command_from_filename(matches, command, filename.as_ref());
                        },
                        Err(oss) => {
                            errln!("The password filename, {:?}, is invalid. It must be valid UTF8.", oss);
                            set_exit_status(1);
                        }
                    }
                },
                None => {
                    errln!("I couldn't figure out what file to use for the passwords.");
                    set_exit_status(1);
                }
            }
        },
        Err(env::VarError::NotUnicode(oss)) => {
            errln!("The password filename, {:?}, is invalid. It must be valid UTF8.", oss);
            set_exit_status(1);
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
        Err(err) => { panic!(err.to_string()) }
    };

    // Global help was requested.
    if matches.opt_present("h") && matches.free.is_empty() {
        usage();
        std::process::exit(get_exit_status());
    }

    // No command was given, this is abnormal.
    if matches.free.is_empty() {
        usage();
        std::process::exit(get_exit_status());
    }

    let command_name = matches.free.get(0).unwrap();
    match command_from_name(command_name.as_ref()) {
        Some(command) => {
            execute_command(&matches, command);
        },
        None => {
            errln!(
                "Woops, the command `{}` does not exist. Try the --help option for more info.",
                command_name
            );
            set_exit_status(1);
        }
    }

    std::process::exit(get_exit_status());
}
