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
use super::super::safe_string::SafeString;
use super::super::ffi;
use super::super::password;
use super::super::generate::{PasswordSpec, generate_hard_password};
use super::super::clipboard::{copy_to_clipboard, paste_keys};
use std::io::Write;
use std::ops::Deref;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster regenerate -h");
    println!("    rooster regenerate <app_name>");
    println!("");
    println!("Example:");
    println!("    rooster regenerate youtube");
}

pub fn callback_exec(matches: &getopts::Matches, store: &mut password::v2::PasswordStore) -> Result<(), i32> {
    if matches.free.len() < 2 {
        println_err!("Woops, seems like the app name is missing here. For help, try:");
        println_err!("    rooster regenerate -h");
        return Err(1);
    }

    let app_name = matches.free[1].clone();

    let password_spec = PasswordSpec::from_matches(matches);

    let password_as_string = match password_spec {
        None => { return Err(1); },
        Some(spec) => {
            match generate_hard_password(spec.alnum, spec.len) {
                Ok(password_as_string) => password_as_string,
                Err(io_err) => {
                    println_stderr!("Woops, I could not generate the password ({:?}).", io_err);
                    return Err(1);
                }
            }
        }
    };

    match store.delete_password(app_name.deref()) {
        Ok(mut previous) => {
            let password_as_string_clipboard = SafeString::new(password_as_string.clone());
            previous.password = SafeString::new(password_as_string);
            previous.updated_at = ffi::time();

            match store.add_password(previous) {
                Ok(_) => {
                    if matches.opt_present("show") {
                        println_ok!("Alright! Here is your password: {}", password_as_string_clipboard.deref());
                        return Ok(());
                    }

                    if copy_to_clipboard(password_as_string_clipboard.deref()).is_err() {
                        println_ok!("Alright! I've saved your new password for {}. Here it is: {}", app_name, password_as_string_clipboard.deref());
                        return Err(1);
                    }

                    println_ok!("Done! I've saved your new password for {}. You can paste it anywhere with {}.", app_name, paste_keys());
                    return Ok(());
                },
                Err(err) => {
                    println_err!("Woops, I couldn't save the new password ({:?}).", err);
                    return Err(1);
                }
            }
        },
        Err(err) => {
            println_err!("Woops, I couldn't get that password ({:?}).", err);
            return Err(1);
        }
    }
}
