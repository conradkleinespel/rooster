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

use macros::show_error;
use password::v2::{Password, PasswordStore};
use std::io::stdin;
use std::io::Write;

/// Used to indicate lists should have a number, ie: 23 Google my.account@gmail.com
pub const WITH_NUMBERS: bool = true;

/// Used to indicate lists should not have a number, ie: Google my.account@gmail.com
pub const WITHOUT_NUMBERS: bool = false;

pub enum OutputStream {
    Stdout,
    Stderr,
}

fn get_list_of_passwords(passwords: &Vec<&Password>, with_numbers: bool) -> Vec<String> {
    // Find the app name column length
    let longest_app_name = passwords.iter().fold(0, |acc, p| {
        if p.name.len() > acc {
            p.name.len()
        } else {
            acc
        }
    });

    // Find the username column length
    let longest_username = passwords.iter().fold(0, |acc, p| {
        if p.username.len() > acc {
            p.username.len()
        } else {
            acc
        }
    });

    // Find the number column length
    let i_width = ((passwords.len() as f64).log10() + 1 as f64).floor() as usize;

    let mut list = Vec::new();

    for (i, p) in passwords.iter().enumerate() {
        let s = match with_numbers {
            WITH_NUMBERS => format!(
                "{:i_width$} {:app_name_width$} {:username_width$}",
                i + 1,
                p.name,
                p.username,
                i_width = i_width,
                app_name_width = longest_app_name,
                username_width = longest_username,
            ),
            WITHOUT_NUMBERS => format!(
                "{:app_name_width$} {:username_width$}",
                p.name,
                p.username,
                app_name_width = longest_app_name,
                username_width = longest_username,
            ),
        };

        list.push(s);
    }

    list
}

pub fn print_list_of_passwords(
    passwords: &Vec<&Password>,
    with_numbers: bool,
    output_stream: OutputStream,
) {
    let list = get_list_of_passwords(passwords, with_numbers);

    for s in list {
        match output_stream {
            OutputStream::Stdout => println!("{}", s),
            OutputStream::Stderr => println!("{}", s),
        }
    }
}

fn request_password_index_from_stdin(passwords: &Vec<&Password>, prompt: &str) -> usize {
    assert!(!passwords.is_empty());

    // Read the index from the command line and convert to a number
    let mut line = String::new();
    loop {
        if passwords.len() > 1 {
            println!("{}", prompt);
            print_stderr!("Type a number from 1 to {}: ", passwords.len());
        } else if passwords.len() == 1 {
            print_stderr!("If this is the password you mean, type \"1\" and hit ENTER: ");
        }

        line.clear();
        match stdin().read_line(&mut line) {
            Ok(_) => {
                match line.trim().parse::<usize>() {
                    Ok(index) => {
                        if index == 0 || index > passwords.len() {
                            print_stderr!(
                                "I need a number between 1 and {}. Let's try again:",
                                passwords.len()
                            );
                            continue;
                        }

                        return index - 1;
                    }
                    Err(err) => {
                        print_stderr!(
                            "This isn't a valid number (reason: {}). Let's try again (1 to {}): ",
                            err,
                            passwords.len()
                        );
                        continue;
                    }
                };
            }
            Err(err) => {
                print_stderr!(
                    "I couldn't read that (reason: {}). Let's try again (1 to {}): ",
                    err,
                    passwords.len()
                );
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
    println!();
    request_password_index_from_stdin(passwords, prompt)
}

pub fn search_and_choose_password<'a>(
    store: &'a PasswordStore,
    query: &str,
    with_numbers: bool,
    prompt: &str,
) -> Option<&'a Password> {
    let passwords = store.search_passwords(query);
    if passwords.len() == 0 {
        show_error(format!("Woops, I can't find any passwords for \"{}\".", query).as_str());
        return None;
    }

    if let Some(&password) = passwords
        .iter()
        .find(|p| p.name.to_lowercase() == query.to_lowercase())
    {
        return Some(&password);
    }

    let index = choose_password_in_list(&passwords, with_numbers, prompt);
    Some(passwords[index])
}

#[cfg(test)]
mod test {
    use super::get_list_of_passwords;
    use list::{WITHOUT_NUMBERS, WITH_NUMBERS};
    use password::v2::Password;
    use safe_string::SafeString;

    // Creates a list of at least two passwords, and more if specified
    fn get_passwords(mut additional: i32) -> Vec<Password> {
        let google = Password::new(
            format!("google"),
            format!("short un"),
            SafeString::new(format!("xxxx")),
        );

        let mut list = vec![
            Password::new(
                format!("youtube.com"),
                format!("that long username"),
                SafeString::new(format!("xxxx")),
            ),
            google.clone(),
        ];

        while additional > 0 {
            list.push(google.clone());
            additional -= 1;
        }

        list
    }

    #[test]
    fn password_list_has_right_format_with_numbers() {
        // With 2 passwords (number width 1)
        let passwords = get_passwords(0);
        let list = get_list_of_passwords(&passwords.iter().collect(), WITH_NUMBERS);

        assert_eq!(
            list,
            &[
                "1 youtube.com that long username",
                "2 google      short un          ",
            ]
        );

        // Now with 10 passwords (number width 2)
        let passwords = get_passwords(8);
        let list = get_list_of_passwords(&passwords.iter().collect(), WITH_NUMBERS);

        assert_eq!(
            list,
            &[
                " 1 youtube.com that long username",
                " 2 google      short un          ",
                " 3 google      short un          ",
                " 4 google      short un          ",
                " 5 google      short un          ",
                " 6 google      short un          ",
                " 7 google      short un          ",
                " 8 google      short un          ",
                " 9 google      short un          ",
                "10 google      short un          ",
            ]
        );
    }

    #[test]
    fn password_list_has_right_format_without_numbers() {
        let passwords = get_passwords(0);
        let list = get_list_of_passwords(&passwords.iter().collect(), WITHOUT_NUMBERS);

        assert_eq!(
            list,
            &[
                "youtube.com that long username",
                "google      short un          ",
            ]
        );
    }
}
