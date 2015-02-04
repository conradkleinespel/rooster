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

use super::super::color::Color;
use super::super::password;
use super::super::rpassword::read_password;
use std::old_io::fs::File;

macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::old_io::stdio::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

macro_rules! fgcolor(
    ($c:expr, $($args:tt)*) => (
        format!("{}{}\x1b[39m", $c.to_color_code(), format!($($args)*))
    )
);

pub fn callback(args: &[String], file: &mut File) {
    let app_name = args[2].as_slice();
    let username = args[3].as_slice();

    print!("What password do you want for {}? ", app_name);
    match read_password() {
        Ok(password) => {
            let mut password = password::Password::new(
                app_name,
                username,
                password.as_slice()
            );

            print!("Type your master password: ");
            match read_password() {
                Ok(master_password) => {
                    password::add_password(
                        master_password.as_slice(),
                        &mut password,
                        file
                    ).unwrap();

                    // read the domain name
                    println!("{}", fgcolor!(Color::Green, "Alright! Your password for {} has been added.", app_name));
                },
                Err(_) => {
                    println_stderr!("");
                    println_stderr!("{}", fgcolor!(Color::Red, "error: could not read the master password"));
                }
            }

        },
        Err(_) => {
            println_stderr!("");
            println_stderr!("{}", fgcolor!(Color::Red, "error: could not read the password"));
        }
    }
}
