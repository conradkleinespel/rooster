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

use clip;
use getopts;
use password;
use list;
use std::io::Write;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster get -h");
    println!("    rooster get <query>");
    println!("");
    println!("Examples:");
    println!("    rooster get youtube");
    println!("    rooster get ytb");
}

pub fn check_args(matches: &getopts::Matches) -> Result<(), i32> {
    if matches.free.len() < 2 {
        println_err!("Woops, seems like the app name is missing here. For help, try:");
        println_err!("    rooster get -h");
        return Err(1);
    }

    Ok(())
}

pub fn callback_exec(
    matches: &getopts::Matches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    check_args(matches)?;

    let show = matches.opt_present("show");

    let query = &matches.free[1];

    let prompt = format!(
        "Which password would you like {}? ",
        if show {
            "to see"
        } else {
            "to copy to your clipboard"
        },
    );
    println_stderr!("");
    let password = list::search_and_choose_password(
        store, query, list::WITH_NUMBERS, &prompt,
    ).ok_or(1)?;

    clip::confirm_password_retrieved(show, &password);

    Ok(())
}
