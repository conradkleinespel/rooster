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
use ffi;
use list;
use password;
use generate::{PasswordSpec, generate_hard_password};
use clip;
use std::io::Write;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster regenerate -h");
    println!("    rooster regenerate <query>");
    println!("");
    println!("Examples:");
    println!("    rooster regenerate youtube");
    println!("    rooster regenerate ytb");
}

pub fn check_args(matches: &getopts::Matches) -> Result<(), i32> {
    if matches.free.len() < 2 {
        println_err!("Woops, seems like the app name is missing here. For help, try:");
        println_err!("    rooster regenerate -h");
        return Err(1);
    }

    Ok(())
}

pub fn callback_exec(matches: &getopts::Matches,
                     store: &mut password::v2::PasswordStore)
                     -> Result<(), i32> {
    check_args(matches)?;

    let query = &matches.free[1];

    println_stderr!("");
    let password = list::search_and_choose_password(
        store, query, list::WITH_NUMBERS,
        "Which password would you like to regenerate?",
    ).ok_or(1)?.clone();

    let password_spec = PasswordSpec::from_matches(matches);

    let password_as_string = match password_spec {
        None => {
            return Err(1);
        }
        Some(spec) => {
            match generate_hard_password(spec.alnum, spec.len) {
                Ok(password_as_string) => password_as_string,
                Err(io_err) => {
                    println_stderr!("Woops, I could not generate the password (reason: {:?}).",
                                    io_err);
                    return Err(1);
                }
            }
        }
    };

    let change_result = store.change_password(&password.name,
                                              &|old_password: password::v2::Password| {
        password::v2::Password {
            name: old_password.name.clone(),
            username: old_password.username.clone(),
            password: password_as_string.clone(),
            created_at: old_password.created_at,
            updated_at: ffi::time(),
        }
    });

    match change_result {
        Ok(_) => {
            let show = matches.opt_present("show");
            clip::confirm_password_retrieved(show, &password);
            Ok(())
        }
        Err(err) => {
            println_err!("Woops, I couldn't save the new password (reason: {:?}).",
                         err);
            Err(1)
        }
    }
}
