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
use list;
use password;
use macros::{show_error, show_ok};

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster delete -h");
    println!("    rooster delete <query>");
    println!();
    println!("Examples:");
    println!("    rooster delete youtube");
    println!("    rooster delete ytb     # fuzzy-searching works too");
}

pub fn check_args(matches: &getopts::Matches) -> Result<(), i32> {
    if matches.free.len() < 2 {
        show_error("Woops, seems like the app name is missing here. For help, try:");
        show_error("    rooster delete -h");
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

    let password = list::search_and_choose_password(
        store,
        query,
        list::WITH_NUMBERS,
        "Which password would you like me to delete?",
    ).ok_or(1)?
        .clone();

    if let Err(err) = store.delete_password(&password.name) {
        show_error(format!(
            "Woops, I couldn't delete this password (reason: {:?}).",
            err).as_str()
        );
        return Err(1);
    }

    show_ok(format!("Done! I've deleted the password for \"{}\".", password.name).as_str());

    Ok(())
}
