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

use super::super::getopts;
use super::generate::PasswordSpec;
use super::generate::generate_hard_password;
use super::super::ffi;
use super::super::color::Color;
use super::super::password;
use super::super::rpassword::read_password;
use std::fs::File;
use std::io::Write;

pub fn callback(matches: &getopts::Matches, file: &mut File) {
    let app_name: &str = matches.free[1].as_ref();

    let password_spec = PasswordSpec::from_matches(matches);

    let password_as_string = match password_spec {
        None => { return; },
        Some(spec) => {
            generate_hard_password(spec.alnum, spec.len)
        }
    };

    print_now!("Type your master password: ");
    match read_password() {
        Ok(ref mut master_password) => {
            match password::get_password(master_password, app_name, file) {
                Ok(ref mut previous) => {
                    previous.password = password_as_string;
                    previous.updated_at = ffi::time();

                    let modified = password::delete_password(master_password, app_name, file).and(
                        password::add_password(master_password, previous, file)
                    );
                    match modified {
                        Ok(_) => {
                            okln!("Done ! The password for {} has been regenerated.", previous.name);
                        },
                        Err(err) => {
                            errln!("Woops, I couldn't save the new password ({:?}).", err);
                            ::set_exit_status(1);
                        }
                    }
                },
                Err(err) => {
                    errln!("Woops, I couldn't get that password. ({:?}).", err);
                    ::set_exit_status(1);
                }
            }
        },
        Err(err) => {
            errln!("\nI couldn't read the master password ({:?}).", err);
            ::set_exit_status(1);
        }
    }
}
