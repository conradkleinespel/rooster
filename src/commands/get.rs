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

use std::fs::File;
use std::env;
use super::super::color::Color;
use super::super::password;
use super::super::password::ScrubMemory;
use super::super::rpassword::read_password;
use std::old_io::stdio;

// POSIX fstat related stuff.
use libc::funcs::posix88::stat_::fstat;
use libc::types::os::arch::posix01::stat as stat_struct;
use libc::consts::os::posix88::S_IFIFO;
use libc::consts::os::posix88::STDOUT_FILENO;
use std::mem;

fn stdout_is_piped() -> bool {
    // We can safely use this struct uninitialized because `fstat` will
    // initialize it for us.
    let mut stat: stat_struct = unsafe { mem::uninitialized() };

    // If there is an error, we'll just say that the output is piped.
    // This should rarely, if ever, happen. And saying the output is piped is
    // the least annoying because it allows piping.
    if unsafe { fstat(STDOUT_FILENO, &mut stat) } != 0 {
        true
    } else {
        // S_IFIFO is the type "Named pipe".
        stat.st_mode & S_IFIFO == S_IFIFO
    }
}

pub fn callback(args: &[String], file: &mut File) {
    let ref app_name = args[2];

    // We print this to STDERR instead of STDOUT so that the output of the
    // command contains *only* the password. This makes it easy to pipe it
    // to something like "xclip" which would save the password in the clipboard.
    print_stderr!("Type your master password: ");
    match read_password() {
        Ok(ref mut master_password) => {
            match password::get_password(master_password, app_name, file) {
                Ok(ref mut password) => {
                    if stdout_is_piped() {
                        print!("{}", password.password);
                        stdio::flush();
                    } else {
                        println!("{}", password.password);
                    }
                    password.scrub_memory();
                },
                Err(err) => {
                    errln!("I couldn't find a password for this app ({:?}).", err);
                    env::set_exit_status(1);
                }
            }
            master_password.scrub_memory();
        },
        Err(err) => {
            errln!("\nI couldn't read the master password ({:?}).", err);
            env::set_exit_status(1);
        }
    }
}
