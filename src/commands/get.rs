// Copyright 2014 The Peevee Developers
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

use std::old_io::fs::File;
use std::old_io::stdio::stdin;
use super::super::color::Color;
use super::super::password;
use super::super::password::ScrubMemory;
use super::super::rpassword::read_password;

pub fn callback(args: &[String], file: &mut File) {
    let ref app_name = args[2];

    print!("Type your master password: ");
    match read_password() {
        Ok(ref mut master_password) => {
            match password::get_passwords(master_password, app_name, file) {
                Ok(ref mut passwords) => {
                    let border: String = range(0, 92).map(|_| '-').collect();

                    println!("");

                    println!("{}", border);
                    println!("| {:2} | {:15} | {:30} | {:32} |", "id", "app", "username", "password");
                    println!("{}", border);
                    let mut i = 0;
                    for p in passwords.as_slice().iter() {
                        println!("| {:2?} | {:15} | {:30} | {:32} |", i, p.name, p.username, p.password);
                        i += 1;
                    }
                    println!("{}", border);

                    println!("");

                    print!("These passwords match your search. Type the id of the password you want: ");
                    match stdin().read_line() {
                        Ok(ref mut line) => {
                            // Remove the \n from the line.
                            line.pop().unwrap();

                            // We're all good !
                            println_stderr!("{}", fgcolor!(Color::Green, "OK {}", line));
                        },
                        Err(_) => {
                            println!("");
                            println_stderr!("{}", fgcolor!(Color::Red, "error: could not read the password ID"));
                        }
                    }

                    passwords.scrub_memory();
                },
                Err(err) => {
                    println_stderr!("{}", fgcolor!(Color::Red, "error: could not read passwords: {:?}", err));
                }
            }
            master_password.scrub_memory();
        },
        Err(_) => {
            println_stderr!("");
            println_stderr!("{}", fgcolor!(Color::Red, "error: could not read the master password"));
        }
    }
}
