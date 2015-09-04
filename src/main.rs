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

const PEEVEE_FILE_DEFAULT: &'static str = ".peevee_passwords.aes";

static mut PEEVEE_EXIT: i32 = 0;

struct Command {
    name: &'static str,
    callback: fn(&getopts::Matches, &mut File) -> ()
}

static COMMANDS: &'static [Command] = &[
    Command { name: "get", callback: commands::get::callback },
    Command { name: "add", callback: commands::add::callback },
    Command { name: "delete", callback: commands::delete::callback },
    Command { name: "generate", callback: commands::generate::callback },
    Command { name: "list", callback: commands::list::callback }
];

fn set_exit_status(status: i32) {
    unsafe { PEEVEE_EXIT = status; }
}

fn get_exit_status() -> i32 {
    unsafe { PEEVEE_EXIT }
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
    match env::var("PEEVEE_FILE") {
        Ok(filename) => {
            execute_command_from_filename(matches, command, filename.as_ref());
        },
        Err(env::VarError::NotPresent) => {
            match env::home_dir() {
                Some(path) => {
                    match path.as_os_str().to_os_string().into_string() {
                        Ok(ref mut filename) => {
                            filename.push(PATH_SEP);
                            filename.push_str(PEEVEE_FILE_DEFAULT);
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
    println!("Welcome to Peevee, the simple password manager :-)");
    println!("");
    println!("Usage:");
    println!("    peevee --help");
    println!("    peevee [options] <command> [<args> ...]");
    println!("    peevee --help <command>");
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
    println!("    get               Retrieve a password");
    println!("    list              List all passwords");
    println!("");
    println!("Extended help for commands:");
    println!("    You can view extended documentation for a specific command with the --help");
    println!("    option followed by the command name. For instance:");
    println!("        peevee --help get");
    println!("    This will display what arguments are available for the 'get' command.");
    println!("");
    println!("App names are case insensitive:");
    println!("    The app names you set for your passwords are case insensitive. This means you");
    println!("    cannot have two passwords for a single app name. For example, if you have 2");
    println!("    Google accounts, you need to save them as something like 'Google-Personnal'");
    println!("    and 'Google-Pro'. The upside to this is that you don't need to remember the");
    println!("    case of the app name when searching for a password. You can just type:");
    println!("        peevee get google-pro");
    println!("    Nevertheless, for improved readability, the 'list' command will display app");
    println!("    names exactly how you type them when using the 'add' or 'generate' commands.");
    println!("");
    println!("Cloud sync:");
    println!("    Peevee supports online syncing of passwords. It is not built into Peevee. But");
    println!("    because Peevee uses an encrypted file for password storage, you can put this");
    println!("    file in your Dropbox folder (or any other folder that gets synced) and you");
    println!("    should be good to go.");
    println!("");
    println!("Setting the password file path:");
    println!("    By default, Peevee will try to use the file pointed to by the PEEVEE_FILE");
    println!("    environment variable. This allows you to set the password file to whatever");
    println!("    you want, via your shell configuration. For instance, if you are using Bash,");
    println!("    you can add the following line to your ~/.bashrc to set the password file:");
    println!("        export PEEVEE_FILE=$HOME/Dropbox/passwords.peevee");
    println!("    If the environment variable PEEVEE_FILE is not set, Peevee will automatically");
    println!("    look for ~/.peevee_passwords.aes.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Display this help message.");
    opts.optflag("a", "alnum", "Only use alphanumeric characters in the generated password.");
    opts.optopt("l", "length", "Length of the generated password.", "32");

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
