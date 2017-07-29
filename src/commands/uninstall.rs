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
use get_password_file_path;
use quale::which;
use std::ops::Deref;

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster uninstall -h");
    println!("    rooster uninstall");
    println!();
    println!("Example:");
    println!("    rooster uninstall");
}

pub fn callback_exec(_matches: &getopts::Matches) -> Result<(), i32> {
    println!("To uninstall Rooster from your system, run the following commands:");
    println!(
        "    sudo rm {}",
        which("rooster").unwrap().to_string_lossy()
    );

    match get_password_file_path().ok() {
        Some((filename, from_env)) => {
            println!();
            println!("If you want to remove your password file as well, you can — just make sure you don't lock yourself out of your online accounts. It is located at:");
            println!("    {}", filename.to_string_lossy().deref());
            if from_env {
                println!();
                println!("Seems like you've set the ROOSTER_FILE environment variable in your shell \
                configuration. You may want to remove it to clean things up.");
            }
        }
        None => {}
    }

    Ok(())
}
