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
use std::io::Write;

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
            println_err!(
                "Woops, I could not read the path to your password file. Make sure it only \
                contains ASCII characters."
            );
            return Err(1);
        }
    };
    let filename_as_string = filename.to_string_lossy().into_owned();
    if filename.exists() {
        println_err!("Woops, there is already a Rooster file located at:");
        println_err!("    {}", filename_as_string);
        println_err!("");
        println_err!("Type `rooster --help` to see what Rooster can do for you.");
        return Err(1);
    }

    println_title!("|---------- Welcome to Rooster  ---------|");
    println!("");
    println!("Rooster is a simple password manager for geeks.");
    println!("");
    println!("Let's get started! Type ENTER to continue.");

    let mut dummy = String::new();
    if let Err(err) = ::std::io::stdin().read_line(&mut dummy) {
        println_err!("Woops, I didn't see the ENTER key (reason: {:?}).", err);
        return Err(1);
    }

    println_title!("|---------- Set Master Password ---------|");
    println!("");
    println!(
        "With Rooster, you only need to remember one password: \
    the Master Password. It keeps all of you other passwords safe."
    );
    println!("");
    println!(
        "The stronger it is, the better your passwords are \
                      protected."
    );
    println!("");

    let master_password = prompt_password_stderr("What would you like it to be? ")
        .map(SafeString::new)
        .map_err(|err| {
            println_err!("Woops, I couldn't read the master passwords ({:?}).", err);
            1
        })?;

    let store = match ::password::v2::PasswordStore::new(master_password) {
        Ok(store) => store,
        Err(err) => {
            println_err!(
                "Woops, I couldn't use the random number generator on your machine \
            (reason: {:?}). Without it, I can't create a secure password file.",
                err
            );
            return Err(1);
        }
    };

    let mut file = match ::create_password_file(filename_as_string.as_str()).map_err(|_| 1) {
        Ok(file) => file,
        Err(err) => {
            println_err!(
                "Woops, I couldn't create a new password file (reason: {:?})",
                err
            );
            return Err(1);
        }
    };

    if let Err(err) = store.sync(&mut file) {
        if let Err(err) = ::std::fs::remove_file(filename) {
            println_err!(
                "Woops, I was able to create a new password file but couldn't save \
            it (reason: {:?}). You may want to remove this dangling file:",
                err
            );
            println_err!("    {}", filename_as_string);
            return Err(1);
        }
        println_err!(
            "Woops, I couldn't create a new password file (reason: {:?}).",
            err
        );
        return Err(1);
    }

    println!("");
    println_title!("|--- All set, you can now use Rooster ---|");
    println!("");
    println!("You passwords will be saved in:");
    println!("    {}", filename_as_string);
    if !filename_from_env {
        println!("");
        println!(
            "If you want to move this file, set the $ROOSTER_FILE \
            environment variable to the new path. For instance:"
        );
        println!("    export ROOSTER_FILE=path/to/passwords.rooster");
    }
    println!("");
    println!("Type `rooster --help` to see what Rooster can do for you.");

    Ok(())
}
