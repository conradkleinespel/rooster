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

use rand::{Rng, rngs::OsRng};
use std::io::{Result as IoResult};
use safe_string::SafeString;
use macros::show_error;

fn generate_password(alnum: bool, len: usize) -> IoResult<SafeString> {
    let mut password_as_string = String::new();
    let mut rng = OsRng::new()?;
    for _ in 0..len {
        if alnum {
            match rng.gen_range(0, 3) {
                // Numbers 0-9
                0 => password_as_string.push(rng.gen_range(48, 58) as u8 as char),
                // Uppercase A-Z
                1 => password_as_string.push(rng.gen_range(65, 91) as u8 as char),
                // Lowercase a-z
                2 => password_as_string.push(rng.gen_range(97, 123) as u8 as char),
                _ => unreachable!(),
            }
        } else {
            password_as_string.push(rng.gen_range(33, 127) as u8 as char);
        }
    }
    Ok(SafeString::new(password_as_string))
}

/// Returns true if the password contains at least one digit, one uppercase letter and one
/// lowercase letter.
fn password_is_hard(password: &str, alnum: bool) -> bool {
    let is_punctuation = |c| -> bool { "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".find(c).is_some() };

    password.find(char::is_numeric).is_some() && password.find(char::is_lowercase).is_some() &&
        password.find(char::is_uppercase).is_some() &&
        (alnum || password.find(is_punctuation).is_some())
}

pub struct PasswordSpec {
    pub alnum: bool,
    pub len: usize,
}

impl PasswordSpec {
    pub fn new(alnum: bool, password_len: Option<usize>) -> PasswordSpec {
        PasswordSpec {
            alnum: alnum,
            len: password_len.unwrap_or(32),
        }
    }

    pub fn generate_hard_password(&self) -> IoResult<SafeString> {
        loop {
            let password = generate_password(self.alnum, self.len)?;
            if password_is_hard(password.as_ref(), self.alnum) {
                return Ok(password);
            }
        }
    }
}

pub fn check_password_len(opt: Option<usize>) -> Option<usize> {
    match opt {
        Some(len) => {
            // We want passwords to contain at least one uppercase letter, one lowercase
            // letter and one digit. So we need at least 4 characters for each password.
            // This checks makes sure we don't run into an infinite loop trying to generate
            // a password of length < 4 with 4 different kinds of characters (uppercase,
            // lowercase, numeric, punctuation).
            if len < 4 {
                show_error("Woops! The length of the password must be at least 4. This");
                show_error("allows us to make sure your password is secure.");
                None
            } else {
                Some(len)
            }
        }
        None => {
            show_error("Woops! The length option must be a valid number, for instance 8 or 16.");
            None
        }
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use generate::PasswordSpec;

    #[test]
    fn test_default_password_size_is_32() {
        assert_eq!(
            PasswordSpec::new(false, None)
                .generate_hard_password()
                .unwrap()
                .len(),
            32
        );
        assert_eq!(
            PasswordSpec::new(false, Some(16))
                .generate_hard_password()
                .unwrap()
                .len(),
            16
        );
    }

    #[test]
    fn test_generate_password_alnum() {
        // All alnum
        let ps = PasswordSpec::new(true, None);
        let pw = ps.generate_hard_password().unwrap();
        for c in pw.deref().chars() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {}
                _ => panic!(),
            }
        }

        // At least one not alnum
        let ps = PasswordSpec::new(false, None);
        let pw = ps.generate_hard_password().unwrap();
        let mut ok = false;
        for c in pw.deref().chars() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {}
                _ => ok = true,
            }
        }
        assert!(ok);
    }
}
