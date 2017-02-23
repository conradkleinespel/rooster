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
    println!("    rooster delete -h");
    println!("    rooster delete <app_name> ...");
    println!("");
    println!("Example:");
    println!("    rooster delete youtube");
}

pub fn check_args(matches: &getopts::Matches) -> Result<(), i32> {
    if matches.free.len() < 2 {
        println_err!("Woops, seems like the app name is missing here. For help, try:");
        println_err!("    rooster delete -h");
        return Err(1);
    }

    Ok(())
}

pub fn callback_exec(matches: &getopts::Matches,
                     store: &mut password::v2::PasswordStore)
                     -> Result<(), i32> {
    check_args(matches)?;

    let mut has_error = false;

    for app_name in &matches.free[1..] {
        match store.delete_password(app_name) {
            Ok(_) => {
                println_ok!("Done! I've deleted the password for \"{}\".", app_name);
            }
            Err(err) => {
                println_err!("Woops! I couldn't find a password for \"{}\" (error: {:?}).",
                             app_name,
                             err);
                has_error = true;
            }
        }
    }

    if has_error { Err(1) } else { Ok(()) }
}
