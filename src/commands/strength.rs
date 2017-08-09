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
use password;
use ffi;
use list;
use std::io::Write;
use rpassword::prompt_password_stderr;
use zxcvbn::{zxcvbn, ZxcvbnError};

pub fn callback_help() {
    println!("Usage:");
    println!("    rooster strength -h");
    println!("    rooster strength");
    println!("    rooster strength <account type / name / website>");
    println!("");
    println!("Examples:");
    println!("    rooster strength youtube");
    println!("    rooster strength ytb");
}

pub fn check_args(matches: &getopts::Matches) -> Result<(), i32> {
    if matches.free.len() < 3 {
        return Ok(());
    }
    callback_help();
    Err(1)
}

pub fn callback_exec(
    matches: &getopts::Matches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    check_args(matches)?;

    let mut prereq: Vec<&str> = Vec::new();
    if matches.free.len() > 1 {
        prereq.push(&*matches.free[1]);
    }
    match prompt_password_stderr(&*format!(
        "Enter the password you want to test the strength for: "
    )) {
        Ok(password) => {
            match zxcvbn(&*password, prereq.as_slice()) {
                Ok(result) => {
                    println!("Here are your results from zxcvbn:");
                    let crack_time_display = result.crack_times_display;
                    println!("Various Crack Times [MA = Multiple Attackers, UU = User-unique Hashing, F/S = Fast / Slow hash functions");
                    println!("  Crack Time (Online attack, rate-limits) {}", crack_time_display.online_throttling_100_per_hour);
                    println!("  Crack Time (Online attack, No rate-limits) {}", crack_time_display.online_no_throttling_10_per_second);
                    println!("  Crack Time (Offline attack, MA UU, S) {}", crack_time_display.offline_slow_hashing_1e4_per_second);
                    println!("  Crack Time (Offline attack, MA, UU, F) {}", crack_time_display.offline_fast_hashing_1e10_per_second);
                    println!("    Crack Time (Milliseconds): {}", result.calc_time);
                    println!("    Guesses: {}", result.guesses);
                    println!("    Score: {}", result.score);
                    if result.feedback.is_some() {
                        let feedback = result.feedback.unwrap();
                        if !feedback.suggestions.is_empty() || feedback.warning.is_some() {
                            println!("Feedback from zxcvbn:");
                            if feedback.warning.is_some() {
                                println!("    {}Warning!{}: {}", ::color::Color::Red.to_color_code(), ::color::Color::Reset.to_color_code(), feedback.warning.unwrap());
                            }   
                            if !feedback.suggestions.is_empty() {
                                println!("Suggestions:");
                                for suggestion in feedback.suggestions {
                                    println!("    {}", suggestion);
                                }
                            }
                        }
                    }
                    Ok(())
                }
                Err(x) => {
                    match x {
                        ZxcvbnError::BlankPassword => {
                            println_err!("\nSorry, but your password was bank!");
                            Err(1)
                        }
                        ZxcvbnError::NonAsciiPassword => {
                            println_err!(
                                "\nSorry, but your password uses another format that is not ASCII!!"
                            );
                            Err(1)
                        }
                    }
                }
            }
        }
        Err(err) => {
            println_err!("\nI couldn't read the app's password (reason: {:?}).", err);
            Err(1)
        }
    }
}
