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
use super::generate::PasswordSpec;
use super::generate::generate_hard_password;
use super::super::safe_string::SafeString;
use super::super::ffi;
use super::super::password;
use std::io::Write;
use std::ops::Deref;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster regenerate -h");
    println!("    rooster regenerate <app_name>");
    println!("");
    println!("Example:");
    println!("    rooster regenerate youtube");
}

pub fn callback_exec(matches: &getopts::Matches, store: &mut password::v2::PasswordStore) -> Result<(), i32> {
    if matches.free.len() < 2 {
        println_err!("Woops, seems like the app name is missing here. For help, try:");
        println_err!("    rooster regenerate -h");
        return Err(1);
    }

    let app_name = matches.free[1].clone();

    let password_spec = PasswordSpec::from_matches(matches);

    let password_as_string = match password_spec {
        None => { return Err(1); },
        Some(spec) => {
            generate_hard_password(spec.alnum, spec.len)
        }
    };

    match store.delete_password(app_name.deref()) {
        Ok(mut previous) => {
            previous.password = SafeString::new(password_as_string);
            previous.updated_at = ffi::time();

            match store.add_password(previous) {
                Ok(_) => {
                    println_ok!("Done ! The password for {} has been regenerated.", app_name);
                    return Ok(());
                },
                Err(err) => {
                    println_err!("Woops, I couldn't save the new password ({:?}).", err);
                    return Err(1);
                }
            }
        },
        Err(err) => {
            println_err!("Woops, I couldn't get that password ({:?}).", err);
            return Err(1);
        }
    }
}
