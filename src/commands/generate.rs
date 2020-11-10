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

use clip::{copy_to_clipboard, paste_keys};
use generate::{check_password_len, PasswordSpec};
use macros::{show_error, show_ok};
use password;
use std::ops::Deref;

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    let app_name = matches.value_of("app").unwrap();
    let username = matches.value_of("username").unwrap();

    if store.has_password(app_name.deref()) {
        show_error("Woops, there is already an app with that name.");
        return Err(1);
    }

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

    // Read the master password and try to save the new password.
    let password_as_string_clipboard = password_as_string.clone();
    let password = password::v2::Password::new(app_name.clone(), username, password_as_string);

    match store.add_password(password) {
        Ok(_) => {
            if matches.is_present("show") {
                show_ok(
                    format!(
                        "Alright! Here is your password: {}",
                        password_as_string_clipboard.deref()
                    )
                    .as_str(),
                );
                return Ok(());
            }

            if copy_to_clipboard(&password_as_string_clipboard).is_err() {
                show_ok(
                    format!(
                        "Hmm, I tried to copy your new password to your clipboard, but \
                         something went wrong. Don't worry, it's saved, and you can see it \
                         with `rooster get {} --show`",
                        app_name
                    )
                    .as_str(),
                );
            } else {
                show_ok(
                    format!(
                        "Alright! I've saved your new password. You can paste it anywhere \
                         with {}.",
                        paste_keys()
                    )
                    .as_str(),
                );
            }

            Ok(())
        }
        Err(err) => {
            show_error(format!("\nI couldn't add this password (reason: {:?}).", err).as_str());
            Err(1)
        }
    }
}
