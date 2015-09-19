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

use super::ffi;
use super::crypto;
use super::crypto::digest::Digest;
use super::aes;
use super::rand::{ Rng, OsRng };
use rustc_serialize::json;
use std::fs::File;
use std::io::{ Seek, SeekFrom, Read, Write };
use std::borrow::ToOwned;

/// The version of the JSON content in the password file.
///
/// The "version" key MUST be provided. It is used to determine which schema
/// structure to use for encoding and decoding. This way, future versions of
/// Rooster can interoperate with older password file formats.
///
/// Version 1 example:
/// ```
/// {
///     "version": 1,
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
pub const KEY_LEN: usize = 32;

/// The format of the encrypted JSON content in the password file v1.
#[derive(RustcDecodable, RustcEncodable)]
pub struct SchemaVersion1 {
    version: usize,
    passwords: Vec<Password>
}

impl SchemaVersion1 {
    fn new(passwords: Vec<Password>) -> SchemaVersion1 {
        SchemaVersion1 {
            version: 1,
            passwords: passwords
        }
    }
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Password {
    pub name: String,
    pub domain: Option<String>,
    pub username: String,
    pub password: String,
    pub created_at: ffi::time_t,
    pub updated_at: ffi::time_t
}

pub trait ScrubMemory {
    fn scrub_memory(&mut self);
}

impl ScrubMemory for SchemaVersion1 {
    fn scrub_memory(&mut self) {
        self.passwords.scrub_memory();
    }
}

impl ScrubMemory for String {
    fn scrub_memory(&mut self) {
        self.clear();
        for _ in 0 .. self.capacity() {
            self.push('0');
        }
    }
}

impl ScrubMemory for [u8] {
    fn scrub_memory(&mut self) {
        for c in self.iter_mut() {
            *c = 0;
        }
    }
}

impl ScrubMemory for [Password] {
    fn scrub_memory(&mut self) {
        for p in self.iter_mut() {
            p.scrub_memory();
        }
    }
}

impl Password {
    pub fn new(name: &str, username: &str, password: &str) -> Password {
        let timestamp = ffi::time();
        Password {
            name: name.to_owned(),
            domain: None,
            username: username.to_owned(),
            password: password.to_owned(),
            created_at: timestamp,
            updated_at: timestamp
        }
    }

    /// Set the memory to `0` everywhere in the structure.
    ///
    /// This is used when a password is no longer needed, in which case we
    /// don't want the memory to leak out to another program that could try to
    /// see its contents.
    pub fn scrub_memory(&mut self) {
        self.name.scrub_memory();
        match self.domain {
            Some(ref mut s) => {
                s.scrub_memory();
            },
            None => {}
        }
        self.domain = None;
        self.username.scrub_memory();
        self.password.scrub_memory();
        self.created_at = 0;
        self.updated_at = 0;
    }
}

// Create a random IV.
fn generate_random_iv() -> [u8; IV_LEN] {
    let mut iv: [u8; IV_LEN] = [0; IV_LEN];
    let mut rng = OsRng::new().unwrap();
    rng.fill_bytes(&mut iv);
    iv
}

/// Derives a 256 bits encryption key from the password.
fn generate_encryption_key(master_password: &str) -> Vec<u8> {
    // Generate the key.
    let mut key: [u8; KEY_LEN] = [0; KEY_LEN];
    let mut hash = crypto::sha2::Sha256::new();
    hash.input(master_password.as_bytes());
    hash.result(&mut key);

    let out = key.to_vec();
    key.scrub_memory();

    out
}

#[derive(Debug)]
pub enum PasswordError {
    ReadError,
    DecryptionError,
    EncryptionError,
    SyncError,
    NoSuchAppError
}

pub fn get_all_passwords(master_password: &str, file: &mut File) -> Result<Vec<Password>, PasswordError> {
    // Go to the start of the file and read it.
    let mut encrypted: Vec<u8> = Vec::new();
    match file.seek(SeekFrom::Start(0)).and_then(|_| { file.read_to_end(&mut encrypted) }) {
        Ok(_) => { },
        Err(_) => { return Err(PasswordError::ReadError) }
    };

    // If there were already some password, we'll decrypt them. Otherwise, we'll
    // start off with an empty list of passwords.
    let passwords: Vec<Password> = if encrypted.len() > 0 {
        // Get previous IV. It is located after the encrypted data in the file.
        let iv = &encrypted[encrypted.len() - IV_LEN ..];

        // Derive a 256 bits encryption key from the password.
        let mut key = generate_encryption_key(master_password);

        // Remove the IV before decoding, otherwise, we cant decrypt the data.
        let encrypted = &encrypted[.. encrypted.len() - IV_LEN];

        // Decrypt the data and remvoe the descryption key from memory.
        let decrypted_maybe = aes::decrypt(encrypted, key.as_ref(), &iv);
        key.scrub_memory();
        match decrypted_maybe {
            Ok(decrypted) => {
                let mut encoded = String::from_utf8_lossy(decrypted.as_ref()).into_owned();

                // This should never fail. The file contents should always be
                // valid JSON.
                let passwords = json::decode::<SchemaVersion1>(encoded.as_ref()).unwrap().passwords;

                // Clear the memory so no other program can see it once freed.
                encoded.scrub_memory();

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

fn save_all_passwords(master_password: &str, passwords: &Vec<Password>, file: &mut File) -> Result<(), PasswordError> {
    // This should never fail. The structs are all encodable.
    let mut schema = SchemaVersion1::new(passwords.clone());
    let mut encoded_after = json::encode(&schema).unwrap();
    schema.scrub_memory();

    // Encrypt the data.
    let mut key = generate_encryption_key(master_password);
    let iv = generate_random_iv();
    let encrypted_after_maybe = aes::encrypt(encoded_after.as_bytes(), key.as_ref(), &iv);

    // Clear the memory so no other program can see it once freed.
    key.scrub_memory();
    encoded_after.scrub_memory();

    let mut encrypted_after = match encrypted_after_maybe {
        Ok(val) => { val },
        Err(_) => { return Err(PasswordError::EncryptionError) }
    };

    // Append the IV to the encrypted data so it can be retrieved later when
    // we want to decrypt said data.
    for b in &iv {
        encrypted_after.push(*b);
    }

    // Save the data to the password file.
    let sync = file.seek(SeekFrom::Start(0))
        .and_then(|_| { file.set_len(0) })
        .and_then(|_| { file.write_all(encrypted_after.as_ref()) })
        .and_then(|_| { file.sync_data() });

    match sync {
        Ok(_) => { Ok(()) },
        Err(_) => { Err(PasswordError::SyncError) }
    }
}

/// Adds a password to the file.
pub fn add_password(master_password: &str, password: &Password, file: &mut File) -> Result<(), PasswordError> {
    let mut passwords = try!(get_all_passwords(master_password, file));

    passwords.push(password.clone());

    let saved = save_all_passwords(master_password, &passwords, file);

    // Clear the memory so no other program can see it once freed.
    passwords.scrub_memory();

    saved
}

pub fn delete_password(master_password: &str, app_name: &str, file: &mut File) -> Result<(), PasswordError> {
    match get_password(master_password, app_name, file) {
        Ok(ref mut p) => {
            let mut result = Ok(());
            match get_all_passwords(master_password, file) {
                Ok(ref mut passwords) => {
                    let mut i = 0;
                    while i < passwords.len() {
                        if passwords[i].name == p.name {
                            break;
                        }
                        i += 1;
                    }
                    if i < passwords.len() {
                        passwords.remove(i);
                    } else {
                        result = Err(PasswordError::NoSuchAppError);
                    }
                    match save_all_passwords(master_password, passwords, file) {
                        Ok(_) => {},
                        Err(err) => {
                            result = Err(err);
                        }
                    }
                    passwords.scrub_memory();
                },
                Err(err) => {
                    result = Err(err)
                }
            }
            p.scrub_memory();
            result
        },
        Err(err) => Err(err)
    }
}

pub fn get_password(master_password: &str, app_name: &str, file: &mut File) -> Result<Password, PasswordError> {
    let mut passwords = try!(get_all_passwords(master_password, file));
    let mut result = Err(PasswordError::NoSuchAppError);

    'passwords_loop: for p in passwords.iter() {
        // Since the app name must be the same, we need the same length.
        if p.name.len() != app_name.len() {
            continue 'passwords_loop;
        }

        // We're looking for the exact same app name, without regard to casing.
        let mut i: usize = 0;
        while i < p.name.len() {
            let c1 = p.name.chars().nth(i).unwrap().to_lowercase().nth(0).unwrap();
            let c2 = app_name.chars().nth(i).unwrap().to_lowercase().nth(0).unwrap();
            if c1 != c2 {
                continue 'passwords_loop;
            }
            i += 1;
        }
        result = Ok(p.clone());
        break;
    }

    // Clear the memory so no other program can see it once freed.
    passwords.scrub_memory();

    result
}

pub fn has_password(master_password: &str, app_name: &str, file: &mut File) -> Result<bool, PasswordError> {
    match get_password(master_password, app_name, file) {
        Ok(ref mut password) => {
            password.scrub_memory();
            Ok(true)
        },
        Err(err) => {
            match err {
                PasswordError::NoSuchAppError => Ok(false),
                _                             => Err(err)
            }
        }
    }
}
