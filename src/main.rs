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

#![allow(unstable)]

extern crate crypto;

use std::rand::{ Rng, OsRng };
use crypto::digest::Digest;

mod aes;

fn main() {
    let message = "Hello World!".as_bytes().to_vec();
    let password = "secret";

    // Create a random IV. It does not need to be secret. As such, it will be
    // appended to the encrypted blob and saved in clear. This will allow us
    // to retrieve the IV when decrypting our data.
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
}
