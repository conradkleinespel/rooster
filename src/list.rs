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

use password::v2::Password;
use std::io::Write;
use std::io::stdin;

/// Used to indicate lists should have a number, ie: 23 Google my.account@gmail.com
pub const WITH_NUMBERS: bool = true;

/// Used to indicate lists should not have a number, ie: Google my.account@gmail.com
pub const WITHOUT_NUMBERS: bool = false;

pub enum OutputStream {
    Stdout,
    Stderr,
}

pub fn print_list_of_passwords(passwords: &Vec<&Password>, with_numbers: bool, output_stream: OutputStream) {
    // Find the app name column length
    let longest_app_name = passwords.iter().fold(0, |acc, p| if p.name.len() > acc {
        p.name.len()
    } else {
        acc
    });

    // Find the number column length
    let i_width = ((passwords.len() as f64).log10() + 1 as f64).floor() as usize;

    for (i, p) in passwords.iter().enumerate() {
        let s = match with_numbers {
            WITH_NUMBERS => {
                format!(
                    "{:i_width$} {:app_name_width$} {:30}",
                    i + 1,
                    p.name,
                    p.username,
                    i_width = i_width,
                    app_name_width = longest_app_name
                )
            }
            WITHOUT_NUMBERS => {
                format!(
                    "{:app_name_width$} {:30}",
                    p.name,
                    p.username,
                    app_name_width = longest_app_name
                )
            }
        };

        match output_stream {
            OutputStream::Stdout => println!("{}", s),
            OutputStream::Stderr => println_stderr!("{}", s)
        }
    }
}

fn request_password_index_from_stdin(passwords: &Vec<&Password>, prompt: &str) -> usize {
    // Read the index from the command line and convert to a number
    let mut line = String::new();
    loop {
        println_stderr!("{}", prompt);

        line.clear();
        match stdin().read_line(&mut line) {
            Ok(_) => {
                match line.trim().parse() {
                    Ok(index) => {
                        if index == 0 || index > passwords.len() {
                            println_err!("I need a number between 1 and {}. Let's try again:", passwords.len());
                            continue;
                        }

                        return index;
                    }
                    Err(err) => {
                        println_err!("This isn't a valid number (reason: {}). Let's try again (1 to {}): ", err, passwords.len());
                        continue;
                    }
                };
            }
            Err(err) => {
                println_err!("I couldn't read that (reason: {}). Let's try again (1 to {}): ", err, passwords.len());
            }
        }
    }
}

pub fn choose_password_in_list(
    passwords: &Vec<&Password>,
    with_numbers: bool,
    prompt: &str,
) -> usize {
    print_list_of_passwords(passwords, with_numbers, OutputStream::Stderr);
    println_stderr!("");
    request_password_index_from_stdin(passwords, prompt)
}
