// Copyright 2014 The Peevee Developers
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

#![feature(core)]
#![feature(exit_status)]
#![feature(convert)]
#![feature(collections)]
#![feature(old_io)]
#![feature(std_misc)]
#![feature(rustc_private)]
#![feature(str_char)]

extern crate libc;
extern crate getopts;
extern crate rustc_serialize;
extern crate crypto;
extern crate rpassword;
extern crate rand;

use color::Color;
use std::slice::AsSlice;
use std::fs::File;
use std::env;
use std::ffi::AsOsStr;
use std::path::MAIN_SEPARATOR as PATH_SEP;
use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::stdin;
use std::io::Write;
use std::path::Path;

mod macros;
mod aes;
mod commands;
mod ffi;
mod password;
mod color;

const PEEVEE_FILE_DEFAULT: &'static str = ".peevee_passwords.aes";

struct Command {
    name: &'static str,
    callback: fn(&[String], &mut File) -> ()
}

static COMMANDS: &'static [Command] = &[
    Command { name: "get", callback: commands::get::callback },
    Command { name: "add", callback: commands::add::callback },
    Command { name: "delete", callback: commands::delete::callback },
    Command { name: "generate", callback: commands::generate::callback },
    Command { name: "list", callback: commands::list::callback }
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

fn get_password_file_from_input(filename: &str) -> IoResult<File> {
    println_stderr!(
        "I could not find the password file \"{}\". Would you like to create it now? [yes/no]",
        filename
    );
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => {
                if line.as_slice().starts_with("yes") {
                    return open_password_file(filename, true);
                } else if line.as_slice().starts_with("no") {
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

fn execute_command_from_filename(args: &[String], command: &Command, filename: &str) {
    match get_password_file(filename) {
        Ok(ref mut file) => {
            (command.callback)(args, file);
        },
        Err(err) => {
            match err.kind() {
                // This was already handled before.
                IoErrorKind::NotFound => {},
                _ => {
                    errln!("I could not open the password file \"{}\" :( ({})", filename, err);
                }
            }
            std::env::set_exit_status(1);
        }
    }
}

fn execute_command(args: &[String], command: &Command) {
    match env::var("PEEVEE_FILE") {
        Ok(filename) => {
            execute_command_from_filename(args, command, filename.as_slice());
        },
        Err(env::VarError::NotPresent) => {
            match env::home_dir() {
                Some(path) => {
                    match path.as_os_str().to_os_string().into_string() {
                        Ok(ref mut filename) => {
                            filename.push(PATH_SEP);
                            filename.push_str(PEEVEE_FILE_DEFAULT);
                            execute_command_from_filename(args, command, filename.as_slice());
                        },
                        Err(oss) => {
                            errln!("The password filename, {:?}, is invalid. It must be valid UTF8.", oss);
                            std::env::set_exit_status(1);
                        }
                    }
                },
                None => {
                    errln!("I couldn't figure out what file to use for the passwords.");
                    std::env::set_exit_status(1);
                }
            }
        },
        Err(env::VarError::NotUnicode(oss)) => {
            errln!("The password filename, {:?}, is invalid. It must be valid UTF8.", oss);
            std::env::set_exit_status(1);
        }
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.as_slice().get(1) {
        Some(command_name) => {
            match command_from_name(command_name.as_slice()) {
                Some(command) => {
                    execute_command(args.as_slice(), command);
                },
                None => {
                    errln!(
                        "I don't know the \"{}\" command. You can check \
                        existing commands at: https://github.com/conradkleinespel/peevee-cli.",
                        command_name
                    );
                    std::env::set_exit_status(1);
                }
            }
        },
        None => {
            errln!(
                "I didn't understand that. You can check the documentation \
                for Peevee at: https://github.com/conradkleinespel/peevee-cli"
            );
            std::env::set_exit_status(1);
        }
    }
}
