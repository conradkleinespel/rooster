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

use super::ffi;
use super::crypto;
use super::crypto::digest::Digest;
use super::aes;
use super::rand::{ Rng, OsRng };
use rustc_serialize::json;
use std::old_io::fs::File;
use std::old_io::{ SeekStyle };
use std::borrow::ToOwned;
use std::slice::bytes::MutableByteVector;

/// The IV is 128 bits long.
///
/// This should be enough for it to be unique. Also, the password storage file
/// is normally unique, so if an attacker gets it, having access to the IV
/// doesn't help much, since there is no other data that uses the same IV to
/// compare with.
const IV_LEN: usize = 16;

/// The key is 256 bits long, which is 32 bytes.
pub const KEY_LEN: usize = 32;

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

impl ScrubMemory for String {
    fn scrub_memory(&mut self) {
        unsafe { self.as_mut_vec() }.set_memory(0);
        self.clear();
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
        for p in self.as_mut_slice().iter_mut() {
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
}

fn get_all_passwords(master_password: &String, file: &mut File) -> Result<Vec<Password>, PasswordError> {
    // Go to the start of the file and read it.
    let encrypted = match file.seek(0, SeekStyle::SeekSet).and_then(|_| { file.read_to_end() }) {
        Ok(val) => { val },
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
        let decrypted_maybe = aes::decrypt(encrypted, key.as_slice(), &iv);
        key.scrub_memory();
        match decrypted_maybe {
            Ok(decrypted) => {
                let mut encoded = String::from_utf8_lossy(decrypted.as_slice()).into_owned();

                // This should never fail. The file contents should always be
                // valid JSON.
                let passwords = json::decode(encoded.as_slice()).unwrap();

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

fn save_all_passwords(master_password: &String, passwords: &Vec<Password>, file: &mut File) -> Result<(), PasswordError> {
    // This should never fail. The structs are all encodable.
    let mut encoded_after = json::encode(passwords).unwrap();

    // Encrypt the data.
    let mut key = generate_encryption_key(master_password);
    let iv = generate_random_iv();
    let encrypted_after_maybe = aes::encrypt(encoded_after.as_slice().as_bytes(), key.as_slice(), &iv);

    // Clear the memory so no other program can see it once freed.
    key.scrub_memory();
    encoded_after.scrub_memory();

    let mut encrypted_after = match encrypted_after_maybe {
        Ok(val) => { val },
        Err(_) => { return Err(PasswordError::EncryptionError) }
    };

    // Append the IV to the encrypted data so it can be retrieved later when
    // we want to decrypt said data.
    encrypted_after.push_all(&iv);

    // Save the data to the password file.
    let sync = file.seek(0, SeekStyle::SeekSet)
        .and_then(|_| { file.truncate(0) })
        .and_then(|_| { file.write_all(encrypted_after.as_slice()) })
        .and_then(|_| { file.datasync() });

    match sync {
        Ok(_) => { Ok(()) },
        Err(_) => { Err(PasswordError::SyncError) }
    }
}

/// Adds a password to the file.
pub fn add_password(master_password: &String, password: &Password, file: &mut File) -> Result<(), PasswordError> {
    let mut passwords = try!(get_all_passwords(master_password, file));

    passwords.push(password.clone());

    let saved = save_all_passwords(master_password, &passwords, file);

    // Clear the memory so no other program can see it once freed.
    passwords.scrub_memory();

    saved
}

pub fn delete_password(master_password: &String, id: usize, file: &mut File)  -> Result<(), PasswordError> {
    let mut passwords = try!(get_all_passwords(master_password, file));

    // Clear the memory so no other program can see it once freed.
    passwords.scrub_memory();

    Ok(())
}

pub fn get_passwords(master_password: &String, app_name: &String, file: &mut File)  -> Result<Vec<Password>, PasswordError> {
    get_all_passwords(master_password, file)
}
