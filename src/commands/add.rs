// Copyright 2014 The Peevee Developers
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
use super::super::color::Color;
use super::super::password;
use super::super::password::ScrubMemory;
use super::super::rpassword::read_password;
use std::io::Write;

pub fn callback(args: &[String], file: &mut File) {
    let app_name = args[2].as_ref();
    let username = args[3].as_ref();


    print_now!("Type your master password: ");
    match read_password() {
        Ok(ref mut master_password) => {
            match password::has_password(master_password, app_name, file) {
                Ok(false) => {
                    print_now!("What password do you want for {}? ", app_name);
                    match read_password() {
                        Ok(ref mut password_as_string) => {
                            let mut password = password::Password::new(
                                app_name,
                                username,
                                password_as_string.as_ref()
                            );
                            let password_added = password::add_password(
                                master_password,
                                &password,
                                file
                            );
                            match password_added {
                                Ok(_) => {
                                    okln!("Alright! Your password for {} has been added.", app_name);
                                },
                                Err(err) => {
                                    errln!("I couldn't add the password ({:?}).", err);
                                    ::set_exit_status(1);
                                }
                            }

                            // Clean up memory so no one can re-use it.
                            password_as_string.scrub_memory();
                            password.scrub_memory();
                        },
                        Err(err) => {
                            errln!("\nI couldn't read the app's password ({:?}).", err);
                            ::set_exit_status(1);
                        }
                    }
                },
                Ok(true) => {
                    errln!("There is already an app with that name.");
                    ::set_exit_status(1);
                },
                Err(err) => {
                    errln!("\nI couldn't add this password ({:?}).", err);
                    ::set_exit_status(1);
                }
            }

            // Clean up memory so no one can re-use it.
            master_password.scrub_memory();
        },
        Err(err) => {
            errln!("\nI couldn't read the master password ({:?}).", err);
            ::set_exit_status(1);
        }
    }
}
