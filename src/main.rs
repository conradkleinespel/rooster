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
extern crate zxcvbn;
extern crate dirs;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::env;
use std::env::VarError;
use std::io::Result as IoResult;
use std::io::{Write, Read, stdin};
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
        callback_without_store: Some(commands::import::check_args),
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
    Command {
        name: "init",
        callback_exec: None,
        callback_help: commands::init::callback_help,
        callback_without_store: Some(commands::init::callback_exec),
    },
    Command {
        name: "weak",
        callback_exec: Some(commands::weak::callback_exec),
        callback_help: commands::weak::callback_help,
        callback_without_store: None,
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

#[cfg(test)]
mod test {
    use super::command_from_name;

    #[test]
    fn test_command_from_name_returns_none_if_not_exists() {
        assert!(command_from_name("haha").is_none());
    }

    #[test]
    fn test_command_from_name_returns_some_if_exists() {
        assert!(command_from_name("get").is_some());
    }
}


fn open_password_file(filename: &str) -> IoResult<File> {
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    options.write(true);
    options.create(false);
    options.open(&Path::new(filename))
}

fn create_password_file(filename: &str) -> IoResult<File> {
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    options.write(true);
    options.create(true);
    options.open(&Path::new(filename))
}

fn get_password_store(file: &mut File) -> Result<password::v2::PasswordStore, i32> {
    // Read the Rooster file contents.
    let mut input: SafeVec = SafeVec::new(Vec::new());
    file.read_to_end(input.inner_mut()).map_err(|_| 1)?;

    return get_password_store_from_input_interactive(&input, 3, false, false).map_err(|_| 1);
}

fn get_password_store_from_input_interactive(
    input: &SafeVec,
    retries: i32,
    force_upgrade: bool,
    retry: bool,
) -> Result<password::v2::PasswordStore, password::PasswordError> {
    if retries == 0 {
        println_err!(
            "Decryption of your Rooster file keeps failing. \
            Your Rooster file is probably corrupted."
        );
        return Err(password::PasswordError::CorruptionLikelyError);
    }

    if retry {
        println_err!("Woops, that's not the right password. Let's try again.");
    }

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

    match get_password_store_from_input(&input, &master_password, force_upgrade) {
        Ok(store) => {
            return Ok(store);
        }
        Err(password::PasswordError::CorruptionError) => {
            println_err!("Your Rooster file is corrupted.");
            return Err(password::PasswordError::CorruptionError);
        }
        Err(password::PasswordError::OutdatedRoosterBinaryError) => {
            println_err!(
                "I could not open the Rooster file because your version of Rooster is outdated."
            );
            println_err!("Try upgrading Rooster to the latest version.");
            return Err(password::PasswordError::OutdatedRoosterBinaryError);
        }
        Err(password::PasswordError::Io(err)) => {
            println_err!("I couldn't open your Rooster file (reason: {:?})", err);
            return Err(password::PasswordError::Io(err));
        }
        Err(password::PasswordError::NeedUpgradeErrorFromV1) => {
            println!("Your Rooster file has version 1. You need to upgrade to version 2.");
            println!();
            println!(
                "WARNING: If in doubt, it could mean you've been hacked. Only \
            proceed if you recently upgraded your Rooster installation."
            );
            println!();
            println!("Upgrade to version 2? [y/n]");
            loop {
                let mut line = String::new();
                match stdin().read_line(&mut line) {
                    Ok(_) => {
                        if line.starts_with('y') {
                            // This time we'll try to upgrade
                            return get_password_store_from_input_interactive(
                                &input,
                                retries,
                                true,
                                false,
                            );
                        } else if line.starts_with('n') {
                            // The user doesn't want to upgrade, that's fine
                            return Err(password::PasswordError::NoUpgradeError);
                        } else {
                            println!("I did not get that. Upgrade from v1 to v2? [y/n]");
                        }
                    }
                    Err(io_err) => {
                        println_err!(
                            "Woops, an error occured while reading your response (reason: {:?}).",
                            io_err
                        );
                        return Err(password::PasswordError::Io(io_err));
                    }
                }
            }
        }
        _ => {
            return get_password_store_from_input_interactive(&input, retries - 1, false, true);
        }
    }
}

fn get_password_store_from_input(
    input: &SafeVec,
    master_password: &SafeString,
    upgrade: bool,
) -> Result<password::v2::PasswordStore, password::PasswordError> {
    // Try to open the file as is.
    match password::v2::PasswordStore::from_input(master_password.clone(), input.clone()) {
        Ok(store) => {
            return Ok(store);
        }
        Err(password::PasswordError::CorruptionError) => {
            return Err(password::PasswordError::CorruptionError);
        }
        Err(password::PasswordError::OutdatedRoosterBinaryError) => {
            return Err(password::PasswordError::OutdatedRoosterBinaryError);
        }
        Err(password::PasswordError::NeedUpgradeErrorFromV1) => {
            if !upgrade {
                return Err(password::PasswordError::NeedUpgradeErrorFromV1);
            }

            // If we can't open the file, we may need to upgrade its format first.
            match password::upgrade(master_password.clone(), input.clone()) {
                Ok(store) => {
                    return Ok(store);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Err(err) => {
            return Err(err);
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
    if let Some(cb) = command.callback_exec {
        (cb)(matches, store)?;
    }

    match store.sync(file) {
        Ok(()) => { Ok(()) }
        Err(err) => {
            println_err!("I could not save the password file (reason: {:?}).", err);
            Err(1)
        }
    }
}

/// Returns (filename: PathBuf, from_env_var: bool)
fn get_password_file_path() -> Result<(PathBuf, bool), i32> {
    // First, look for the ROOSTER_FILE environment variable.
    let rooster_file_from_env_var = env::var(ROOSTER_FILE_ENV_VAR);
    let (path, from_env) = match rooster_file_from_env_var {
        Ok(filename) => (PathBuf::from(filename), true),
        Err(VarError::NotPresent) => {
            // If the environment variable is not there, we'll look in the default location:
            // ~/.passwords.rooster
            let mut file_default = PathBuf::from(dirs::home_dir()
                .ok_or(1)?
                .as_os_str()
                .to_os_string()
                .into_string()
                .map_err(|_| 1)?);
            file_default.push(ROOSTER_FILE_DEFAULT);
            (file_default, false)
        }
        Err(VarError::NotUnicode(_)) => return Err(1),
    };
    Ok((path, from_env))
}

fn ask_master_password() -> IoResult<SafeString> {
    prompt_password_stderr("Type your master password: ").map(SafeString::new)
}

fn usage() {
    println!("Welcome to Rooster, the simple password manager for geeks :-)");
    println!();
    println!("Usage:");
    println!("    rooster -h");
    println!("    rooster [options] <command> [<args> ...]");
    println!("    rooster <command> -h");
    println!();
    println!("Options:");
    println!("    -h, --help                 Display a help message");
    println!("    -V, --version              Display the version of Rooster you are using");
    println!();
    println!("Commands:");
    println!("    init                       Create a new password file");
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
    println!("    weak                       List apps that have weak passwords");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Display a help message");
    opts.optflag(
        "V",
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


    // Global help was requested.
    if matches.opt_present("help") && matches.free.is_empty() {
        usage();
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
            usage();
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

    if let Some(cb) = command.callback_without_store {
        if let Err(i) = (cb)(&matches) {
            std::process::exit(i);
        }
    }

    if command.callback_exec.is_some() {
        // Fetch the Rooster file path now, so we can display it in help messages.
        let (password_file_path, password_file_path_from_env) = match get_password_file_path() {
            Ok(path) => path,
            Err(_) => {
                println_err!(
                    "Woops, I could not read the path to your password file. \
                    Make sure it only contains ASCII characters."
                );
                std::process::exit(1);
            }
        };
        let password_file_path_as_string = password_file_path.to_string_lossy().into_owned();

        if !password_file_path.exists() {
            if password_file_path_from_env {
                println_err!("Woops, I couldn't find your password file at:");
                println_err!("    {}", password_file_path_as_string);
                println_err!("");
                println_err!(
                    "Update or remove the ROOSTER_FILE environment variable \
                and try again."
                );
            } else {
                println_title!("|---------- First time user  ---------|");
                println!();
                println!("First time running Rooster? Try `rooster init`.");
                println!();
                println_title!("|----------- Long time user ----------|");
                println!();
                println!(
                    "You already have a Rooster file? Set the ROOSTER_FILE environment variable. \
                    For instance:"
                );
                println!("    export ROOSTER_FILE=path/to/passwords.rooster");
            }
            std::process::exit(1);
        }

        let mut file = match open_password_file(password_file_path_as_string.deref()) {
            Ok(file) => file,
            Err(err) => {
                match err.kind() {
                    std::io::ErrorKind::NotFound => {
                        println_err!(
                            "Woops, I can't find your password file\
                            . Run `rooster init` to create one."
                        );
                    }
                    _ => {
                        println_err!(
                            "Woops, I couldn't read your password file ({} for \"{}\").",
                            err,
                            password_file_path_as_string
                        );
                    }
                }
                std::process::exit(1);
            }
        };

        let mut store = match get_password_store(&mut file) {
            Err(i) => std::process::exit(i),
            Ok(store) => store,
        };

        match execute_command_from_filename(&matches, command, &mut file, &mut store) {
            Err(i) => std::process::exit(i),
            _ => std::process::exit(0),
        }
    }
}
