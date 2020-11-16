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

use csv::Writer;
use macros::show_error;
use password;
use password::v2::Password;
use safe_string::SafeString;
use serde_json;
use std::io::stdout;
use std::ops::Deref;

#[derive(Serialize, Deserialize)]
pub struct JsonExport {
    passwords: Vec<Password>,
}

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    let subcommand_name = matches.subcommand_name().unwrap();
    let subcommand_matches = matches.subcommand_matches(subcommand_name).unwrap();

    if subcommand_name == "json" {
        export_to_json(subcommand_matches, store)
    } else if subcommand_name == "csv" {
        export_to_csv(subcommand_matches, store)
    } else if subcommand_name == "1password" {
        export_to_csv(subcommand_matches, store)
    } else {
        unimplemented!("Invalid export destination")
    }
}

fn export_to_csv(
    _matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    let passwords_ref = store.get_all_passwords();
    let mut csv_writer = Writer::from_writer(stdout());
    for password in passwords_ref {
        match csv_writer.write_record(&[
            &password.name,
            &password.username,
            password.password.inner.as_str(),
        ]) {
            Ok(_) => {}
            Err(_) => return Err(1),
        }
    }
    return Ok(());
}

fn export_to_json(
    _matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    let export = JsonExport {
        passwords: store
            .get_all_passwords()
            .into_iter()
            .map(|password| password.clone())
            .collect(),
    };
    let passwords_json = match serde_json::to_string(&export) {
        Ok(passwords_json) => passwords_json,
        Err(json_err) => {
            show_error(
                format!(
                    "Woops, I could not encode the passwords into JSON (reason: {:?}).",
                    json_err
                )
                .as_str(),
            );
            return Err(1);
        }
    };

    let passwords = SafeString::new(passwords_json);
    // We exceptionally print to STDOUT because the export will most likely be redirected to a file
    println!("{}", passwords.deref());
    return Ok(());
}
