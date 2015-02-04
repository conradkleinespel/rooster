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

extern crate libc;
extern crate getopts;
extern crate serialize;
extern crate crypto;
extern crate rpassword;

use std::slice::AsSlice;
use std::old_io::fs::File;
use std::old_io::{ FileMode, FileAccess };

mod aes;
mod commands;
mod ffi;
mod password;
mod color;

macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::old_io::stdio::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

struct Command {
    name: &'static str,
    callback: fn(&[String], &mut File) -> ()
}

static COMMANDS: &'static [Command] = &[
    Command { name: "get", callback: commands::get::callback },
    Command { name: "add", callback: commands::add::callback },
    Command { name: "del", callback: commands::del::callback },
    Command { name: "gen", callback: commands::gen::callback }
];

fn command_from_name(name: &str) -> Option<&'static Command> {
    for c in COMMANDS.iter() {
        if c.name == name {
            return Some(&c);
        }
    }
    None
}

fn execute_command(args: &[String], command: &Command) {
    let filename = "/tmp/passwords";

    let mut file_maybe = File::open_mode(
        &Path::new(filename),
        FileMode::Open,
        FileAccess::ReadWrite
    );

    match file_maybe {
        Ok(ref mut file) => {
            (command.callback)(args.as_slice(), file);
        },
        Err(_) => {
            println_stderr!("error: could not open file `{}`", filename);
            std::os::set_exit_status(3);
        }
    }

}

fn main() {
    let args = std::os::args();

    match args.as_slice().get(1) {
        Some(command_name) => {
            match command_from_name(command_name.as_slice()) {
                Some(command) => {
                    execute_command(args.as_slice(), command);
                },
                None => {
                    println_stderr!("error: unknown command: `{}`", command_name);
                    std::os::set_exit_status(2);
                }
            }
        },
        None => {
            println_stderr!("error: usage: {} <command> [options] [args]", args[0]);
            std::os::set_exit_status(1);
        }
    }
}
