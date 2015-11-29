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
use super::super::rpassword::read_password;
use std::io::Write;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster add -h");
    println!("    rooster add <app_name> <username>");
    println!("");
    println!("Example:");
    println!("    rooster add YouTube me@example.com");
}

pub fn callback_exec(matches: &getopts::Matches, store: &mut password::v2::PasswordStore, master_password: &str) -> Result<(), i32> {
    if matches.free.len() < 3 {
        println_err!("Woops, seems like the app name or the username is missing here. For help, try:");
        println_err!("    rooster add -h");
        return Err(1);
    }

    let app_name = matches.free[1].as_ref();
    let username = matches.free[2].as_ref();

    match store.has_password(app_name) {
        Ok(false) => {
            write!(::std::io::stderr(), "What password do you want for {}? ", app_name).unwrap();
            ::std::io::stderr().flush().unwrap();
            match read_password() {
                Ok(password_as_string) => {
                    let password = password::v2::Password::new(
                        app_name.to_owned(),
                        username.to_owned(),
                        password_as_string
                    );
                    match store.add_password(password) {
                        Ok(_) => {
                            println_ok!("Alright! Your password for {} has been added.", app_name);
                        },
                        Err(err) => {
                            println_err!("Woops, I couldn't add the password ({:?}).", err);
                            return Err(1);
                        }
                    }

                    return Ok(());
                },
                Err(err) => {
                    println_err!("\nI couldn't read the app's password ({:?}).", err);
                    return Err(1);
                }
            }
        },
        Ok(true) => {
            println_err!("Woops, there is already an app with that name.");
            return Err(1);
        },
        Err(err) => {
            println_err!("\nWoops, I couldn't add this password ({:?}).", err);
            return Err(1);
        }
    }
}
