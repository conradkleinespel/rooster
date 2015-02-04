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
use std::rand::{ Rng, OsRng };
use serialize::json;
use std::old_io::fs::File;
use std::old_io::{ IoResult, SeekStyle };
use std::borrow::ToOwned;

/// The IV is 128 bits long.
///
/// This should be enough for it to be unique. Also, the password storage file
/// is normally unique, so if an attacker gets it, having access to the IV
/// doesn't help much, since there is no other data that uses the same IV to
/// compare with.
const IV_LEN: usize = 16;

/// The key is 256 bits long, which is 32 bytes.
pub const KEY_LEN: usize = 32;

#[derive(Clone, Debug, Decodable, Encodable)]
pub struct Password {
    name: String,
    domain: Option<String>,
    username: String,
    password: String,
    created_at: ffi::time_t,
    updated_at: ffi::time_t
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
}

// Create a random IV.
fn generate_random_iv() -> [u8; IV_LEN] {
    let mut iv: [u8; IV_LEN] = [0; IV_LEN];
    let mut rng = OsRng::new().unwrap();
    rng.fill_bytes(&mut iv);
    iv
}

/// Derives a 256 bits encryption key from the password.
fn generate_encryption_key(master_password: &str) -> [u8; KEY_LEN] {
    let mut key: [u8; KEY_LEN] = [0; KEY_LEN];
    let mut hash = crypto::sha2::Sha256::new();
    hash.input(master_password.as_bytes());
    hash.result(&mut key);
    key
}

/// Adds a password to the file.
pub fn add_password(master_password: &str, password: &Password, file: &mut File) -> IoResult<()> {
    // Go to the start of the file and read it.
    try!(file.seek(0, SeekStyle::SeekSet));
    let encrypted = try!(file.read_to_end());

    // If there were already some password, we'll decrypt them. Otherwise, we'll
    // start off with an empty list of passwords.
    let mut passwords: Vec<Password> = if encrypted.len() > 0 {
        // Get previous IV. It is located after the encrypted data in the file.
        let iv = &encrypted[encrypted.len() - IV_LEN ..];

        // Derive a 256 bits encryption key from the password.
        let key = generate_encryption_key(master_password);

        // Remove the IV before decoding, otherwise, we cant decrypt the data.
        let encrypted = &encrypted[.. encrypted.len() - IV_LEN];

        // Decrypt and decode the data (JSON).
        let decrypted = aes::decrypt(encrypted, &key, &iv).unwrap();
        let encoded: String = String::from_utf8(decrypted).unwrap();

        json::decode(encoded.as_slice()).unwrap()
    } else {
        Vec::new()
    };

    passwords.push(password.clone());

    let encoded_after = json::encode(&passwords).unwrap();

    // Encrypt the data.
    let key = generate_encryption_key(master_password);
    let iv = generate_random_iv();
    let mut encrypted_after = aes::encrypt(encoded_after.as_slice().as_bytes(), &key, &iv).unwrap();

    // Append the IV to the encrypted data so it can be retrieved later when
    // we want to decrypt said data.
    encrypted_after.push_all(&iv);

    // Save the data to the password file.
    try!(file.seek(0, SeekStyle::SeekSet));
    try!(file.truncate(0));
    try!(file.write_all(encrypted_after.as_slice()));
    try!(file.datasync());

    Ok(())
}

/*
fn example() {
    let message = "Hello World!".as_bytes().to_vec();
    let password = "secret";

    // Create a random initialization vector (IV). It does not need to be
    // secret. As such, it will be appended to the encrypted blob and saved in
    // clear. This will allow us to retrieve the IV when decrypting our data.
    let mut iv: [u8; 16] = [0; 16];
    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut iv);

    // Derive a 256 bits encryption key from the password.
    let mut key: [u8; 32] = [0; 32];
    let mut hash = crypto::sha2::Sha256::new();
    hash.input(password.as_bytes());
    hash.result(&mut key);

    // Encrypt the data.
    let mut encrypted_data = aes::encrypt(message.as_slice(), &key, &iv).ok().unwrap();
    encrypted_data.push_all(&iv);

    // Separate the IV from the encrypted blob.
    let iv_index = encrypted_data.len() - iv.len();
    let (encrypted_data, iv) = (
        &encrypted_data[.. iv_index],
        &encrypted_data[iv_index ..]
    );

    // Decrypt the data.
    let decrypted_data = aes::decrypt(encrypted_data, &key, &iv).ok().unwrap();

    assert!(message.as_slice() == &decrypted_data[]);

    print!("Please provide a password: ");
    match read_password() {
        Ok(password) => {
            println!("Alright, here we go: '{}'", password);
        },
        Err(_) => {
            println!("Could not read password.");
        }
    }
}
*/
