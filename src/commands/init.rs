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
use std::io::{BufRead, Write};
use std::path::PathBuf;

pub fn callback_exec<
    R: BufRead,
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    matches: &clap::ArgMatches,
    reader: &mut ReaderManager<R>,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
    rooster_file_path: &PathBuf,
) -> Result<(), i32> {
    let filename_as_string = rooster_file_path.to_string_lossy().into_owned();
    if rooster_file_path.exists() && !matches.is_present("force-for-tests") {
        writer
            .error()
            .error("Woops, there is already a Rooster file located at:");
        writer
            .error()
            .error(format!("    {}", filename_as_string).as_str());
        writer.error().error("");
        writer
            .error()
            .error("Type `rooster --help` to see what Rooster can do for you.");
        return Err(1);
    }

    writer.instruction().title("Welcome to Rooster");
    writer.instruction().newline();
    writer.instruction().info("Rooster is a simple password manager for geeks. Let's get started! Type ENTER to continue.");

    if let Err(err) = reader.read_line() {
        writer
            .error()
            .error(format!("Woops, I didn't see the ENTER key (reason: {:?}).", err).as_str());
        return Err(1);
    }

    writer.instruction().title("The master password");
    writer.instruction().newline();
    writer.instruction().info(
        "With Rooster, you only need to remember one password: \
    the master password. It keeps all of you other passwords safe. The stronger it is, the better your passwords are \
                      protected."
    );
    writer.instruction().newline();

    writer.instruction().prompt("Choose your master password: ");
    let master_password = reader.read_password().map_err(|err| {
        writer
            .error()
            .error(format!("Woops, I couldn't read the master passwords ({:?}).", err).as_str());
        1
    })?;

    if master_password.len() == 0 {
        writer
            .error()
            .error("Your master password cannot be empty.");
        return Err(1);
    }

    let store = match ::password::v2::PasswordStore::new(master_password) {
        Ok(store) => store,
        Err(err) => {
            writer.error().error(
                format!(
                    "Woops, I couldn't use the random number generator on your machine \
                     (reason: {:?}). Without it, I can't create a secure password file.",
                    err
                )
                .as_str(),
            );
            return Err(1);
        }
    };

    let mut file = match ::create_password_file(filename_as_string.as_str()).map_err(|_| 1) {
        Ok(file) => file,
        Err(err) => {
            writer.error().error(
                format!(
                    "Woops, I couldn't create a new password file (reason: {:?})",
                    err
                )
                .as_str(),
            );
            return Err(1);
        }
    };

    if let Err(err) = store.sync(&mut file) {
        if let Err(err) = ::std::fs::remove_file(rooster_file_path) {
            writer.error().error(
                format!(
                    "Woops, I was able to create a new password file but couldn't save \
                     it (reason: {:?}). You may want to remove this dangling file:",
                    err
                )
                .as_str(),
            );
            writer
                .error()
                .error(format!("    {}", filename_as_string).as_str());
            return Err(1);
        }
        writer.error().error(
            format!(
                "Woops, I couldn't create a new password file (reason: {:?}).",
                err
            )
            .as_str(),
        );
        return Err(1);
    }

    writer.instruction().newline();
    writer.instruction().title("All done and ready to rock");
    writer.instruction().newline();
    writer
        .instruction()
        .success("You passwords will be saved in:");
    writer
        .instruction()
        .success(format!("    {}", filename_as_string).as_str());
    writer.instruction().newline();
    writer.instruction().info(
        "If you wish to change the location of your password file, you can set it in the \
        ROOSTER_FILE environment variable. For instance:",
    );
    writer
        .instruction()
        .info("    export ROOSTER_FILE=path/to/passwords.rooster");
    writer.instruction().newline();
    writer
        .instruction()
        .info("Type `rooster --help` to see what Rooster can do for you.");

    Ok(())
}
