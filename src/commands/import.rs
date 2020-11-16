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
use macros::{show_error, show_ok, show_warning};
use password::v2::{Password, PasswordStore};
use serde_json;
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct JsonExport {
    passwords: Vec<Password>,
}

pub fn callback_exec(matches: &clap::ArgMatches, store: &mut PasswordStore) -> Result<(), i32> {
    let subcommand_name = matches.subcommand_name().unwrap();
    let subcommand_matches = matches.subcommand_matches(subcommand_name).unwrap();

    let (valid, invalid) = if subcommand_name == "json" {
        create_imported_passwords_from_json(subcommand_matches)
    } else if subcommand_name == "1password" {
        create_imported_passwords_from_1password(subcommand_matches)
    } else {
        unimplemented!("Invalid import source")
    }?;

    import_passwords(valid, invalid, store)
}

fn import_passwords(
    valid: Vec<Password>,
    invalid: Vec<Password>,
    store: &mut PasswordStore,
) -> Result<(), i32> {
    let mut errors = 0;
    let mut warnings = 0;
    let mut successes = 0;
    for password in invalid {
        show_error(format!("{}, invalid format, skipping", password.name).as_str());
        errors += 1;
    }
    for password in valid {
        if let Some(_) = store.get_password(&password.name) {
            show_warning(
                format!("{}, already in password store, skipping", password.name).as_str(),
            );
            warnings += 1;
            continue;
        }

        if let Err(err) = store.add_password(password.clone()) {
            show_error(format!("{}, error ({:?})", password.name, err).as_str());
            errors += 1;
            continue;
        }

        successes += 1;
    }

    show_ok(format!("Imported: {}", successes).as_str());
    show_warning(format!("Warnings: {}", warnings).as_str());
    show_error(format!("Errors: {}", errors).as_str());

    Ok(())
}

fn create_imported_passwords_from_1password(
    matches: &clap::ArgMatches,
) -> Result<(Vec<Password>, Vec<Password>), i32> {
    let path_str = matches.value_of("path").unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path_str)
        .map_err(|err| {
            show_error(
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

fn create_imported_passwords_from_json(
    matches: &clap::ArgMatches,
) -> Result<(Vec<Password>, Vec<Password>), i32> {
    let path_str = matches.value_of("path").unwrap();
    let dump_file = File::open(path_str).map_err(|err| {
        show_error(format!("Uh oh, could not open the file (reason: {})", err).as_str());
        1
    })?;
    let export: JsonExport = serde_json::from_reader(dump_file).map_err(|json_err| {
        show_error(
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
