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

extern crate ansi_term;
extern crate byteorder;
extern crate clap;
extern crate clipboard;
extern crate csv;
extern crate dirs;
extern crate libc;
extern crate openssl;
extern crate rand;
extern crate rpassword;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use clap::{App, AppSettings, Arg, ArgMatches};
use macros::{show_error, show_title_1};
use rpassword::prompt_password_stderr;
use safe_string::SafeString;
use safe_vec::SafeVec;
use std::env;
use std::env::VarError;
use std::fs::File;
use std::io::Result as IoResult;
use std::io::{stdin, Read};
use std::ops::Deref;
use std::path::{Path, PathBuf};

mod aes;
mod clip;
mod commands;
mod ffi;
mod generate;
mod list;
mod macros;
mod password;
mod quale;
mod safe_string;
mod safe_vec;

// We conditionally compile this module to avoid "unused function" warnings.
#[cfg(all(unix, not(target_os = "macos")))]
mod shell_escape;

const ROOSTER_FILE_ENV_VAR: &'static str = "ROOSTER_FILE";
const ROOSTER_FILE_DEFAULT: &'static str = ".passwords.rooster";

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
        show_error(
            "Decryption of your Rooster file keeps failing. \
             Your Rooster file is probably corrupted.",
        );
        return Err(password::PasswordError::CorruptionLikelyError);
    }

    if retry {
        show_error("Woops, that's not the right password. Let's try again.");
    }

    let master_password = match ask_master_password() {
        Ok(p) => p,
        Err(err) => {
            show_error(
                format!(
                    "Woops, I could not read your master password (reason: {}).",
                    err
                )
                .as_str(),
            );
            std::process::exit(1);
        }
    };

    match get_password_store_from_input(&input, &master_password, force_upgrade) {
        Ok(store) => {
            return Ok(store);
        }
        Err(password::PasswordError::CorruptionError) => {
            show_error("Your Rooster file is corrupted.");
            return Err(password::PasswordError::CorruptionError);
        }
        Err(password::PasswordError::OutdatedRoosterBinaryError) => {
            show_error(
                "I could not open the Rooster file because your version of Rooster is outdated.",
            );
            show_error("Try upgrading Rooster to the latest version.");
            return Err(password::PasswordError::OutdatedRoosterBinaryError);
        }
        Err(password::PasswordError::Io(err)) => {
            show_error(format!("I couldn't open your Rooster file (reason: {:?})", err).as_str());
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
                                &input, retries, true, false,
                            );
                        } else if line.starts_with('n') {
                            // The user doesn't want to upgrade, that's fine
                            return Err(password::PasswordError::NoUpgradeError);
                        } else {
                            println!("I did not get that. Upgrade from v1 to v2? [y/n]");
                        }
                    }
                    Err(io_err) => {
                        show_error(
                            format!(
                            "Woops, an error occured while reading your response (reason: {:?}).",
                            io_err)
                            .as_str(),
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
    matches: &ArgMatches,
    command: fn(&ArgMatches, &mut password::v2::PasswordStore) -> Result<(), i32>,
    file: &mut File,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    (command)(matches, store)?;

    match store.sync(file) {
        Ok(()) => Ok(()),
        Err(err) => {
            show_error(format!("I could not save the password file (reason: {:?}).", err).as_str());
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
            let mut file_default = PathBuf::from(
                dirs::home_dir()
                    .ok_or(1)?
                    .as_os_str()
                    .to_os_string()
                    .into_string()
                    .map_err(|_| 1)?,
            );
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

fn main() {
    let matches: ArgMatches = App::new("rooster")
        // .global_setting(AppSettings::HelpRequired)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Welcome to Rooster, the simple password manager for geeks :-)")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(App::new("init").about("Create a new password file"))
        .subcommand(
            App::new("add")
                .about("Add a new password manually")
                .arg(Arg::new("app"))
                .arg(Arg::new("username"))
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .about("Show the password instead of copying it to the clipboard"),
                ),
        )
        .subcommand(
            App::new("change")
                .about("Change a password manually")
                .arg(Arg::new("app"))
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .about("Show the password instead of copying it to the clipboard"),
                ),
        )
        .subcommand(
            App::new("delete")
                .about("Delete a password")
                .arg(Arg::new("app")),
        )
        .subcommand(
            App::new("generate")
                .about("Generate a password")
                .arg(Arg::new("app"))
                .arg(Arg::new("username"))
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .about("Show the password instead of copying it to the clipboard"),
                )
                .arg(
                    Arg::new("alnum")
                        .short('a')
                        .long("alnum")
                        .about("Only use alpha numeric (a-z, A-Z, 0-9) in generated passwords"),
                )
                .arg(
                    Arg::new("length")
                        .short('l')
                        .long("length")
                        .default_value("32")
                        .about("Set a custom length for the generated password"),
                ),
        )
        .subcommand(
            App::new("regenerate")
                .about("Regenerate a previously existing password")
                .arg(Arg::new("app"))
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .about("Show the password instead of copying it to the clipboard"),
                )
                .arg(
                    Arg::new("alnum")
                        .short('a')
                        .long("alnum")
                        .about("Only use alpha numeric (a-z, A-Z, 0-9) in generated passwords"),
                )
                .arg(
                    Arg::new("length")
                        .short('l')
                        .long("length")
                        .default_value("32")
                        .about("Set a custom length for the generated password"),
                ),
        )
        .subcommand(
            App::new("get")
                .about("Retrieve a password")
                .arg(Arg::new("app"))
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .about("Show the password instead of copying it to the clipboard"),
                ),
        )
        .subcommand(
            App::new("rename")
                .about("Rename the app for a password")
                .arg(Arg::new("app"))
                .arg(Arg::new("new_name")),
        )
        .subcommand(
            App::new("transfer")
                .about("Change the username for a password")
                .arg(Arg::new("app"))
                .arg(Arg::new("new_username")),
        )
        .subcommand(App::new("list").about("List all apps and usernames"))
        .subcommand(App::new("import").about("Load all your raw password data from JSON file"))
        .subcommand(
            App::new("export")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Export raw password data")
                .subcommand(
                    App::new("rooster").about("Export raw password data in Rooster's JSON format"),
                )
                .subcommand(
                    App::new("1password")
                        .about("Export raw password data in 1Password compatible CSV format"),
                ),
        )
        .subcommand(App::new("set-master-password").about("Set your master password"))
        .subcommand(
            App::new("set-scrypt-params")
                .about("Set the key derivation parameters")
                .arg(Arg::new("log2n"))
                .arg(Arg::new("r"))
                .arg(Arg::new("p")),
        )
        .get_matches();

    let subcommand = matches.subcommand_name().unwrap();

    let command_matches = matches.subcommand_matches(subcommand).unwrap();

    if subcommand == "init" {
        match commands::init::callback_exec(command_matches) {
            Err(i) => std::process::exit(i),
            _ => std::process::exit(0),
        }
    }

    let command_callback = match subcommand {
        "get" => commands::get::callback_exec,
        "add" => commands::add::callback_exec,
        "delete" => commands::delete::callback_exec,
        "generate" => commands::generate::callback_exec,
        "regenerate" => commands::regenerate::callback_exec,
        "list" => commands::list::callback_exec,
        "import" => commands::import::callback_exec,
        "export" => commands::export::callback_exec,
        "set-master-password" => commands::set_master_password::callback_exec,
        "set-scrypt-params" => commands::set_scrypt_params::callback_exec,
        "rename" => commands::rename::callback_exec,
        "transfer" => commands::transfer::callback_exec,
        "change" => commands::change::callback_exec,
        _ => unreachable!("Validation should have been done by `clap` before"),
    };

    // Fetch the Rooster file path now, so we can display it in help messages.
    let (password_file_path, password_file_path_from_env) = match get_password_file_path() {
        Ok(info) => info,
        Err(_) => {
            show_error(
                "Woops, I could not read the path to your password file. \
             Make sure it only contains ASCII characters.",
            );
            std::process::exit(1);
        }
    };

    let password_file_path_as_string = password_file_path.to_string_lossy().into_owned();

    if !password_file_path.exists() {
        if password_file_path_from_env {
            show_error("Woops, I couldn't find your password file at:");
            show_error(format!("    {}", password_file_path_as_string).as_str());
            show_error("");
            show_error(
                "Update or remove the ROOSTER_FILE environment variable \
             and try again. Or run `rooster init` to start fresh.",
            );
        } else {
            show_title_1("First time user");
            println!();
            println!("Try `rooster init`.");
            println!();
            show_title_1("Long time user");
            println!();
            println!(
                "Set the ROOSTER_FILE environment variable. \
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
                    show_error(
                        "Woops, I can't find your password file\
                     . Run `rooster init` to create one.",
                    );
                }
                _ => {
                    show_error(
                        format!(
                            "Woops, I couldn't read your password file ({} for \"{}\").",
                            err, password_file_path_as_string
                        )
                        .as_str(),
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

    match execute_command_from_filename(command_matches, command_callback, &mut file, &mut store) {
        Err(i) => std::process::exit(i),
        _ => std::process::exit(0),
    }
}
