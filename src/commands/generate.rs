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

use std::fs::File;
use super::super::getopts;
use super::super::password;
use super::super::rand::{Rng, OsRng};
use std::io::Write;

fn generate_password(alnum: bool, len: usize) -> String {
    let mut password_as_string = String::new();
    let mut rng = OsRng::new().unwrap();
    for _ in 0 .. len {
        if alnum {
            match rng.gen_range(0, 3) {
                // Numbers 0-9
                0 => { password_as_string.push(rng.gen_range(48, 58) as u8 as char) },
                // Uppercase A-Z
                1 => { password_as_string.push(rng.gen_range(65, 91) as u8 as char) },
                // Lowercase a-z
                2 => { password_as_string.push(rng.gen_range(97, 123) as u8 as char) },
                _ => { panic!("Unexpected random value.") }
            }
        } else {
            password_as_string.push(rng.gen_range(33, 127) as u8 as char);
        }
    }
    password_as_string
}

/// Returns true if the password contains at least one digit, one uppercase letter and one
/// lowercase letter.
fn password_is_hard(password: &str) -> bool {
    if password.find(char::is_numeric).is_some()
    && password.find(char::is_lowercase).is_some()
    && password.find(char::is_uppercase).is_some() {
        true
    } else {
        false
    }
}

pub fn generate_hard_password(alnum: bool, len: usize) -> String {
    loop {
        let password = generate_password(alnum, len);
        if password_is_hard(password.as_ref()) {
            return password;
        }
    }
}

pub struct PasswordSpec {
    pub alnum: bool,
    pub len: usize
}

impl PasswordSpec {
    pub fn from_matches(matches: &getopts::Matches) -> Option<PasswordSpec> {
        let alnum = matches.opt_present("alnum");
        let mut password_len = 32;
        if let Some(len) = matches.opt_str("length") {
            password_len = match len.parse::<usize>() {
                Ok(parsed_len) => {
                    // We want passwords to contain at least one uppercase letter, one lowercase
                    // letter and one digit. So we need at least 3 characters for each password.
                    // This checks makes sure we don't run into an infinite loop trying to generate
                    // a password of length 2 with 3 different kinds of characters.
                    if parsed_len < 3 {
                        println_err!("Woops! The length of the password must be at least 3. This allows us");
                        println_err!("to make sure each password contains at least one lowercase letter, one");
                        println_err!("uppercase letter and one number.");
                        return None;
                    }
                    parsed_len
                },
                Err(_) => {
                    println_err!("Woops! The length option must be a valid number, for instance 8 or 16.");
                    return None;
                }
            }
        }
        Some(PasswordSpec {
            alnum: alnum,
            len: password_len
        })
    }
}

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster generate -h");
    println!("    rooster generate <app_name> <username>");
    println!("");
    println!("Example:");
    println!("    rooster generate YouTube me@example.com");
}

pub fn callback_exec(matches: &getopts::Matches, file: &mut File, master_password: &str) -> Result<(), i32> {
    if matches.free.len() < 3 {
        println_err!("Woops, seems like the app name or the username is missing here. For help, try:");
        println_err!("    rooster generate -h");
        return Err(1);
    }

    let app_name = matches.free[1].as_ref();
    let username = matches.free[2].as_ref();

    let password_spec = PasswordSpec::from_matches(matches);

    let mut password_as_string = match password_spec {
        None => { return Err(1); },
        Some(spec) => {
            generate_hard_password(spec.alnum, spec.len)
        }
    };

    // Read the master password and try to save the new password.
    let mut password = password::v2::Password::new(
        app_name,
        username,
        password_as_string.as_ref()
    );

    match password::v2::has_password(master_password, app_name, file) {
        Ok(false) => {
            let password_added = password::v2::add_password(
                master_password,
                &password,
                file
            );
            match password_added {
                Ok(_) => {
                    println_ok!("Alright! Your password for {} has been added.", app_name);
                    return Ok(());
                },
                Err(err) => {
                    println_err!("\nI couldn't add this password ({:?}).", err);
                    return Err(1);
                }
            }
        },
        Ok(true) => {
            println_err!("There is already an app with that name.");
            return Err(1);
        },
        Err(err) => {
            println_err!("\nI couldn't add this password ({:?}).", err);
            return Err(1);
        }
    }
}
