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
use super::super::clipboard::{copy_to_clipboard, paste_keys};
use std::io::Write;
use std::ops::Deref;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster get -h");
    println!("    rooster get <app_name>");
    println!("");
    println!("Example:");
    println!("    rooster get youtube");
    println!("    rooster get -s youtube");
}

pub fn callback_exec(matches: &getopts::Matches,
                     store: &mut password::v2::PasswordStore)
                     -> Result<(), i32> {
    if matches.free.len() < 2 {
        println_err!("Woops, seems like the app name is missing here. For help, try:");
        println_err!("    rooster get -h");
        return Err(1);
    }

    let app_name = &matches.free[1];

    match store.get_password(app_name) {
        Some(ref password) => {
            if matches.opt_present("show") {
                println_ok!("Alright! Here is your password: {}",
                            password.password.deref());
                return Ok(());
            }

            if copy_to_clipboard(password.password.deref()).is_err() {
                println_ok!("Hmm, I tried to copy your new password to your clipboard, but \
                             something went wrong. You can see it with `rooster get {} --show`",
                            app_name);
            } else {
                println_ok!("Alright! You can paste your password anywhere with {}.",
                            paste_keys());
            }
            Ok(())
        }
        None => {
            println_err!("I couldn't find a password for this app. Make sure you");
            println_err!("didn't make a typo. For a list of passwords, try:");
            println_err!("    rooster list");
            Err(1)
        }
    }
}
