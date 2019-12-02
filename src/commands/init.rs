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
use rpassword::prompt_password_stderr;
use safe_string::SafeString;
use macros::{show_error, show_title_1};

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster init -h");
    println!("    rooster init");
    println!();
    println!("Example:");
    println!("    rooster init");
}

pub fn callback_exec(_matches: &getopts::Matches) -> Result<(), i32> {
    let (filename, filename_from_env) = match ::get_password_file_path() {
        Ok(path) => path,
        Err(_) => {
            show_error(
                "Woops, I could not read the path to your password file. Make sure it only \
                contains ASCII characters."
            );
            return Err(1);
        }
    };
    let filename_as_string = filename.to_string_lossy().into_owned();
    if filename.exists() {
        show_error("Woops, there is already a Rooster file located at:");
        show_error(format!("    {}", filename_as_string).as_str());
        show_error("");
        show_error("Type `rooster --help` to see what Rooster can do for you.");
        return Err(1);
    }

    show_title_1("Welcome to Rooster");
    println!();
    println!("Rooster is a simple password manager for geeks. Let's get started! Type ENTER to continue.");

    let mut dummy = String::new();
    if let Err(err) = ::std::io::stdin().read_line(&mut dummy) {
        show_error(format!("Woops, I didn't see the ENTER key (reason: {:?}).", err).as_str());
        return Err(1);
    }

    show_title_1("The master password");
    println!();
    println!(
        "With Rooster, you only need to remember one password: \
    the master password. It keeps all of you other passwords safe. The stronger it is, the better your passwords are \
                      protected."
    );
    println!();

    let master_password = prompt_password_stderr("Choose your master password: ")
        .map(SafeString::new)
        .map_err(|err| {
            show_error(format!("Woops, I couldn't read the master passwords ({:?}).", err).as_str());
            1
        })?;
    let store = match ::password::v2::PasswordStore::new(master_password) {
        Ok(store) => store,
        Err(err) => {
            show_error(format!(
                "Woops, I couldn't use the random number generator on your machine \
            (reason: {:?}). Without it, I can't create a secure password file.",
                err).as_str()
            );
            return Err(1);
        }
    };

    let mut file = match ::create_password_file(filename_as_string.as_str()).map_err(|_| 1) {
        Ok(file) => file,
        Err(err) => {
            show_error(format!(
                "Woops, I couldn't create a new password file (reason: {:?})",
                err).as_str()
            );
            return Err(1);
        }
    };

    if let Err(err) = store.sync(&mut file) {
        if let Err(err) = ::std::fs::remove_file(filename) {
            show_error(format!(
                "Woops, I was able to create a new password file but couldn't save \
            it (reason: {:?}). You may want to remove this dangling file:",
                err).as_str()
            );
            show_error(format!("    {}", filename_as_string).as_str());
            return Err(1);
        }
        show_error(format!(
            "Woops, I couldn't create a new password file (reason: {:?}).",
            err).as_str()
        );
        return Err(1);
    }

    println!();
    show_title_1("All done and ready to rock");
    println!();
    println!("You passwords will be saved in:");
    println!("    {}", filename_as_string);
    if !filename_from_env {
        println!();
        println!(
            "If you want to move this file, set the $ROOSTER_FILE \
            environment variable to the new path. For instance:"
        );
        println!("    export ROOSTER_FILE=path/to/passwords.rooster");
    }
    println!();
    println!("Type `rooster --help` to see what Rooster can do for you.");

    Ok(())
}
