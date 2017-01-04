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

use super::super::getopts;
use super::super::password;
use std::iter::{Iterator, FromIterator, repeat};
use libc::isatty;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster list -h");
    println!("    rooster list");
    println!("");
    println!("Example:");
    println!("    rooster list");
}

pub fn callback_exec(_matches: &getopts::Matches,
                     store: &mut password::v2::PasswordStore)
                     -> Result<(), i32> {
    let all_passwords = store.get_all_passwords();

    let output_is_piped = unsafe { isatty(1) } == 0;

    if all_passwords.len() == 0 {
        if !output_is_piped {
            println!("No passwords on record yet. Add one with 'rooster add <app> <username>'.");
        }
    } else {
        // We'll now print the password in a table.
        // The table is delimited by borders.
        let horizontal_border = String::from_iter(repeat('-').take(73));

        if !output_is_piped {
            println!("{}", horizontal_border);
            println!("| {:2} | {:30} | {:30} |", "id", "app", "username");
            println!("{}", horizontal_border);
        }
        for (i, p) in all_passwords.iter().enumerate() {
            println!("| {:2?} | {:30} | {:30} |", i, p.name, p.username);
        }
        if !output_is_piped {
            println!("{}", horizontal_border);
        }
    }

    Ok(())
}
