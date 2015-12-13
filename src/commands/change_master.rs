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
use super::super::rpassword::read_password;
use super::super::safe_string::SafeString;
use std::io::Write;
use std::ops::Deref;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster change-master -h");
    println!("    rooster change-master");
    println!("");
    println!("Example:");
    println!("    rooster change-master");
}

pub fn callback_exec(_matches: &getopts::Matches, store: &mut password::v2::PasswordStore) -> Result<(), i32> {
    write!(::std::io::stderr(), "Type your new master password: ").unwrap();
    ::std::io::stderr().flush().unwrap();
    match read_password() {
        Ok(master_password) => {
            let master_password = SafeString::new(master_password);
            store.change_master_password(master_password.deref());
        }
        Err(err) => {
            println_err!("I could not read your new master password ({:?}).", err);
            return Err(1);
        }
    }
    println_ok!("Your master password has been changed.");
    Ok(())
}
