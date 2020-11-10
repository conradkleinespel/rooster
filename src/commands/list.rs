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

use list;
use password;

pub fn callback_exec(
    _matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    let passwords = store.get_all_passwords();

    if passwords.len() == 0 {
        println!("No passwords on record yet. Add one with `rooster add <app> <username>`.");
    } else {
        list::print_list_of_passwords(
            &passwords,
            list::WITHOUT_NUMBERS,
            list::OutputStream::Stdout,
        );
    }

    Ok(())
}
