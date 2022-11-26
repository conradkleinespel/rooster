use crate::ffi;
use crate::password;
use crate::password::v2::{Password, PasswordStore};
use rclio::{CliInputOutput, OutputType};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct JsonExport {
    passwords: Vec<Password>,
}

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    let subcommand_name = matches.subcommand_name().unwrap();
    let subcommand_matches = matches.subcommand_matches(subcommand_name).unwrap();

    let (valid, invalid) = if subcommand_name == "json" {
        create_imported_passwords_from_json(subcommand_matches, io)
    } else if subcommand_name == "csv" {
        create_imported_passwords_from_csv(subcommand_matches, io)
    } else if subcommand_name == "1password" {
        create_imported_passwords_from_1password(subcommand_matches, io)
    } else {
        unimplemented!("Invalid import source")
    }?;

    import_passwords(valid, invalid, store, io)
}

fn import_passwords(
    valid: Vec<Password>,
    invalid: Vec<Password>,
    store: &mut PasswordStore,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    let mut errors = 0;
    let mut warnings = 0;
    let mut successes = 0;
    for password in invalid {
        io.error(
            format!("{}, invalid format, skipping", password.name),
            OutputType::Error,
        );
        errors += 1;
    }
    for password in valid {
        if let Some(_) = store.get_password(&password.name) {
            io.warning(
                format!("{}, already in password store, skipping", password.name),
                OutputType::Error,
            );
            warnings += 1;
            continue;
        }

        if let Err(err) = store.add_password(password.clone()) {
            io.error(
                format!("{}, error ({:?})", password.name, err),
                OutputType::Error,
            );
            errors += 1;
            continue;
        }

        successes += 1;
    }

    io.success(format!("Imported: {}", successes), OutputType::Standard);
    io.warning(format!("Warnings: {}", warnings), OutputType::Error);
    io.error(format!("Errors: {}", errors), OutputType::Error);

    Ok(())
}

fn create_imported_passwords_from_csv(
    matches: &clap::ArgMatches,
    io: &mut impl CliInputOutput,
) -> Result<(Vec<Password>, Vec<Password>), i32> {
    let path_str = matches.value_of("path").unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path_str)
        .map_err(|err| {
            io.error(
                format!("Uh oh, could not open or read the file (reason: {})", err),
                OutputType::Error,
            );
            1
        })?;
    let mut valid = vec![];
    for record_result in reader.records() {
        if let Ok(record) = record_result {
            valid.push(Password {
                name: record[0].into(),
                username: record[1].into(),
                password: record[2].into(),
                created_at: ffi::time(),
                updated_at: ffi::time(),
            });
        } else {
            return Err(1);
        }
    }
    return Ok((valid, vec![]));
}

fn create_imported_passwords_from_1password(
    matches: &clap::ArgMatches,
    io: &mut impl CliInputOutput,
) -> Result<(Vec<Password>, Vec<Password>), i32> {
    let path_str = matches.value_of("path").unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path_str)
        .map_err(|err| {
            io.error(
                format!("Uh oh, could not open or read the file (reason: {})", err),
                OutputType::Error,
            );
            1
        })?;
    let mut valid = vec![];
    let mut invalid = vec![];
    for record_result in reader.records() {
        if let Ok(record) = record_result {
            if &record[3] != "Login" {
                invalid.push(Password {
                    name: record[2].into(),
                    username: record[5].into(),
                    password: record[1].into(),
                    created_at: ffi::time(),
                    updated_at: ffi::time(),
                });
                continue;
            }

            // Fields are, in order: 0/Notes, 1/Password, 2/Title, 3/Type (we can only import "Login"), 4/URL, 5/Username
            valid.push(Password {
                name: record[2].into(),
                username: record[5].into(),
                password: record[1].into(),
                created_at: ffi::time(),
                updated_at: ffi::time(),
            });
        } else {
            return Err(1);
        }
    }
    return Ok((valid, invalid));
}

fn create_imported_passwords_from_json(
    matches: &clap::ArgMatches,
    io: &mut impl CliInputOutput,
) -> Result<(Vec<Password>, Vec<Password>), i32> {
    let path_str = matches.value_of("path").unwrap();
    let dump_file = File::open(path_str).map_err(|err| {
        io.error(
            format!("Uh oh, could not open the file (reason: {})", err),
            OutputType::Error,
        );
        1
    })?;
    let export: JsonExport = serde_json::from_reader(dump_file).map_err(|json_err| {
        io.error(
            format!(
                "Woops, I could not import the passwords from JSON (reason: {}).",
                json_err
            ),
            OutputType::Error,
        );
        1
    })?;
    Ok((export.passwords, vec![]))
}
