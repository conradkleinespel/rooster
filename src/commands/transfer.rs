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
use list;
use macros::{show_error, show_ok};
use password;

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    let query = matches.value_of("app").unwrap();
    let new_username = matches.value_of("new_username").unwrap().to_owned();

    let password = list::search_and_choose_password(
        store,
        query,
        list::WITH_NUMBERS,
        "Which password would you like to transfer?",
    )
    .ok_or(1)?
    .clone();

    let old_username = password.username;

    let change_result =
        store.change_password(&password.name, &|old_password: password::v2::Password| {
            password::v2::Password {
                name: old_password.name.clone(),
                username: new_username.clone(),
                password: old_password.password.clone(),
                created_at: old_password.created_at,
                updated_at: ffi::time(),
            }
        });

    match change_result {
        Ok(_) => {
            show_ok(format!("Done! I've transfered {} to {}", old_username, new_username).as_str());
            Ok(())
        }
        Err(err) => {
            show_error(
                format!(
                    "Woops, I couldn't save the new app name (reason: {:?}).",
                    err
                )
                .as_str(),
            );
            Err(1)
        }
    }
}
