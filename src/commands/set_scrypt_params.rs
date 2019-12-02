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
use macros::show_error;
use password;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster set-scrypt-params -h");
    println!("    rooster set-scrypt-params <log2n> <r> <p>");
    println!();
    println!("Example:");
    println!("    rooster set-scrypt-params 12 8 1");
}

pub fn check_args(matches: &getopts::Matches) -> Result<(), i32> {
    if matches.free.len() < 4 {
        show_error("Woops, seems like a param is missing here. For help, try:");
        show_error("    rooster set-scrypt-params -h");
        return Err(1);
    }

    Ok(())
}

pub fn callback_exec(
    matches: &getopts::Matches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    check_args(matches)?;

    let log2_n = matches.free[1].trim().parse::<u8>().unwrap();
    let r = matches.free[2].trim().parse::<u32>().unwrap();
    let p = matches.free[3].trim().parse::<u32>().unwrap();

    if log2_n > 20 || r > 8 || p > 1 {
        show_error("These parameters seem very high. You might be unable to open your password file ever again. Aborting.");
        return Err(1);
    }

    store.change_scrypt_params(log2_n, r, p);

    Ok(())
}
