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
use std::ops::Deref;
use zxcvbn::zxcvbn;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster weak -h");
    println!("    rooster weak");
    println!();
    println!("Options:");
    println!("    -s, --show        Show the password next to the app name");
    println!();
    println!("Examples:");
    println!("    rooster weak");
}

pub fn callback_exec(
    matches: &getopts::Matches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    let show_password = matches.opt_present("show");

    let mut weak_passwords: Vec<password::v2::Password> = Vec::new();

    for password in store.get_all_passwords() {
        if let Ok(result) = zxcvbn(password.password.deref(), &[]) {
            // Score is from 0 to 4. Anything lower than 4 should be considered not good enough.
            if result.score <= 3 {
                weak_passwords.push(password.clone());
            }
        }
    }

    let longest_password = weak_passwords.iter().fold(0, |acc, p| {
        if p.name.len() > acc {
            p.name.len()
        } else {
            acc
        }
    });

    if weak_passwords.len() > 0 {
        println!("The following passwords are weak:");
        for password in weak_passwords {
            if show_password {
                println!(
                    "{:app_name_width$} {}",
                    password.name,
                    password.password.deref(),
                    app_name_width = longest_password
                );
            } else {
                println!("{}", password.name);
            }
        }
    } else {
        println!("All of your passwords are strong, good job!");
    }

    Ok(())
}
