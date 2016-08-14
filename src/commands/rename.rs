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
use super::super::ffi;
use std::io::Write;
use std::ops::Deref;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster rename -h");
    println!("    rooster rename <old_app_name> <new_app_name>");
    println!("");
    println!("Example:");
    println!("    rooster rename youtube Dailymotion");
}

pub fn callback_exec(matches: &getopts::Matches, store: &mut password::v2::PasswordStore) -> Result<(), i32> {
    if matches.free.len() < 3 {
        println_err!("Woops, seems like the app name is missing here. For help, try:");
        println_err!("    rooster rename -h");
        return Err(1);
    }

    let old_name = matches.free[1].clone();
    let new_name = matches.free[2].clone();

    let change_result = store.change_password(old_name.deref(), &|old_password: password::v2::Password| {
        password::v2::Password {
            name: new_name.clone(),
            username: old_password.username.clone(),
            password: old_password.password.clone(),
            created_at: old_password.created_at,
            updated_at: ffi::time(),
        }
    });

    match change_result {
        Ok(_) => {
            println_ok!("Done! I've renamed `{}` to `{}`", old_name, new_name);
            Ok(())
        }
        Err(err) => {
            println_err!("Woops, I couldn't save the new password ({:?}).", err);
            Err(1)
        }
    }
}
