// #![allow(useless_format, too_many_arguments)]

use crate::password::v2::PasswordStore;
use crate::rclio::CliInputOutput;
use crate::rclio::OutputType;
use crate::rutil::safe_string::SafeString;
use crate::rutil::safe_vec::SafeVec;
use clap::{App, AppSettings, Arg};
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Result as IoResult;
use std::ops::Deref;
use std::path::{Path, PathBuf};

mod aes;
mod clip;
mod commands;
mod ffi;
mod generate;
mod list;
mod password;
mod quale;
#[allow(unused)]
pub mod rclio;
#[allow(unused)]
mod rpassword;
#[allow(unused)]
mod rprompt;
#[allow(unused)]
mod rutil;
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

fn sync_password_store(
    store: &mut PasswordStore,
    file: &mut File,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    if let Err(err) = store.sync(file) {
        io.error(
            format!("I could not save the password file (reason: {:?}).", err),
            OutputType::Error,
        );
        return Err(1);
    }

    return Ok(());
}

fn get_password_store(
    file: &mut File,
    io: &mut impl CliInputOutput,
) -> Result<password::v2::PasswordStore, i32> {
    // Read the Rooster file contents.
    let mut input: SafeVec = SafeVec::new(Vec::new());
    file.read_to_end(input.inner_mut()).map_err(|_| 1)?;

    return get_password_store_from_input_interactive(&input, 3, false, false, io).map_err(|_| 1);
}

fn get_password_store_from_input_interactive(
    input: &SafeVec,
    retries: i32,
    force_upgrade: bool,
    retry: bool,
    io: &mut impl CliInputOutput,
) -> Result<password::v2::PasswordStore, password::PasswordError> {
    if retries == 0 {
        io.error(
            "Decryption of your Rooster file keeps failing. \
             Your Rooster file is probably corrupted.",
            OutputType::Error,
        );
        return Err(password::PasswordError::CorruptionLikelyError);
    }

    if retry {
        io.error(
            "Woops, that's not the right password. Let's try again.",
            OutputType::Error,
        );
    }

    let master_password = match ask_master_password(io) {
        Ok(p) => p,
        Err(err) => {
            io.error(
                format!(
                    "Woops, I could not read your master password (reason: {}).",
                    err
                ),
                OutputType::Error,
            );
            return Err(password::PasswordError::Io(err));
        }
    };

    match get_password_store_from_input(&input, &master_password, force_upgrade) {
        Ok(store) => {
            return Ok(store);
        }
        Err(password::PasswordError::CorruptionError) => {
            io.error("Your Rooster file is corrupted.", OutputType::Error);
            return Err(password::PasswordError::CorruptionError);
        }
        Err(password::PasswordError::OutdatedRoosterBinaryError) => {
            io.error(
                "I could not open the Rooster file because your version of Rooster is outdated.",
                OutputType::Error,
            );
            io.error(
                "Try upgrading Rooster to the latest version.",
                OutputType::Error,
            );
            return Err(password::PasswordError::OutdatedRoosterBinaryError);
        }
        Err(password::PasswordError::Io(err)) => {
            io.error(
                format!("I couldn't open your Rooster file (reason: {:?})", err),
                OutputType::Error,
            );
            return Err(password::PasswordError::Io(err));
        }
        Err(password::PasswordError::NeedUpgradeErrorFromV1) => {
            io.error("Your Rooster file has version 1. You need to upgrade to version 2.\n\nWARNING: If in doubt, it could mean you've been hacked. Only \
                 proceed if you recently upgraded your Rooster installation.\nUpgrade to version 2? [y/n]", OutputType::Error
            );
            loop {
                match io.read_line() {
                    Ok(line) => {
                        if line.starts_with('y') {
                            // This time we'll try to upgrade
                            return get_password_store_from_input_interactive(
                                &input, retries, true, false, io,
                            );
                        } else if line.starts_with('n') {
                            // The user doesn't want to upgrade, that's fine
                            return Err(password::PasswordError::NoUpgradeError);
                        } else {
                            io.error(
                                "I did not get that. Upgrade from v1 to v2? [y/n]",
                                OutputType::Error,
                            );
                        }
                    }
                    Err(io_err) => {
                        io.error(format!(
                                "Woops, an error occured while reading your response (reason: {:?}).",
                                io_err
                            ), OutputType::Error,
                        );
                        return Err(password::PasswordError::Io(io_err));
                    }
                }
            }
        }
        _ => {
            return get_password_store_from_input_interactive(&input, retries - 1, false, true, io);
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

fn ask_master_password(io: &mut impl CliInputOutput) -> IoResult<SafeString> {
    io.prompt_password("Type your master password: ")
}

pub fn main_with_args(
    args: &[&str],
    io: &mut impl CliInputOutput,
    rooster_file_path: &PathBuf,
) -> i32 {
    let matches = App::new("rooster")
        .global_setting(AppSettings::HelpExpected)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .override_help("Welcome to Rooster, the simple password manager for geeks :-)")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            App::new("init").override_help("Create a new password file").arg(
                Arg::new("force-for-tests")
                    .long("force-for-tests")
                    .hide(true)
                    .help("Forces initializing the file, used in integration tests only"),
            ),
        )
        .subcommand(
            App::new("add")
                .override_help("Add a new password manually")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .help("The name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("username")
                        .required(true)
                        .help("Your username for this account"),
                )
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .help("Show the password instead of copying it to the clipboard"),
                ),
        )
        .subcommand(
            App::new("change")
                .override_help("Change a password manually")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .help("The name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .help("Show the password instead of copying it to the clipboard"),
                ),
        )
        .subcommand(
            App::new("delete").override_help("Delete a password").arg(
                Arg::new("app")
                    .required(true)
                    .help("The name of the app (fuzzy-matched)"),
            ),
        )
        .subcommand(
            App::new("generate")
                .override_help("Generate a password")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .help("The name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("username")
                        .required(true)
                        .help("Your username for this account"),
                )
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .help("Show the password instead of copying it to the clipboard"),
                )
                .arg(
                    Arg::new("alnum")
                        .short('a')
                        .long("alnum")
                        .help("Only use alpha numeric (a-z, A-Z, 0-9) in generated passwords"),
                )
                .arg(
                    Arg::new("length")
                        .short('l')
                        .long("length")
                        .default_value("32")
                        .help("Set a custom length for the generated password")
                        .validator(validate_arg_digits),
                ),
        )
        .subcommand(
            App::new("regenerate")
                .override_help("Regenerate a previously existing password")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .help("The name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .help("Show the password instead of copying it to the clipboard"),
                )
                .arg(
                    Arg::new("alnum")
                        .short('a')
                        .long("alnum")
                        .help("Only use alpha numeric (a-z, A-Z, 0-9) in generated passwords"),
                )
                .arg(
                    Arg::new("length")
                        .short('l')
                        .long("length")
                        .default_value("32")
                        .help("Set a custom length for the generated password")
                        .validator(validate_arg_digits),
                ),
        )
        .subcommand(
            App::new("get")
                .override_help("Retrieve a password")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .help("The name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .help("Show the password instead of copying it to the clipboard"),
                ),
        )
        .subcommand(
            App::new("rename")
                .override_help("Rename the app for a password")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .help("The current name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("new_name")
                        .required(true)
                        .help("The new name of the app"),
                ),
        )
        .subcommand(
            App::new("transfer")
                .override_help("Change the username for a password")
                .arg(
                    Arg::new("app")
                        .required(true)
                        .help("The current name of the app (fuzzy-matched)"),
                )
                .arg(
                    Arg::new("new_username")
                        .required(true)
                        .help("Your new username for this account"),
                ),
        )
        .subcommand(App::new("list").override_help("List all apps and usernames"))
        .subcommand(
            App::new("import")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .override_help("Import all your existing passwords from elsewhere")
                .subcommand(
                    App::new("json")
                        .override_help("Import a file generated with `rooster export json`")
                        .arg(
                            Arg::new("path")
                                .required(true)
                                .help("The path to the file you want to import"),
                        ),
                )
                .subcommand(
                    App::new("csv")
                        .override_help("Import a file generated with `rooster export csv`")
                        .arg(
                            Arg::new("path")
                                .required(true)
                                .help("The path to the file you want to import"),
                        ),
                )
                .subcommand(
                    App::new("1password")
                        .override_help("Import a \"Common Fields\" CSV export from 1Password")
                        .arg(
                            Arg::new("path")
                                .required(true)
                                .help("The path to the file you want to import"),
                        ),
                ),
        )
        .subcommand(
            App::new("export")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .override_help("Export raw password data")
                .subcommand(App::new("json").override_help("Export raw password data in JSON format"))
                .subcommand(App::new("csv").override_help("Export raw password data in CSV format"))
                .subcommand(
                    App::new("1password")
                        .override_help("Export raw password data in 1Password compatible CSV format"),
                ),
        )
        .subcommand(App::new("set-master-password").override_help("Set your master password"))
        .subcommand(
            App::new("set-scrypt-params")
                .override_help("Set the key derivation parameters")
                .arg(
                    Arg::new("log2n")
                        .required(true)
                        .help("The log2n parameter")
                        .validator(validate_arg_digits),
                )
                .arg(
                    Arg::new("r")
                        .required(true)
                        .help("The r parameter")
                        .validator(validate_arg_digits),
                )
                .arg(
                    Arg::new("p")
                        .required(true)
                        .help("The p parameter")
                        .validator(validate_arg_digits),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Disable parameter checks"),
                ),
        )
        .get_matches_from(args);

    let subcommand = matches.subcommand_name().unwrap();

    let command_matches = matches.subcommand_matches(subcommand).unwrap();

    if subcommand == "init" {
        match commands::init::callback_exec(command_matches, io, rooster_file_path) {
            Err(i) => return i,
            _ => return 0,
        }
    }

    let password_file_path_as_string = rooster_file_path.to_string_lossy().into_owned();

    if !rooster_file_path.exists() {
        io.title("First time user", OutputType::Standard);
        io.nl(OutputType::Standard);
        io.info("Try `rooster init`.", OutputType::Standard);
        io.nl(OutputType::Standard);
        io.title("Long time user", OutputType::Standard);
        io.nl(OutputType::Standard);
        io.info(
            "Set the ROOSTER_FILE environment variable. For instance:",
            OutputType::Standard,
        );
        io.info(
            "    export ROOSTER_FILE=path/to/passwords.rooster",
            OutputType::Standard,
        );
        return 1;
    }

    let mut file = match open_password_file(password_file_path_as_string.deref()) {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => {
                    io.error(
                        "Woops, I can't find your password file. Run `rooster init` to create one.",
                        OutputType::Error,
                    );
                }
                _ => {
                    io.error(
                        format!(
                            "Woops, I couldn't read your password file ({} for \"{}\").",
                            err, password_file_path_as_string
                        ),
                        OutputType::Error,
                    );
                }
            }
            return 1;
        }
    };

    let mut store = match get_password_store(&mut file, io) {
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

    if let Err(code) = callback(command_matches, &mut store, io) {
        return code;
    }

    if let Err(code) = sync_password_store(&mut store, &mut file, io) {
        return code;
    }

    return 0;
}
