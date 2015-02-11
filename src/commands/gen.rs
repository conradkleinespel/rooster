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

use std::old_io::fs::File;
use super::super::rand::{ Rng, OsRng };
use super::super::color::Color;
use super::super::password;
use super::super::password::ScrubMemory;
use super::super::rpassword::read_password;

const PASSWORD_LEN: usize = 32;

pub fn callback(args: &[String], file: &mut File) {
    let app_name = args[2].as_slice();
    let username = args[3].as_slice();

    // Generate a random password.
    let mut buffer: [u8; PASSWORD_LEN] = [0; PASSWORD_LEN];
    let mut rng = OsRng::new().unwrap();
    for i in 0 .. PASSWORD_LEN - 1 {
        buffer[i] = rng.gen_range(33, 126);
    }
    let mut password_as_string = String::from_utf8_lossy(&buffer).into_owned();

    // Read the master password and try to save the new password.
    let mut password = password::Password::new(
        app_name,
        username,
        password_as_string.as_slice()
    );

    print!("Type your master password: ");
    match read_password() {
        Ok(ref mut master_password) => {
            match password::has_password(master_password, app_name, file) {
                Ok(false) => {
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
                            errln!("\nI couldn't add this password ({:?}).", err);
                        }
                    }
                },
                Ok(true) => {
                    errln!("There is already an app with that name.");
                },
                Err(err) => {
                    errln!("\nI couldn't add this password ({:?}).", err);
                }
            }

            // Clean up memory so no one can re-use it.
            master_password.scrub_memory();
        },
        Err(err) => {
            errln!("\nI couldn't read the master password ({:?}).", err);
        }
    }

    // Clean up memory so no one can re-use it.
    password_as_string.scrub_memory();
    password.scrub_memory();
}
