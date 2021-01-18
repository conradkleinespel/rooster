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

use ffi;
use io::{ReaderManager, WriterManager};
use list;
use password;
use std::io::{BufRead, Write};

pub fn callback_exec<
    R: BufRead,
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    reader: &mut ReaderManager<R>,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> Result<(), i32> {
    let query = matches.value_of("app").unwrap();
    let new_name = matches.value_of("new_name").unwrap().to_owned();

    let password = list::search_and_choose_password(
        store,
        query,
        list::WITH_NUMBERS,
        "Which password would you like to rename?",
        reader,
        writer,
    )
    .ok_or(1)?
    .clone();

    let change_result =
        store.change_password(&password.name, &|old_password: password::v2::Password| {
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
            writer
                .output()
                .success(format!("Done! I've renamed {} to {}", password.name, new_name).as_str());
            Ok(())
        }
        Err(err) => {
            writer.error().error(
                format!(
                    "Woops, I couldn't save the new app name (reason: {:?}).",
                    err
                )
                .as_str(),
            );
            Err(1)
        }
    }
}
