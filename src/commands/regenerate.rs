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

use clip;
use ffi;
use generate::{check_password_len, PasswordSpec};
use list;
use macros::show_error;
use password;

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    let query = matches.value_of("app").unwrap();

    let password = list::search_and_choose_password(
        store,
        query,
        list::WITH_NUMBERS,
        "Which password would you like to regenerate?",
    )
    .ok_or(1)?
    .clone();

    let pwspec = PasswordSpec::new(
        matches.is_present("alnum"),
        matches
            .value_of("length")
            .and_then(|len| check_password_len(len.parse::<usize>().ok())),
    );

    let password_as_string = match pwspec.generate_hard_password() {
        Ok(password_as_string) => password_as_string,
        Err(io_err) => {
            show_error(
                format!(
                    "Woops, I could not generate the password (reason: {:?}).",
                    io_err
                )
                .as_str(),
            );
            return Err(1);
        }
    };

    let change_result =
        store.change_password(&password.name, &|old_password: password::v2::Password| {
            password::v2::Password {
                name: old_password.name.clone(),
                username: old_password.username.clone(),
                password: password_as_string.clone(),
                created_at: old_password.created_at,
                updated_at: ffi::time(),
            }
        });

    match change_result {
        Ok(password) => {
            let show = matches.is_present("show");
            clip::confirm_password_retrieved(show, &password);
            Ok(())
        }
        Err(err) => {
            show_error(
                format!(
                    "Woops, I couldn't save the new password (reason: {:?}).",
                    err
                )
                .as_str(),
            );
            Err(1)
        }
    }
}
