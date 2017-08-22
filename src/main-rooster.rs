// Copyright 2014-2017 The Rooster Developers
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

// #![allow(useless_format, too_many_arguments)]

extern crate libc;
extern crate getopts;
extern crate crypto;
extern crate rpassword;
extern crate rand;
extern crate byteorder;
extern crate quale;
extern crate serde;
extern crate serde_json;
extern crate clipboard;
extern crate shell_escape;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::env;
use std::env::VarError;
use std::path::MAIN_SEPARATOR as PATH_SEP;
use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::Write;
use std::io::Read;
use std::path::{Path, PathBuf};
use getopts::Options;
use rpassword::prompt_password_stderr;
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
mod clip;
mod list;

const ROOSTER_FILE_ENV_VAR: &'static str = "ROOSTER_FILE";
const ROOSTER_FILE_DEFAULT: &'static str = ".passwords.rooster";
const DONT_CREATE_PASSWORD_FILE: &'static str = "DONT_CREATE_PASSWORD_FILE";
const FAIL_READING_NEW_PASSWORD: &'static str = "FAIL_READING_NEW_PASSWORD";

struct Command {
    name: &'static str,
    callback_exec:
        Option<fn(&getopts::Matches, &mut password::v2::PasswordStore) -> Result<(), i32>>,
    callback_help: fn(),
    callback_without_store: Option<fn(&getopts::Matches) -> Result<(), i32>>,
}

static COMMANDS: &'static [Command] = &[
    Command {
        name: "get",
        callback_exec: Some(commands::get::callback_exec),
        callback_help: commands::get::callback_help,
        callback_without_store: Some(commands::get::check_args),
    },
    Command {
        name: "add",
        callback_exec: Some(commands::add::callback_exec),
        callback_help: commands::add::callback_help,
        callback_without_store: Some(commands::add::check_args),
    },
    Command {
        name: "delete",
        callback_exec: Some(commands::delete::callback_exec),
        callback_help: commands::delete::callback_help,
        callback_without_store: Some(commands::delete::check_args),
    },
    Command {
        name: "generate",
        callback_exec: Some(commands::generate::callback_exec),
        callback_help: commands::generate::callback_help,
        callback_without_store: Some(commands::generate::check_args),
    },
    Command {
        name: "regenerate",
        callback_exec: Some(commands::regenerate::callback_exec),
        callback_help: commands::regenerate::callback_help,
        callback_without_store: Some(commands::regenerate::check_args),
    },
    Command {
        name: "list",
        callback_exec: Some(commands::list::callback_exec),
        callback_help: commands::list::callback_help,
        callback_without_store: None,
    },
    Command {
        name: "import",
        callback_exec: Some(commands::import::callback_exec),
        callback_help: commands::import::callback_help,
        callback_without_store: None,
    },
    Command {
        name: "export",
        callback_exec: Some(commands::export::callback_exec),
        callback_help: commands::export::callback_help,
        callback_without_store: None,
    },
    Command {
        name: "set-master-password",
        callback_exec: Some(commands::set_master_password::callback_exec),
        callback_help: commands::set_master_password::callback_help,
        callback_without_store: None,
    },
    Command {
        name: "rename",
        callback_exec: Some(commands::rename::callback_exec),
        callback_help: commands::rename::callback_help,
        callback_without_store: Some(commands::rename::check_args),
    },
    Command {
        name: "transfer",
        callback_exec: Some(commands::transfer::callback_exec),
        callback_help: commands::transfer::callback_help,
        callback_without_store: Some(commands::transfer::check_args),
    },
    Command {
        name: "change",
        callback_exec: Some(commands::change::callback_exec),
        callback_help: commands::change::callback_help,
        callback_without_store: Some(commands::change::check_args),
    },
    Command {
        name: "uninstall",
        callback_exec: None,
        callback_help: commands::uninstall::callback_help,
        callback_without_store: Some(commands::uninstall::callback_exec),
    },
];

fn command_from_name(name: &str) -> Option<&'static Command> {
    for c in COMMANDS.iter() {
        if c.name == name {
            return Some(c);
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

// Look for Dropbox folder.
//
// If you want support for other cloud services, please open an issue
// and we'll see if we can add support for it.
//
// TODO: This is naive implementation that only works properly with the
// default "$HOME/Dropbox" folder. But if you use custom Dropbox folder
// locations, then Rooster doesn't know what to do. It is possible to
// programatically know where Dropbox stores files:
// https://www.dropbox.com/help/4584
fn get_dropbox_folder() -> Option<PathBuf> {
    match env::home_dir() {
        Some(mut dropbox_folder) => {
            dropbox_folder.push("Dropbox");

            if dropbox_folder.exists() {
                Some(dropbox_folder)
            } else {
                None
            }
        }
        None => None,
    }
}

fn get_password_file(
    filename: &str,
    show_running_rooster_msg: bool,
) -> IoResult<(Option<SafeString>, File)> {
    match open_password_file(filename, false) {
        Ok(file) => {
            if show_running_rooster_msg {
                println_stderr!("");
                println_title!("|---- All set! Running Rooster now... ---|");
                println_stderr!("");
            }

            Ok((None, file))
        }
        Err(err) => {
            match err.kind() {
                IoErrorKind::NotFound => {
                    let mut show_default_no_file_msg = true;

                    if let Some(dropbox_folder) = get_dropbox_folder() {
                        let mut file_in_dropbox = dropbox_folder.clone();
                        file_in_dropbox.push(ROOSTER_FILE_DEFAULT);

                        if file_in_dropbox.exists() {
                            println_title!("|---------------- Dropbox ---------------|");
                            println_stderr!("");
                            println_stderr!(
                                "Seems like you have a Rooster file in your Dropbox \
                                             folder."
                            );
                            println_stderr!("");
                            println_stderr!(
                                "It is located at: ~/Dropbox/{}.",
                                ROOSTER_FILE_DEFAULT
                            );

                            println_stderr!("");
                            print_stderr!("Is that your correct password file (y/n)? ");
                            let mut line = String::new();
                            std::io::stdin().read_line(&mut line)?;
                            if line.starts_with('y') {
                                println_stderr!("");
                                println_title!("|------------- Configuration ------------|");
                                println_stderr!("");
                                println_stderr!(
                                    "You might want to add this to your shell config \
                                                 (.bashrc, .zshrc, etc):"
                                );
                                println_stderr!(
                                    "    export ROOSTER_FILE={}",
                                    file_in_dropbox.to_string_lossy()
                                );
                                println_stderr!("");
                                println_stderr!(
                                    "This way, I won't ask you if this is the right \
                                                 file every time."
                                );

                                println_stderr!("");
                                return get_password_file(
                                    file_in_dropbox.to_string_lossy().as_ref(),
                                    true,
                                );
                            }

                            show_default_no_file_msg = false;
                            println_stderr!("");
                            println_title!("|----------- New password file ----------|");
                            println_stderr!("");
                            print_stderr!(
                                "OK. Would you like to create a new \
                                             password file now (y/n)? "
                            );
                        }
                    }

                    loop {
                        if show_default_no_file_msg {
                            println_title!("|----------- New password file ----------|");
                            println_stderr!("");
                            println_stderr!(
                                "I can't find your password file. This is expected \
                                             if you are using Rooster for the first time."
                            );
                            println_stderr!("");
                            print_stderr!("Would you like to create a password file now (y/n)? ");
                        }

                        let mut line = String::new();
                        std::io::stdin().read_line(&mut line)?;
                        if line.starts_with('y') {
                            println_stderr!("");
                            println_stderr!(
                                "Alright, will do! But first, there is some stuff we \
                                             have to take care of."
                            );
                            println_stderr!("");
                            println_title!("|---------- Set Master Password ---------|");
                            println_stderr!("");
                            println_stderr!(
                                "In order to keep your passwords safe & \
                                             secure, we encrypt them using a Master \
                                             Password."
                            );
                            println_stderr!("");
                            println_stderr!(
                                "The stronger it is, the better your passwords are \
                                             protected."
                            );
                            println_stderr!("");

                            let master_password = prompt_password_stderr(
                                "What would you like it \
                                                                          to be? ",
                            );
                            let master_password =
                                master_password.map(SafeString::new).map_err(|_| {
                                    IoError::new(IoErrorKind::Other, FAIL_READING_NEW_PASSWORD)
                                })?;

                            let mut filename = filename.to_owned();

                            // Maybe the user wants their Rooster file in Dropbox.
                            if let Some(folder) = get_dropbox_folder() {
                                println_stderr!("");
                                println_title!("|---------------- Dropbox ---------------|");

                                println_stderr!("");
                                println_stderr!("Seems like you're using Dropbox.");

                                println_stderr!("");
                                print_stderr!(
                                    "Would you like to add your password file to \
                                               Dropbox (y/n)? "
                                );
                                let mut line = String::new();
                                std::io::stdin().read_line(&mut line)?;
                                if line.starts_with('y') {
                                    filename = format!(
                                        "{}/{}",
                                        folder.to_string_lossy(),
                                        ROOSTER_FILE_DEFAULT
                                    );

                                    println_stderr!("");
                                    println_title!("|------------- Configuration ------------|");
                                    println_stderr!("");
                                    println_stderr!(
                                        "You'll need to add this to your shell \
                                                     config (.bashrc, .zshrc, etc):"
                                    );
                                    println_stderr!("    export ROOSTER_FILE={}", filename);

                                    if let Some(previous) = env::var(ROOSTER_FILE_ENV_VAR).ok() {
                                        println_stderr!("");
                                        println_stderr!(
                                            "You'll also need to delete your \
                                                         previous Rooster file configuration. It \
                                                         probably looks something like this:"
                                        );
                                        println_stderr!("    export ROOSTER_FILE={}", previous);
                                    }
                                }
                            }

                            let password_file = open_password_file(filename.as_str(), true)?;

                            println_stderr!("");
                            println_title!("|---- All set! Running Rooster now... ---|");
                            println_stderr!("");

                            return Ok((Some(master_password), password_file));
                        } else if line.starts_with('n') {
                            return Err(IoError::new(IoErrorKind::Other, DONT_CREATE_PASSWORD_FILE));
                        } else {
                            println_stderr!(
                                "I didn't get that. Should I create a password file \
                                             now (y/n)? "
                            );
                        }
                    }
                }
                _ => Err(err),
            }
        }
    }
}

fn get_password_store(
    file: &mut File,
    new_master_password: Option<SafeString>,
) -> Result<password::v2::PasswordStore, i32> {
    // If there was no password file, return early with an empty store
    match new_master_password {
        Some(p) => return password::v2::PasswordStore::new(p.clone()).map_err(|_| 1),
        None => {}
    }

    // Read the Rooster file contents.
    let mut input: SafeVec = SafeVec::new(Vec::new());
    file.read_to_end(input.inner_mut()).map_err(|_| 1)?;

    // We'll ask the master password 3 times before considering that the Rooster file
    // is corrupted and telling the user about it.
    let mut number_allowed_fails = 3 - 1;
    loop {
        let master_password = match ask_master_password() {
            Ok(p) => p,
            Err(err) => {
                println_err!(
                    "Woops, I could not read your master password (reason: {}).",
                    err
                );
                std::process::exit(1);
            }
        };

        // Try to open the file as is.
        match password::v2::PasswordStore::from_input(master_password.clone(), input.clone()) {
            Ok(store) => {
                return Ok(store);
            }
            Err(password::PasswordError::CorruptionError) => {
                println_err!("Your Rooster file is corrupted.");
                return Err(1);
            }
            Err(err) => {
                // Try again.
                if number_allowed_fails > 0 {
                    number_allowed_fails = number_allowed_fails - 1;
                    println_err!("Woops, that's not the right password. Let's try again.");
                    continue;
                }

                match err {
                    password::PasswordError::WrongVersionError => {
                        // If we can't open the file, we may need to upgrade its format first.
                        match password::upgrade(master_password.clone(), input.clone()) {
                            Ok(store) => {
                                return Ok(store);
                            }
                            Err(err) => {
                                // Try again.
                                if number_allowed_fails > 0 {
                                    number_allowed_fails = number_allowed_fails - 1;
                                    println_err!(
                                        "Woops, that's not the right password. Let's \
                                                  try again."
                                    );
                                    continue;
                                }
                                match err {
                                    password::PasswordError::WrongVersionError => {
                                        println_err!(
                                            "I could not open the Rooster file because \
                                                      your version of Rooster is outdated."
                                        );
                                        println_err!(
                                            "Try upgrading Rooster to the latest \
                                                      version."
                                        );

                                        return Err(1);
                                    }
                                    password::PasswordError::Io(err) => {
                                        println_err!(
                                            "I couldn't open your Rooster file (reason: \
                                                      {:?})",
                                            err
                                        );

                                        return Err(1);
                                    }
                                    _ => {
                                        println_err!(
                                            "Decryption of your Rooster file keeps \
                                                      failing. This is a sign that your Rooster \
                                                      file is probably corrupted."
                                        );

                                        return Err(1);
                                    }
                                }
                            }
                        }
                    }
                    password::PasswordError::Io(err) => {
                        println_err!("I couldn't open your Rooster file (reason: {:?})", err);

                        return Err(1);
                    }
                    _ => {
                        println_err!(
                            "Decryption of your Rooster file keeps failing. This is a \
                                      sign that your Rooster file is probably corrupted."
                        );

                        return Err(1);
                    }
                }
            }
        }
    }
}

fn execute_command_from_filename(
    matches: &getopts::Matches,
    command: &Command,
    file: &mut File,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    // Execute the command and save the new password list
    match command.callback_exec {
        Some(cb) => {
            (cb)(matches, store)?;
        }
        None => {}
    }

    match store.sync(file) {
        Ok(()) => { Ok(()) }
        Err(err) => {
            println_err!("I could not save the password file (reason: {:?}).", err);
            Err(1)
        }
    }
}

fn get_password_file_path() -> Result<String, i32> {
    let rooster_file = env::var(ROOSTER_FILE_ENV_VAR);
    let home_dir = env::home_dir();

    match rooster_file {
        Ok(filename) => Ok(filename),
        Err(VarError::NotPresent) => {
            let mut filename = match home_dir {
                Some(home) => home.as_os_str().to_os_string().into_string().map_err(|_| 1)?,
                None => {
                    return Err(1);
                }
            };
            filename.push(PATH_SEP);
            filename.push_str(ROOSTER_FILE_DEFAULT);
            Ok(filename)
        }
        Err(VarError::NotUnicode(_)) => Err(1),
    }
}

fn ask_master_password() -> IoResult<SafeString> {
    prompt_password_stderr("Type your master password: ").map(SafeString::new)
}

fn usage(password_file: &str) {
    println!("Welcome to Rooster, the simple password manager for geeks :-)");
    println!();
    println!("The current password file is: {}", password_file);
    println!("You may override this path in the $ROOSTER_FILE environment variable.");
    println!("");
    println!("Usage:");
    println!("    rooster -h");
    println!("    rooster [options] <command> [<args> ...]");
    println!("    rooster <command> -h");
    println!();
    println!("Options:");
    println!("    -h, --help        Display a help message");
    println!("    -v, --version     Display the version of Rooster you are using");
    println!("    -a, --alnum       Only use alpha numeric (a-z, A-Z, 0-9) in generated passwords");
    println!("    -l, --length      Set a custom length for the generated password, default is 32");
    println!("    -s, --show        Show the password instead of copying it to the clipboard");
    println!();
    println!("Commands for everyday use:");
    println!("    add                        Add a new password manually");
    println!("    change                     Change a password manually");
    println!("    delete                     Delete a password");
    println!("    generate                   Generate a password");
    println!("    regenerate                 Regenerate a previously existing password");
    println!("    get                        Retrieve a password");
    println!("    rename                     Rename the app for a password");
    println!("    transfer                   Change the username for a password");
    println!("    list                       List all apps and usernames");
    println!("    import                     Load all your raw password data from JSON file");
    println!("    export                     Dump all your raw password data in JSON");
    println!("    set-master-password        Set your master password");
    println!("    uninstall                  Show instructions to uninstall Rooster");
    println!("");
    println!("Some commands (change, delete, regenerate, get, rename, transfer)");
    println!("support fuzzy search of passwords:");
    println!("    rooster get google");
    println!("    rooster get ggl");
    println!("");
    println!("If multiple passwords match your search, you will be asked to choose.")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Display a help message");
    opts.optflag(
        "v",
        "version",
        "Display the version of Rooster you are using",
    );
    opts.optflag(
        "a",
        "alnum",
        "Only use alpha numeric (a-z, A-Z, 0-9) in generated passwords",
    );
    opts.optopt(
        "l",
        "length",
        "Set a custom length for the generated password",
        "32",
    );
    opts.optflag(
        "s",
        "show",
        "Show the password instead of copying it to the clipboard",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
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
    if matches.opt_present("help") && matches.free.is_empty() {
        usage(password_file_path.deref());
        std::process::exit(0);
    }

    if matches.opt_present("version") {
        println!("v{}", env!("CARGO_PKG_VERSION"));
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

    let command: &Command = match command_from_name(command_name.as_ref()) {
        Some(command) => command,
        None => {
            println_err!(
                "Woops, the command `{}` does not exist. Try the --help option for more \
                          info.",
                command_name
            );
            std::process::exit(1);
        }
    };

    if matches.opt_present("help") {
        (command.callback_help)();
        std::process::exit(0);
    }

    match command.callback_without_store {
        Some(cb) => {
            match (cb)(&matches) {
                Err(i) => {
                    std::process::exit(i);
                }
                Ok(_) => {}
            };
        }
        None => {}
    }


    if command.callback_exec.is_some() {
        let (new_master_password, mut file) =
            match get_password_file(password_file_path.deref(), false) {
                Ok(file) => file,
                Err(err) => {
                    if format!("{}", err) == DONT_CREATE_PASSWORD_FILE {
                        println_err!("I can't go on without a password file, sorry");
                    } else if format!("{}", err) == FAIL_READING_NEW_PASSWORD {
                        println_err!("I couldn't read your Master Password, sorry");
                    } else {
                        println_err!(
                            "I can't find your password file at {} (reason: {})",
                            password_file_path,
                            err
                        );
                    }
                    std::process::exit(1);
                }
            };

        let mut store = match get_password_store(&mut file, new_master_password) {
            Err(i) => std::process::exit(i),
            Ok(store) => store,
        };

        match execute_command_from_filename(&matches, command, &mut file, &mut store) {
            Err(i) => std::process::exit(i),
            _ => std::process::exit(0),
        }
    }
}
