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

use macros::show_error;
use password;
use safe_string::SafeString;
use serde_json;
use std::ops::Deref;

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    let passwords_ref = store.get_all_passwords();

    if matches.is_present("1password") {
        println!("Exporting for 1password");
        Ok(())
    } else {
        let passwords_json = match serde_json::to_string(&passwords_ref) {
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
        Ok(())
    }
}
