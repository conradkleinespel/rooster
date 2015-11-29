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
use std::io::Write;
use std::iter::repeat;
use std::iter::FromIterator;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster list -h");
    println!("    rooster list");
    println!("");
    println!("Example:");
    println!("    rooster list");
}

pub fn callback_exec(_matches: &getopts::Matches, file: &mut File, master_password: &str) -> Result<(), i32> {
    match password::v2::get_all_passwords(master_password, file) {
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
            return Ok(());
        },
        Err(err) => {
            println_err!("Woops, I could not retrieve passwords ({:?}).", err);
            return Err(1);
        }
    }
}
