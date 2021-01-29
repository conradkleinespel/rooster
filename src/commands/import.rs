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

use ffi;
use io::{ReaderManager, WriterManager};
use password;
use password::v2::{Password, PasswordStore};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{BufRead, Write};

#[derive(Serialize, Deserialize)]
pub struct JsonExport {
    passwords: Vec<Password>,
}

pub fn callback_exec<
    R: BufRead,
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    reader: &mut ReaderManager<R>,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> Result<(), i32> {
    let subcommand_name = matches.subcommand_name().unwrap();
    let subcommand_matches = matches.subcommand_matches(subcommand_name).unwrap();

    let (valid, invalid) = if subcommand_name == "json" {
        create_imported_passwords_from_json(subcommand_matches, writer)
    } else if subcommand_name == "csv" {
        create_imported_passwords_from_csv(subcommand_matches, writer)
    } else if subcommand_name == "1password" {
        create_imported_passwords_from_1password(subcommand_matches, writer)
    } else {
        unimplemented!("Invalid import source")
    }?;

    import_passwords(valid, invalid, store, writer)
}

fn import_passwords<
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    valid: Vec<Password>,
    invalid: Vec<Password>,
    store: &mut PasswordStore,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> Result<(), i32> {
    let mut errors = 0;
    let mut warnings = 0;
    let mut successes = 0;
    for password in invalid {
        writer
            .error()
            .error(format!("{}, invalid format, skipping", password.name).as_str());
        errors += 1;
    }
    for password in valid {
        if let Some(_) = store.get_password(&password.name) {
            writer.error().warning(
                format!("{}, already in password store, skipping", password.name).as_str(),
            );
            warnings += 1;
            continue;
        }

        if let Err(err) = store.add_password(password.clone()) {
            writer
                .error()
                .error(format!("{}, error ({:?})", password.name, err).as_str());
            errors += 1;
            continue;
        }

        successes += 1;
    }

    writer
        .output()
        .success(format!("Imported: {}", successes).as_str());
    writer
        .error()
        .warning(format!("Warnings: {}", warnings).as_str());
    writer.error().error(format!("Errors: {}", errors).as_str());

    Ok(())
}

fn create_imported_passwords_from_csv<
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    matches: &clap::ArgMatches,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> Result<(Vec<Password>, Vec<Password>), i32> {
    let path_str = matches.value_of("path").unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path_str)
        .map_err(|err| {
            writer.error().error(
                format!("Uh oh, could not open or read the file (reason: {})", err).as_str(),
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

fn create_imported_passwords_from_1password<
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    matches: &clap::ArgMatches,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> Result<(Vec<Password>, Vec<Password>), i32> {
    let path_str = matches.value_of("path").unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path_str)
        .map_err(|err| {
            writer.error().error(
                format!("Uh oh, could not open or read the file (reason: {})", err).as_str(),
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

fn create_imported_passwords_from_json<
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    matches: &clap::ArgMatches,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> Result<(Vec<Password>, Vec<Password>), i32> {
    let path_str = matches.value_of("path").unwrap();
    let dump_file = File::open(path_str).map_err(|err| {
        writer
            .error()
            .error(format!("Uh oh, could not open the file (reason: {})", err).as_str());
        1
    })?;
    let export: JsonExport = serde_json::from_reader(dump_file).map_err(|json_err| {
        writer.error().error(
            format!(
                "Woops, I could not import the passwords from JSON (reason: {}).",
                json_err
            )
            .as_str(),
        );
        1
    })?;
    Ok((export.passwords, vec![]))
}
