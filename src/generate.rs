use rand::{rngs::OsRng, Rng};
use rclio::CliInputOutput;
use rclio::OutputType;
use rtoolbox::safe_string::SafeString;
use std::io::Result as IoResult;

fn generate_password(alnum: bool, len: usize) -> IoResult<SafeString> {
    let mut password_as_string = String::new();
    let mut rng = OsRng::default();
    for _ in 0..len {
        if alnum {
            match rng.gen_range(0..3) {
                // Numbers 0-9
                0 => password_as_string.push(rng.gen_range(48..58) as u8 as char),
                // Uppercase A-Z
                1 => password_as_string.push(rng.gen_range(65..91) as u8 as char),
                // Lowercase a-z
                2 => password_as_string.push(rng.gen_range(97..123) as u8 as char),
                _ => unreachable!(),
            }
        } else {
            password_as_string.push(rng.gen_range(33..127) as u8 as char);
        }
    }
    Ok(SafeString::from_string(password_as_string))
}

/// Returns true if the password contains at least one digit, one uppercase letter and one
/// lowercase letter.
fn password_is_hard(password: &str, alnum: bool) -> bool {
    let is_punctuation = |c| -> bool { "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".find(c).is_some() };

    password.find(char::is_numeric).is_some()
        && password.find(char::is_lowercase).is_some()
        && password.find(char::is_uppercase).is_some()
        && (alnum || password.find(is_punctuation).is_some())
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

pub fn check_password_len(opt: Option<usize>, io: &mut impl CliInputOutput) -> Option<usize> {
    match opt {
        Some(len) => {
            // We want passwords to contain at least one uppercase letter, one lowercase
            // letter and one digit. So we need at least 4 characters for each password.
            // This checks makes sure we don't run into an infinite loop trying to generate
            // a password of length < 4 with 4 different kinds of characters (uppercase,
            // lowercase, numeric, punctuation).
            if len < 4 {
                io.error("Woops! The length of the password must be at least 4. This allows us to make sure your password is secure.", OutputType::Error);
                None
            } else {
                Some(len)
            }
        }
        None => {
            io.error(
                "Woops! The length option must be a valid number, for instance 8 or 16.",
                OutputType::Error,
            );
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::generate::PasswordSpec;
    use std::ops::Deref;

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
