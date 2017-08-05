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
use password;
use generate::{PasswordSpec, generate_hard_password};
use clip::{copy_to_clipboard, paste_keys};
use std::io::Write;
use std::ops::Deref;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster generate -h");
    println!("    rooster generate <app_name> <username>");
    println!("");
    println!("Example:");
    println!("    rooster generate YouTube me@example.com");
}

pub fn check_args(matches: &getopts::Matches) -> Result<(), i32> {
    if matches.free.len() < 3 {
        println_err!(
            "Woops, seems like the app name or the username is missing here. For help, \
        try:"
        );
        println_err!("    rooster generate -h");
        return Err(1);
    }

    Ok(())
}

pub fn callback_exec(
    matches: &getopts::Matches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    check_args(matches)?;

    let app_name = matches.free[1].clone();
    let username = matches.free[2].clone();

    if store.has_password(app_name.deref()) {
        println_err!("Woops, there is already an app with that name.");
        return Err(1);
    }

    let password_spec = PasswordSpec::from_matches(matches);

    let password_as_string = match password_spec {
        None => {
            return Err(1);
        }
        Some(spec) => {
            match generate_hard_password(spec.alnum, spec.len) {
                Ok(password_as_string) => password_as_string,
                Err(io_err) => {
                    println_stderr!(
                        "Woops, I could not generate the password (reason: {:?}).",
                        io_err
                    );
                    return Err(1);
                }
            }
        }
    };

    // Read the master password and try to save the new password.
    let password_as_string_clipboard = password_as_string.clone();
    let password = password::v2::Password::new(app_name.clone(), username, password_as_string);

    match store.add_password(password) {
        Ok(_) => {
            if matches.opt_present("show") {
                println_ok!(
                    "Alright! Here is your password: {}",
                    password_as_string_clipboard.deref()
                );
                return Ok(());
            }

            if copy_to_clipboard(&password_as_string_clipboard).is_err() {
                println_ok!(
                    "Hmm, I tried to copy your new password to your clipboard, but \
                             something went wrong. Don't worry, it's saved, and you can see it \
                             with `rooster get {} --show`",
                    app_name
                );
            } else {
                println_ok!(
                    "Alright! I've saved your new password. You can paste it anywhere \
                             with {}.",
                    paste_keys()
                );
            }

            Ok(())
        }
        Err(err) => {
            println_err!("\nI couldn't add this password (reason: {:?}).", err);
            Err(1)
        }
    }
}
