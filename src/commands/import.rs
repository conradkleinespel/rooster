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

use csv::StringRecord;
use ffi;
use macros::{show_error, show_ok};
use password::v2::{Password, PasswordStore};
use serde_json;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct JsonExport {
    passwords: Vec<Password>,
}

pub fn callback_exec(matches: &clap::ArgMatches, store: &mut PasswordStore) -> Result<(), i32> {
    let subcommand_name = matches.subcommand_name().unwrap();
    let subcommand_matches = matches.subcommand_matches(subcommand_name).unwrap();

    let passwords = if subcommand_name == "json" {
        create_imported_passwords_from_json(subcommand_matches)
    } else if subcommand_name == "1password" {
        create_imported_passwords_from_1password(subcommand_matches)
    } else {
        unimplemented!("Invalid import source")
    }?;

    import_passwords(passwords, store)
}

fn import_passwords(passwords: Vec<Password>, store: &mut PasswordStore) -> Result<(), i32> {
    let mut added = 0;
    for password in passwords {
        if let Some(_) = store.get_password(&password.name) {
            show_error(
                format!(
                    "Oh, password for {} is already present! Skipping it.",
                    password.name
                )
                .as_str(),
            );
            continue;
        }

        if let Err(err) = store.add_password(password.clone()) {
            show_error(
                format!(
                    "Woops, couldn't add password for {} (reason: {:?})",
                    password.name, err
                )
                .as_str(),
            );
            continue;
        }

        added += 1;
    }

    if added == 0 {
        show_error("Apparently, I could not find any new password :(");
    } else if added == 1 {
        show_ok(
            format!(
                "Imported {} brand new password into the Rooster file!",
                added
            )
            .as_str(),
        );
    } else {
        show_ok(
            format!(
                "Imported {} brand new passwords into the Rooster file!",
                added
            )
            .as_str(),
        );
    }

    Ok(())
}

fn create_imported_passwords_from_1password(
    matches: &clap::ArgMatches,
) -> Result<Vec<Password>, i32> {
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
    let mut passwords = vec![];
    for record_result in reader.records() {
        if let Ok(record) = record_result {
            if &record[3] != "Login" {
                continue;
            }

            // Fields are, in order: 0/Notes, 1/Password, 2/Title, 3/Type (we can only import "Login"), 4/URL, 5/Username
            passwords.push(Password {
                name: record[2].into(),
                username: record[5].into(),
                password: record[1].into(),
                created_at: ffi::time(),
                updated_at: ffi::time(),
            })
        } else {
            return Err(1);
        }
    }
    return Ok(passwords);
}

fn create_imported_passwords_from_json(matches: &clap::ArgMatches) -> Result<Vec<Password>, i32> {
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
    Ok(export.passwords)
}
