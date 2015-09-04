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
use super::super::getopts;
use super::super::color::Color;
use super::super::password;
use super::super::password::ScrubMemory;
use super::super::rpassword::read_password;
use std::io::Write;
use std::iter::repeat;
use std::iter::FromIterator;

pub fn callback(_: &getopts::Matches, file: &mut File) {
    print_now!("Type your master password: ");
    match read_password() {
        Ok(ref mut master_password) => {
            match password::get_all_passwords(master_password, file) {
                Ok(ref mut passwords) => {
                    match passwords.len() {
                        0 => {
                            println!("There are no passwords saved in your file.");
                        },
                        _ => {
                            // We'll now print the password in a table.
                            // The table is delimited by borders.
                            let horizontal_border = String::from_iter(repeat('-').take(73));

                            println!("{}", horizontal_border);
                            println!("| {:2} | {:30} | {:30} |", "id", "app", "username");
                            println!("{}", horizontal_border);
                            let mut i = 0;
                            for p in passwords.iter() {
                                println!("| {:2?} | {:30} | {:30} |", i, p.name, p.username);
                                i += 1;
                            }
                            println!("{}", horizontal_border);
                        }
                    }

                    // Clean up memory so no one can re-use it.
                    passwords.scrub_memory();
                },
                Err(err) => {
                    errln!("Woops, I could not retrieve passwords ({:?}).", err);
                    ::set_exit_status(1);
                }
            }

            // Clean up memory so no one can re-use it.
            master_password.scrub_memory();
        },
        Err(err) => {
            errln!("\nWoops, I couldn't read the master password ({:?}).", err);
            ::set_exit_status(1);
        }
    }
}
