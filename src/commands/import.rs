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

use getopts;
use serde_json;
use password::v2::{Password, PasswordStore};
use std::fs::File;
use macros::{show_error, show_ok};

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster import -h");
    println!("    rooster import <file_path>");
    println!();
    println!("Example:");
    println!("    rooster import dump.json");
}

pub fn check_args(matches: &getopts::Matches) -> Result<(), i32> {
    if matches.free.len() < 2 {
        show_error("Woops, seems like the file path is missing here. For help, try:");
        show_error("    rooster import -h");
        return Err(1);
    }

    Ok(())
}

pub fn callback_exec(matches: &getopts::Matches, store: &mut PasswordStore) -> Result<(), i32> {
    check_args(matches)?;

    let imported_pwds: Vec<Password> = {
        let path_str = &matches.free[1];
        let dump_file = File::open(path_str).map_err(|err| {
            show_error(format!("Uh oh, could not open the file (reason: {})", err).as_str());
            1
        })?;
        serde_json::from_reader(&dump_file).map_err(|json_err| {
            show_error(format!(
                "Woops, I could not import the passwords from JSON (reason: {}).",
                json_err).as_str()
            );
            1
        })?
    };

    let mut added = 0;
    for password in imported_pwds {
        if let Some(_) = store.get_password(&password.name) {
            show_error(format!(
                "Oh, password for {} is already present! Skipping it.",
                password.name).as_str()
            );
            continue;
        }

        if let Err(err) = store.add_password(password.clone()) {
            show_error(format!(
                "Woops, couldn't add password for {} (reason: {:?})",
                password.name,
                err).as_str()
            );
            continue;
        }

        added += 1;
    }

    if added == 0 {
        show_error("Apparently, I could not find any new password :(");
    } else if added == 1 {
        show_ok(format!(
            "Imported {} brand new password into the Rooster file!",
            added).as_str()
        );
    } else {
        show_ok(format!(
            "Imported {} brand new passwords into the Rooster file!",
            added).as_str()
        );
    }

    Ok(())
}
