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

use getopts;
use password;
use ffi;
use list;
use std::io::Write;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster transfer -h");
    println!("    rooster transfer <app> <new_username>");
    println!("");
    println!("Example:");
    println!("    rooster transfer dailymotion new@example.com");
}

pub fn check_args(matches: &getopts::Matches) -> Result<(), i32> {
    if matches.free.len() < 3 {
        println_err!("Woops, seems like the app name or username is missing here. For help, try:");
        println_err!("    rooster transfer -h");
        return Err(1);
    }

    Ok(())
}

pub fn callback_exec(matches: &getopts::Matches,
                     store: &mut password::v2::PasswordStore)
                     -> Result<(), i32> {
    check_args(matches)?;

    let query = &matches.free[1];
    let new_username = &matches.free[2];

    println_stderr!("");
    let password = list::search_and_choose_password(
        store, query, list::WITH_NUMBERS,
        "Which password would you like to transfer?",
    ).ok_or(1)?.clone();
    println_stderr!("");

    let old_username = password.username;

    let change_result = store.change_password(&password.name,
                                              &|old_password: password::v2::Password| {
        password::v2::Password {
            name: old_password.name.clone(),
            username: new_username.clone(),
            password: old_password.password.clone(),
            created_at: old_password.created_at,
            updated_at: ffi::time(),
        }
    });

    match change_result {
        Ok(_) => {
            println_ok!("Done! I've transferd {} to {}", old_username, new_username);
            Ok(())
        }
        Err(err) => {
            println_err!("Woops, I couldn't save the new app name (reason: {:?}).",
                         err);
            Err(1)
        }
    }
}
