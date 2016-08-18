// Copyright 2014 The Rooster Developers
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

use super::super::getopts;
use super::super::password;
use super::super::rpassword::read_password;
use super::super::safe_string::SafeString;
use super::super::clipboard::{copy_to_clipboard, paste_keys};
use super::super::ffi;
use std::io::Write;
use std::ops::Deref;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster change -h");
    println!("    rooster change <app_name>");
    println!("");
    println!("Example:");
    println!("    rooster change youtube");
}

pub fn callback_exec(matches: &getopts::Matches, store: &mut password::v2::PasswordStore) -> Result<(), i32> {
    if matches.free.len() < 2 {
        println_err!("Woops, seems like the app name is missing here. For help, try:");
        println_err!("    rooster change -h");
        return Err(1);
    }

    let app_name = matches.free[1].clone();

    print_stderr!("What password do you want for \"{}\"? ", app_name);
    match read_password() {
        Ok(password_as_string) => {
            let password_as_string = SafeString::new(password_as_string.clone());

            let change_result = store.change_password(app_name.deref(), &|old_password: password::v2::Password| {
                password::v2::Password {
                    name: old_password.name.clone(),
                    username: old_password.username.clone(),
                    password: password_as_string.clone(),
                    created_at: old_password.created_at,
                    updated_at: ffi::time(),
                }
            });

            match change_result {
                Ok(_) => {
                    if matches.opt_present("show") {
                        println_ok!("Alright! Here is your new password: {}", password_as_string.deref());
                        return Ok(());
                    }

                    if copy_to_clipboard(password_as_string.deref()).is_err() {
                        println_ok!("Alright! Here is your new password: {}", password_as_string.deref());
                        return Err(1);
                    }

                    println_ok!("Done! I've saved your new password for \"{}\". You can paste it anywhere with {}.", app_name, paste_keys());
                    Ok(())
                }
                Err(err) => {
                    println_err!("Woops, I couldn't save the new password ({:?}).", err);
                    Err(1)
                }
            }
        },
        Err(err) => {
            println_err!("\nI couldn't read the app's password ({:?}).", err);
            Err(1)
        }
    }
}
