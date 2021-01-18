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

use io::{ReaderManager, WriterManager};
use password;
use std::io::{BufRead, Write};
use std::ops::Deref;

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
    writer
        .instruction()
        .prompt("Type your new master password: ");
    match reader.read_password() {
        Ok(master_password) => {
            writer
                .instruction()
                .prompt("Type your new master password once more: ");
            let master_password_confirmation = match reader.read_password() {
                Ok(master_password_confirmation) => master_password_confirmation,
                Err(err) => {
                    writer.error().error(
                        format!(
                            "I could not read your new master password (reason: {:?}).",
                            err
                        )
                        .as_str(),
                    );
                    return Err(1);
                }
            };

            if master_password != master_password_confirmation {
                writer
                    .error()
                    .error("The master password confirmation did not match. Aborting.");
                return Err(1);
            }

            store.change_master_password(master_password.deref());
        }
        Err(err) => {
            writer.error().error(
                format!(
                    "I could not read your new master password (reason: {:?}).",
                    err
                )
                .as_str(),
            );
            return Err(1);
        }
    }
    writer
        .output()
        .success("Your master password has been changed.");
    Ok(())
}
