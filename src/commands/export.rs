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

use super::super::getopts;
use super::super::password;
use super::super::safe_string::SafeString;
use super::super::rustc_serialize::json;
use std::ops::Deref;
use std::io::stdin;
use std::io::Write;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster export -h");
    println!("    rooster export");
    println!("");
    println!("Example:");
    println!("    rooster export");
}

pub fn callback_exec(_matches: &getopts::Matches, store: &mut password::v2::PasswordStore) -> Result<(), i32> {
    println_err!("Printing all passwords unencrypted can be risky. Are you sure you want to proceed? [y/n]");
    let msg = format!("I did not understand that. Are you sure you want to print all passwords unencrypted? [y/n]");
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => {
                if line.starts_with("n") {
                    return Ok(());
                } else if line.starts_with("y") {
                    break;
                } else {
                    println_err!("{}", msg);
                }
            },
            Err(_) => {
                println_err!("{}", msg);
                return Err(1);
            }
        }
    }

    let passwords_ref = store.get_all_passwords();
    let passwords = SafeString::new(json::encode(&passwords_ref).unwrap());
    println!("{}", passwords.deref());
    Ok(())
}
