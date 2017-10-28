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
use rpassword::prompt_password_stderr;
use safe_string::SafeString;
use clip;
use ffi;
use list;
use std::io::Write;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster change -h");
    println!("    rooster change <query>");
    println!();
    println!("Options:");
    println!("    -s, --show        Show the password instead of copying it to the clipboard");
    println!();
    println!("Examples:");
    println!("    rooster change youtube");
    println!("    rooster change ytb     # fuzzy-searching works too");
}

pub fn check_args(matches: &getopts::Matches) -> Result<(), i32> {
    if matches.free.len() < 2 {
        println_err!("Woops, seems like the app name is missing here. For help, try:");
        println_err!("    rooster change -h");
        return Err(1);
    }

    Ok(())
}

pub fn callback_exec(
    matches: &getopts::Matches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    check_args(matches)?;

    let query = &matches.free[1];

    println_stderr!("");
    let password = list::search_and_choose_password(
        store,
        query,
        list::WITH_NUMBERS,
        "Which password would like to update?",
    ).ok_or(1)?
        .clone();

    println_stderr!("");
    // TODO: prevent empty passwords
    let password_as_string = prompt_password_stderr(
        format!("What password do you want for \"{}\"? ", password.name)
            .as_str(),
    ).map_err(|err| {
        println_err!("\nI couldn't read the app's password (reason: {:?}).", err);
        1
    })?;

    let password_as_string = SafeString::new(password_as_string);

    let password = store
        .change_password(&password.name, &|old_password: password::v2::Password| {
            password::v2::Password {
                name: old_password.name,
                username: old_password.username,
                password: password_as_string.clone(),
                created_at: old_password.created_at,
                updated_at: ffi::time(),
            }
        })
        .map_err(|err| {
            println_err!(
                "Woops, I couldn't save the new password (reason: {:?}).",
                err
            );
            1
        })?;

    let show = matches.opt_present("show");
    clip::confirm_password_retrieved(show, &password);
    Ok(())
}
