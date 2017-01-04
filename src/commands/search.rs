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
use std::io::Write;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster search -h");
    println!("    rooster search <query>");
    println!("");
    println!("Example if you want to list all Google accounts:");
    println!("    rooster search google");
}

pub fn callback_exec(matches: &getopts::Matches,
                     store: &mut password::v2::PasswordStore)
                     -> Result<(), i32> {
    if matches.free.len() < 2 {
        println_err!("Woops, seems like the app name is missing here. For help, try:");
        println_err!("    rooster search -h");
        return Err(1);
    }

    let query = matches.free[1].clone();

    let passwords = store.search_passwords(query.as_str());

    let longest_app_name = passwords.iter().fold(0, |acc, p| if p.name.len() > acc {
        p.name.len()
    } else {
        acc
    });

    for p in passwords.iter() {
        println!("{:width$} {:30}",
                 p.name,
                 p.username,
                 width = longest_app_name);
    }

    Ok(())
}
