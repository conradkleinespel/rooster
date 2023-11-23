use super::PasswordError;
use crate::aes;
use crate::ffi;
use rtoolbox::safe_string::SafeString;
use rtoolbox::safe_vec::SafeVec;
use serde::{Deserialize, Serialize};
use serde_json;

use serde_json::Error;
use std::ops::Deref;
use openssl::hash::{hash, MessageDigest};

/// The Rooster file format
///
/// encrypted blob: variable length
/// iv: 128 bits

/// The schema of the JSON content in the password file.
///
/// ```json
/// {
///     "passwords": [
///         "name": "YouTube",
///         "username": "conradk",
///         "password": "xxxxxxxx",
///         "created_at": 23145436,
///         "updated_at": 23145546,
///     ]
/// }
/// ```

/// The IV is 128 bits long.
///
/// This should be enough for it to be unique. Also, the password storage file
/// is normally unique, so if an attacker gets it, having access to the IV
/// doesn't help much, since there is no other data that uses the same IV to
/// compare with.
const IV_LEN: usize = 16;

/// The format of the encrypted JSON content in the password file v1.
#[derive(Serialize, Deserialize)]
pub struct Schema {
    passwords: Vec<Password>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Password {
    pub name: String,
    pub domain: Option<String>,
    pub username: String,
    pub password: SafeString,
    pub created_at: ffi::time_t,
    pub updated_at: ffi::time_t,
}

/// Derives a 256 bits encryption key from the password.
fn generate_encryption_key(master_password: &str) -> SafeVec {
    let result = hash(
        MessageDigest::sha256(),
        master_password.as_bytes()
    ).unwrap();

    SafeVec::new(result.deref().to_vec())
}

pub fn get_all_passwords(
    master_password: &str,
    encrypted: &[u8],
) -> Result<Vec<Password>, PasswordError> {
    // If there were already some password, we'll decrypt them. Otherwise, we'll
    // start off with an empty list of passwords.
    let passwords: Vec<Password> = if encrypted.len() > 0 {
        // Get previous IV. It is located after the encrypted data in the file.
        let iv = &encrypted[encrypted.len() - IV_LEN..];

        // Derive a 256 bits encryption key from the password.
        let key = generate_encryption_key(master_password);

        // Remove the IV before decoding, otherwise, we cant decrypt the data.
        let encrypted = &encrypted[..encrypted.len() - IV_LEN];

        // Decrypt the data.
        let decrypted_maybe = aes::decrypt(encrypted, key.deref(), iv);

        match decrypted_maybe {
            Ok(decrypted) => {
                let encoded = SafeString::from_string(
                    String::from_utf8_lossy(decrypted.deref()).into_owned(),
                );

                let s: Result<Schema, Error> = serde_json::from_str(encoded.deref());

                match s {
                    Ok(schema) => schema.passwords,
                    Err(_) => {
                        return Err(PasswordError::InvalidJsonError);
                    }
                }
            }
            Err(_) => {
                return Err(PasswordError::DecryptionError);
            }
        }
    } else {
        Vec::new()
    };

    Ok(passwords)
}

#[cfg(test)]
mod test {
    use super::generate_encryption_key;
    use std::ops::Deref;

    #[test]
    fn test_generate_encryption_key_is_256_bits() {
        assert!(generate_encryption_key("test").deref().len() == (256 / 8));
    }
}
