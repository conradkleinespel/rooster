// Copyright 2014 The Rooster Developers
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

use super::super::ffi;
use super::super::crypto;
use super::super::crypto::digest::Digest;
use super::super::aes;
use super::PasswordError;
use super::super::safe_string::SafeString;
use super::super::safe_vec::SafeVec;
use rustc_serialize::json;
use std::ops::Drop;
use std::ops::DerefMut;
use std::ops::Deref;

/// The Rooster file format
///
/// encrypted blob: variable length
/// iv: 128 bits

/// The schema of the JSON content in the password file.
///
/// ```
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

/// The key is 256 bits long, which is 32 bytes.
const KEY_LEN: usize = 32;

/// The format of the encrypted JSON content in the password file v1.
#[derive(RustcDecodable, RustcEncodable)]
pub struct Schema {
    passwords: Vec<Password>
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Password {
    pub name: String,
    pub domain: Option<String>,
    pub username: String,
    pub password: SafeString,
    pub created_at: ffi::time_t,
    pub updated_at: ffi::time_t
}

/// Derives a 256 bits encryption key from the password.
fn generate_encryption_key(master_password: &str) -> SafeVec {
    // Generate the key.
    let mut key = SafeVec::new(Vec::<u8>::with_capacity(KEY_LEN));
    let mut hash = crypto::sha2::Sha256::new();
    hash.input(master_password.as_bytes());
    hash.result(key.deref_mut());

    key
}

pub fn get_all_passwords(master_password: &str, encrypted: &[u8]) -> Result<Vec<Password>, PasswordError> {
    // If there were already some password, we'll decrypt them. Otherwise, we'll
    // start off with an empty list of passwords.
    let passwords: Vec<Password> = if encrypted.len() > 0 {
        // Get previous IV. It is located after the encrypted data in the file.
        let iv = &encrypted[encrypted.len() - IV_LEN ..];

        // Derive a 256 bits encryption key from the password.
        let key = generate_encryption_key(master_password);

        // Remove the IV before decoding, otherwise, we cant decrypt the data.
        let encrypted = &encrypted[.. encrypted.len() - IV_LEN];

        // Decrypt the data and remvoe the descryption key from memory.
        let decrypted_maybe = aes::decrypt(encrypted, key.deref(), &iv);

        match decrypted_maybe {
            Ok(decrypted) => {
                let encoded = SafeString::new(String::from_utf8_lossy(decrypted.deref()).into_owned());

                // This should never fail. The file contents should always be
                // valid JSON.
                let passwords = json::decode::<Schema>(encoded.deref()).unwrap().passwords;

                passwords
            },
            Err(_) => {
                return Err(PasswordError::DecryptionError);
            }
        }
    } else {
        Vec::new()
    };

    Ok(passwords)
}
