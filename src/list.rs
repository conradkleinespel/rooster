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

use io::{ReaderManager, WriterManager};
use password::v2::{Password, PasswordStore};
use std::io::{BufRead, Write};

/// Used to indicate lists should have a number, ie: 23 Google my.account@gmail.com
pub const WITH_NUMBERS: bool = true;

/// Used to indicate lists should not have a number, ie: Google my.account@gmail.com
pub const WITHOUT_NUMBERS: bool = false;

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

pub fn print_list_of_passwords<
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    passwords: &Vec<&Password>,
    with_numbers: bool,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) {
    let list = get_list_of_passwords(passwords, with_numbers);

    for s in list {
        writer.output().info(s.as_str());
    }
}

fn request_password_index_from_stdin<
    R: BufRead,
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    passwords: &Vec<&Password>,
    prompt: &str,
    reader: &mut ReaderManager<R>,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> usize {
    assert!(!passwords.is_empty());

    // Read the index from the command line and convert to a number
    loop {
        if passwords.len() > 1 {
            writer.instruction().info(prompt);
            writer
                .instruction()
                .prompt(format!("Type a number from 1 to {}: ", passwords.len()).as_str());
        } else if passwords.len() == 1 {
            writer
                .instruction()
                .prompt("If this is the password you mean, type \"1\" and hit ENTER: ");
        }

        match reader.read_line() {
            Ok(line) => {
                match line.trim().parse::<usize>() {
                    Ok(index) => {
                        if index == 0 || index > passwords.len() {
                            writer.instruction().prompt(
                                format!(
                                    "I need a number between 1 and {}. Let's try again:",
                                    passwords.len()
                                )
                                .as_str(),
                            );
                            continue;
                        }

                        return index - 1;
                    }
                    Err(err) => {
                        writer.instruction().prompt(
                            format!(
                            "This isn't a valid number (reason: {}). Let's try again (1 to {}): ",
                            err,
                            passwords.len()
                        )
                            .as_str(),
                        );
                        continue;
                    }
                };
            }
            Err(err) => {
                writer.instruction().prompt(
                    format!(
                        "I couldn't read that (reason: {}). Let's try again (1 to {}): ",
                        err,
                        passwords.len()
                    )
                    .as_str(),
                );
            }
        }
    }
}

fn choose_password_in_list<
    R: BufRead,
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    passwords: &Vec<&Password>,
    with_numbers: bool,
    prompt: &str,
    reader: &mut ReaderManager<R>,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> usize {
    print_list_of_passwords(passwords, with_numbers, writer);
    writer.output().newline();
    request_password_index_from_stdin(passwords, prompt, reader, writer)
}

pub fn search_and_choose_password<
    'a,
    'b,
    'c,
    R: BufRead,
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    store: &'c PasswordStore,
    query: &str,
    with_numbers: bool,
    prompt: &str,
    reader: &mut ReaderManager<'b, R>,
    writer: &mut WriterManager<'a, ErrorWriter, OutputWriter, InstructionWriter>,
) -> Option<&'c Password> {
    let passwords = store.search_passwords(query);
    if passwords.len() == 0 {
        writer
            .error()
            .error(format!("Woops, I can't find any passwords for \"{}\".", query).as_str());
        return None;
    }

    if let Some(&password) = passwords
        .iter()
        .find(|p| p.name.to_lowercase() == query.to_lowercase())
    {
        return Some(&password);
    }

    let index = choose_password_in_list(&passwords, with_numbers, prompt, reader, writer);
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
