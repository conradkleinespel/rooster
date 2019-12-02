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
use macros::{show_error, show_ok};
use password;
use rpassword::prompt_password_stderr;
use safe_string::SafeString;
use std::ops::Deref;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster set-master-password -h");
    println!("    rooster set-master-password");
    println!();
    println!("Example:");
    println!("    rooster set-master-password");
}

pub fn callback_exec(
    _matches: &getopts::Matches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    match prompt_password_stderr("Type your new master password: ") {
        Ok(master_password) => {
            let master_password = SafeString::new(master_password);

            let master_password_confirmation = match prompt_password_stderr(
                "Type your new \
                 master password \
                 once more: ",
            ) {
                Ok(master_password_confirmation) => SafeString::new(master_password_confirmation),
                Err(err) => {
                    show_error(
                        format!(
                            "I could not read your new master password (reason: {:?}).",
                            err
                        )
                        .as_str(),
                    );
                    return Err(1);
                }
            };

            if master_password != master_password_confirmation {
                show_error("The master password confirmation did not match. Aborting.");
                return Err(1);
            }

            store.change_master_password(master_password.deref());
        }
        Err(err) => {
            show_error(
                format!(
                    "I could not read your new master password (reason: {:?}).",
                    err
                )
                .as_str(),
            );
            return Err(1);
        }
    }
    show_ok("Your master password has been changed.");
    Ok(())
}
