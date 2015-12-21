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
use super::super::safe_string::SafeString;
use super::super::generate::{PasswordSpec, generate_hard_password};
use std::io::Write;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster generate -h");
    println!("    rooster generate <app_name> <username>");
    println!("");
    println!("Example:");
    println!("    rooster generate YouTube me@example.com");
}

pub fn callback_exec(matches: &getopts::Matches, store: &mut password::v2::PasswordStore) -> Result<(), i32> {
    if matches.free.len() < 3 {
        println_err!("Woops, seems like the app name or the username is missing here. For help, try:");
        println_err!("    rooster generate -h");
        return Err(1);
    }

    let app_name = matches.free[1].clone();
    let username = matches.free[2].clone();

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

    // Read the master password and try to save the new password.
    let password = password::v2::Password::new(
        app_name.clone(),
        username,
        SafeString::new(password_as_string)
    );

    match store.add_password(password) {
        Ok(_) => {
            println_ok!("Alright! Your password for {} has been added.", app_name);
            return Ok(());
        },
        Err(err) => {
            println_err!("\nI couldn't add this password ({:?}).", err);
            return Err(1);
        }
    }
}
