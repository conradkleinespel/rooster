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
use rpassword::prompt_password_stderr;
use safe_string::SafeString;
use clip::{copy_to_clipboard, paste_keys};
use std::io::Write;
use std::ops::Deref;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster add -h");
    println!("    rooster add <app_name> <username>");
    println!();
    println!("Options:");
    println!("    -s, --show        Show the password instead of copying it to the clipboard");
    println!();
    println!("Example:");
    println!("    rooster add YouTube me@example.com");
}

pub fn check_args(matches: &getopts::Matches) -> Result<(), i32> {
    if matches.free.len() < 3 {
        println_err!(
            "Woops, seems like the app name or the username is missing here. For help, \
                      try:"
        );
        println_err!("    rooster add -h");
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

    match prompt_password_stderr(
        format!("What password do you want for \"{}\"? ", app_name).as_str(),
    ) {
        Ok(password_as_string) => {
            let password_as_string_clipboard = SafeString::new(password_as_string.clone());
            let password = password::v2::Password::new(
                app_name.clone(),
                username,
                SafeString::new(password_as_string),
            );
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
                            "Hmm, I tried to copy your new password to your clipboard, \
                                     but something went wrong. Don't worry, it's saved, and you \
                                     can see it with `rooster get {} --show`",
                            app_name
                        );
                    } else {
                        println_ok!(
                            "Alright! I've saved your new password. You can paste it \
                                     anywhere with {}.",
                            paste_keys()
                        );
                    }
                }
                Err(err) => {
                    println_err!("Woops, I couldn't add the password (reason: {:?}).", err);
                    return Err(1);
                }
            }

            Ok(())
        }
        Err(err) => {
            println_err!("\nI couldn't read the app's password (reason: {:?}).", err);
            Err(1)
        }
    }
}
