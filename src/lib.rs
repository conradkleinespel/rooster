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
use io::{ReaderManager, WriterManager};
use password::v2::PasswordStore;
use safe_string::SafeString;
use safe_vec::SafeVec;
use std::env;
use std::fs::File;
use std::io::{BufRead, Result as IoResult};
use std::io::{Read, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};

mod aes;
mod clip;
mod commands;
mod ffi;
mod generate;
pub mod io;
mod list;
mod password;
mod quale;
mod safe_string;
mod safe_vec;

// We conditionally compile this module to avoid "unused function" warnings.
#[cfg(all(unix, not(target_os = "macos")))]
mod shell_escape;

fn validate_arg_digits(v: &str) -> Result<(), String> {
    if v.chars()
        .map(|c| char::is_ascii_digit(&c))
        .collect::<Vec<bool>>()
        .contains(&false)
    {
        return Err(String::from("The value must be made of digits"));
    }
    Ok(())
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

fn sync_password_store<
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    store: &mut PasswordStore,
    file: &mut File,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> Result<(), i32> {
    if let Err(err) = store.sync(file) {
        writer
            .error()
            .error(format!("I could not save the password file (reason: {:?}).", err).as_str());
        return Err(1);
    }

    return Ok(());
}

fn get_password_store<
    R: BufRead,
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    file: &mut File,
    reader: &mut ReaderManager<R>,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> Result<password::v2::PasswordStore, i32> {
    // Read the Rooster file contents.
    let mut input: SafeVec = SafeVec::new(Vec::new());
    file.read_to_end(input.inner_mut()).map_err(|_| 1)?;

    return get_password_store_from_input_interactive(&input, 3, false, false, reader, writer)
        .map_err(|_| 1);
}

fn get_password_store_from_input_interactive<
    R: BufRead,
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    input: &SafeVec,
    retries: i32,
    force_upgrade: bool,
    retry: bool,
    reader: &mut ReaderManager<R>,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> Result<password::v2::PasswordStore, password::PasswordError> {
    if retries == 0 {
        writer.error().error(
            "Decryption of your Rooster file keeps failing. \
             Your Rooster file is probably corrupted.",
        );
        return Err(password::PasswordError::CorruptionLikelyError);
    }

    if retry {
        writer
            .error()
            .error("Woops, that's not the right password. Let's try again.");
    }

    let master_password = match ask_master_password(reader, writer) {
        Ok(p) => p,
        Err(err) => {
            writer.error().error(
                format!(
                    "Woops, I could not read your master password (reason: {}).",
                    err
                )
                .as_str(),
            );
            return Err(password::PasswordError::Io(err));
        }
    };

    match get_password_store_from_input(&input, &master_password, force_upgrade) {
        Ok(store) => {
            return Ok(store);
        }
        Err(password::PasswordError::CorruptionError) => {
            writer.error().error("Your Rooster file is corrupted.");
            return Err(password::PasswordError::CorruptionError);
        }
        Err(password::PasswordError::OutdatedRoosterBinaryError) => {
            writer.error().error(
                "I could not open the Rooster file because your version of Rooster is outdated.",
            );
            writer
                .error()
                .error("Try upgrading Rooster to the latest version.");
            return Err(password::PasswordError::OutdatedRoosterBinaryError);
        }
        Err(password::PasswordError::Io(err)) => {
            writer
                .error()
                .error(format!("I couldn't open your Rooster file (reason: {:?})", err).as_str());
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
                match reader.read_line() {
                    Ok(line) => {
                        if line.starts_with('y') {
                            // This time we'll try to upgrade
                            return get_password_store_from_input_interactive(
                                &input, retries, true, false, reader, writer,
                            );
                        } else if line.starts_with('n') {
                            // The user doesn't want to upgrade, that's fine
                            return Err(password::PasswordError::NoUpgradeError);
                        } else {
                            println!("I did not get that. Upgrade from v1 to v2? [y/n]");
                        }
                    }
                    Err(io_err) => {
                        writer.error().error(
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
            return get_password_store_from_input_interactive(
                &input,
                retries - 1,
                false,
                true,
                reader,
                writer,
            );
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

fn ask_master_password<
    R: BufRead,
    ErrorWriter: ?Sized + Write,
    OutputWriter: ?Sized + Write,
    InstructionWriter: ?Sized + Write,
>(
    reader: &mut ReaderManager<R>,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> IoResult<SafeString> {
    writer.instruction().prompt("Type your master password: ");
    reader.read_password()
}

pub fn main_with_args<
    R: BufRead,
    ErrorWriter: ?Sized + Write,
    OutputWriter: ?Sized + Write,
    InstructionWriter: ?Sized + Write,
>(
    args: &[&str],
    reader: &mut ReaderManager<R>,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
    rooster_file_path: &PathBuf,
) -> i32 {
    let matches: ArgMatches = App::new("rooster")
        .global_setting(AppSettings::HelpRequired)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Welcome to Rooster, the simple password manager for geeks :-)")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            App::new("init").about("Create a new password file").arg(
                Arg::new("force-for-tests")
                    .long("force-for-tests")
                    .hidden(true)
                    .about("Forces initializing the file, used in integration tests only"),
            ),
        )
        .subcommand(
            App::new("add")
                .about("Add a new password manually")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .about("The name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("username")
                        .required(true)
                        .about("Your username for this account"),
                )
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
                .arg(
                    Arg::new("app")
                        .required(true)
                        .about("The name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .about("Show the password instead of copying it to the clipboard"),
                ),
        )
        .subcommand(
            App::new("delete").about("Delete a password").arg(
                Arg::new("app")
                    .required(true)
                    .about("The name of the app (fuzzy-matched)"),
            ),
        )
        .subcommand(
            App::new("generate")
                .about("Generate a password")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .about("The name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("username")
                        .required(true)
                        .about("Your username for this account"),
                )
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
                        .about("Set a custom length for the generated password")
                        .validator(validate_arg_digits),
                ),
        )
        .subcommand(
            App::new("regenerate")
                .about("Regenerate a previously existing password")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .about("The name of the app (fuzzy-matched)"),
                )
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
                        .about("Set a custom length for the generated password")
                        .validator(validate_arg_digits),
                ),
        )
        .subcommand(
            App::new("get")
                .about("Retrieve a password")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .about("The name of the app (fuzzy-matched)"),
                )
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
                .arg(
                    Arg::new("app")
                        .required(true)
                        .about("The current name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("new_name")
                        .required(true)
                        .about("The new name of the app"),
                ),
        )
        .subcommand(
            App::new("transfer")
                .about("Change the username for a password")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .about("The current name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("new_username")
                        .required(true)
                        .about("Your new username for this account"),
                ),
        )
        .subcommand(App::new("list").about("List all apps and usernames"))
        .subcommand(
            App::new("import")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Import all your existing passwords from elsewhere")
                .subcommand(
                    App::new("json")
                        .about("Import a file generated with `rooster export json`")
                        .arg(
                            Arg::new("path")
                                .required(true)
                                .about("The path to the file you want to import"),
                        ),
                )
                .subcommand(
                    App::new("csv")
                        .about("Import a file generated with `rooster export csv`")
                        .arg(
                            Arg::new("path")
                                .required(true)
                                .about("The path to the file you want to import"),
                        ),
                )
                .subcommand(
                    App::new("1password")
                        .about("Import a \"Common Fields\" CSV export from 1Password")
                        .arg(
                            Arg::new("path")
                                .required(true)
                                .about("The path to the file you want to import"),
                        ),
                ),
        )
        .subcommand(
            App::new("export")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Export raw password data")
                .subcommand(App::new("json").about("Export raw password data in JSON format"))
                .subcommand(App::new("csv").about("Export raw password data in CSV format"))
                .subcommand(
                    App::new("1password")
                        .about("Export raw password data in 1Password compatible CSV format"),
                ),
        )
        .subcommand(App::new("set-master-password").about("Set your master password"))
        .subcommand(
            App::new("set-scrypt-params")
                .about("Set the key derivation parameters")
                .arg(
                    Arg::new("log2n")
                        .required(true)
                        .about("The log2n parameter")
                        .validator(validate_arg_digits),
                )
                .arg(
                    Arg::new("r")
                        .required(true)
                        .about("The r parameter")
                        .validator(validate_arg_digits),
                )
                .arg(
                    Arg::new("p")
                        .required(true)
                        .about("The p parameter")
                        .validator(validate_arg_digits),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .about("Disable parameter checks"),
                ),
        )
        .get_matches_from(args);

    let subcommand = matches.subcommand_name().unwrap();

    let command_matches = matches.subcommand_matches(subcommand).unwrap();

    if subcommand == "init" {
        match commands::init::callback_exec(command_matches, reader, writer, rooster_file_path) {
            Err(i) => return i,
            _ => return 0,
        }
    }

    let password_file_path_as_string = rooster_file_path.to_string_lossy().into_owned();

    if !rooster_file_path.exists() {
        writer.output().title("First time user");
        writer.output().newline();
        writer.output().info("Try `rooster init`.");
        writer.output().newline();
        writer.output().title("Long time user");
        writer.output().newline();
        writer
            .output()
            .info("Set the ROOSTER_FILE environment variable. For instance:");
        writer
            .output()
            .info("    export ROOSTER_FILE=path/to/passwords.rooster");
        return 1;
    }

    let mut file = match open_password_file(password_file_path_as_string.deref()) {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => {
                    writer.error().error(
                        "Woops, I can't find your password file\
                     . Run `rooster init` to create one.",
                    );
                }
                _ => {
                    writer.error().error(
                        format!(
                            "Woops, I couldn't read your password file ({} for \"{}\").",
                            err, password_file_path_as_string
                        )
                        .as_str(),
                    );
                }
            }
            return 1;
        }
    };

    let mut store = match get_password_store(&mut file, reader, writer) {
        Err(code) => return code,
        Ok(store) => store,
    };

    let callback = match subcommand {
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

    if let Err(code) = callback(command_matches, &mut store, reader, writer) {
        return code;
    }

    if let Err(code) = sync_password_store(&mut store, &mut file, writer) {
        return code;
    }

    return 0;
}
