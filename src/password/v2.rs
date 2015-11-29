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
use super::super::aes;
use super::super::rand::{Rng, OsRng};
use super::super::byteorder::{WriteBytesExt, BigEndian, Error as ByteorderError};
use super::super::rustc_serialize::json;
use super::PasswordError;
use std::ops::Deref;
use std::mem;
use std::io::{Seek, SeekFrom, Error as IoError, ErrorKind as IoErrorKind, Read, Write};
use std::borrow::ToOwned;
use std::fs::File;
use std::ops::Drop;
#[cfg(test)]
use std::fs::OpenOptions;

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

/// Length of the key derived ffrom the user password.
const KEY_LEN: usize = 32;

/// Length of the salt passed to the key derivation function.
const SALT_LEN: usize = 32;

/// Scrypt parameters. Set in November 2015.
const SCRYPT_PARAM_LOG2_N: u8 = 12;
const SCRYPT_PARAM_R: u32 = 8;
const SCRYPT_PARAM_P: u32 = 1;

/// The version of this lib.
const VERSION: u32 = 2;

/// The Rooster file format
///
/// rooster version (unsigned int, big endian): 32 bits
/// salt: 256 bits
/// iv: 256 bits
/// encrypted blob: variable length
#[derive(Debug)]
pub struct RoosterFile {
    version: u32,
	salt: [u8; SALT_LEN],
    iv: [u8; IV_LEN],
    blob: Vec<u8>,
}

impl RoosterFile {
    pub fn from_file<F: Read + Seek>(file: &mut F) -> Result<Option<RoosterFile>, PasswordError> {
        let mut enc: Vec<u8> = Vec::new();
        try!(file.seek(SeekFrom::Start(0)).and_then(|_| file.read_to_end(&mut enc)).map_err(|err| PasswordError::Io(err)));

		if enc.len() == 0 {
			return Ok(None);
		}

        // Version taken from network byte order (big endian).
        let version = 2u32.pow(3) * (enc[0] as u32) + 2u32.pow(2) * (enc[1] as u32) + 2u32.pow(1) * (enc[2] as u32) + 2u32.pow(0) * (enc[3] as u32);

		// Copy the salt into a fixed size buffer.
        let salt_orig = &enc[mem::size_of::<u32>() .. mem::size_of::<u32>() + SALT_LEN];
        let mut salt: [u8; SALT_LEN] = [0u8; SALT_LEN];
        for (i, byte) in salt_orig.iter().enumerate() {
            salt[i] = *byte;
        }

        // Copy the IV into a fixed size buffer.
        let iv_orig = &enc[mem::size_of::<u32>() + SALT_LEN .. mem::size_of::<u32>() + SALT_LEN + IV_LEN];
        let mut iv: [u8; IV_LEN] = [0u8; IV_LEN];
        for (i, byte) in iv_orig.iter().enumerate() {
            iv[i] = *byte;
        }

        // The encrypted password data.
        let blob = enc[mem::size_of::<u32>() + SALT_LEN + IV_LEN ..].to_owned();

        Ok(Some(RoosterFile {
            version: version,
			salt: salt,
            iv: iv,
            blob: blob,
        }))
    }

    pub fn new(version: u32, salt: [u8; SALT_LEN], iv: [u8; IV_LEN], blob: Vec<u8>) -> RoosterFile {
        RoosterFile {
            version: version,
			salt: salt,
            iv: iv,
            blob: blob,
        }
    }

    pub fn to_file(self, file: &mut File) -> Result<(), PasswordError> {
        try!(file.seek(SeekFrom::Start(0)).and_then(|_| file.set_len(0)).map_err(|err| PasswordError::Io(err)));
        try!(match file.write_u32::<BigEndian>(self.version) {
            Ok(_) => Ok(()),
            Err(err) => {
                match err {
                    ByteorderError::Io(err) => Err(PasswordError::Io(err)),
                    _ => Err(PasswordError::Io(IoError::new(IoErrorKind::Other, "unknown")))
                }
            }
        });
        try!(file.write_all(&self.salt).map_err(|err| PasswordError::Io(err)));
        try!(file.write_all(&self.iv).map_err(|err| PasswordError::Io(err)));
        try!(file.write_all(&self.blob.as_ref()).map_err(|err| PasswordError::Io(err)));
        try!(file.sync_all().map_err(|err| PasswordError::Io(err)));

        Ok(())
    }
}

/// The format of the encrypted JSON content in the password file v1.
#[derive(RustcDecodable, RustcEncodable)]
pub struct Schema {
    passwords: Vec<Password>,
}

impl Schema {
    fn new(passwords: Vec<Password>) -> Schema {
        Schema {
            passwords: passwords,
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

impl Password {
    pub fn new(name: String, username: String, password: String) -> Password {
        let timestamp = ffi::time();
        Password {
            name: name,
            domain: None,
            username: username,
            password: password,
            created_at: timestamp,
            updated_at: timestamp
        }
    }
}

impl Drop for Password {
    fn drop(&mut self) {
        self.password.clear();
        for _ in 0 .. self.password.capacity() {
            self.password.push('0');
        }
    }
}

struct PasswordStore {
    master_password: String,
}

impl Drop for PasswordStore {
    fn drop(&mut self) {
        self.master_password.clear();
        for _ in 0 .. self.master_password.capacity() {
            self.master_password.push('0');
        }
    }
}

// Create a random IV.
fn generate_random_iv() -> [u8; IV_LEN] {
    let mut bytes: [u8; IV_LEN] = [0; IV_LEN];
    let mut rng = OsRng::new().unwrap();
    rng.fill_bytes(&mut bytes);
    bytes
}

// Create a random salt.
fn generate_random_salt() -> [u8; SALT_LEN] {
    let mut bytes: [u8; SALT_LEN] = [0; SALT_LEN];
    let mut rng = OsRng::new().unwrap();
    rng.fill_bytes(&mut bytes);
    bytes
}

/// Derives a 256 bits encryption key from the password.
fn generate_encryption_key(master_password: &str, salt: [u8; SALT_LEN]) -> [u8; KEY_LEN] {
    let scrypt_params = crypto::scrypt::ScryptParams::new(
        SCRYPT_PARAM_LOG2_N,
        SCRYPT_PARAM_R,
        SCRYPT_PARAM_P
    );

	let mut output: [u8; KEY_LEN] = [0; KEY_LEN];

    crypto::scrypt::scrypt(master_password.as_bytes(), &salt, &scrypt_params, &mut output);

	output
}

pub fn get_all_passwords<F: Read + Seek>(master_password: &str, file: &mut F) -> Result<Vec<Password>, PasswordError> {
	// An empty password file means there are no passwords, but its still valid.
    let rf = match try!(RoosterFile::from_file(file)) {
		Some(rf) => rf,
		None => {
			return Ok(Vec::new());
		}
	};

    // Derive a 256 bits encryption key from the password.
    let mut key = generate_encryption_key(master_password, rf.salt);

    // Decrypt the data and remvoe the descryption key from memory.
    let decrypted_maybe = aes::decrypt(rf.blob.deref(), key.as_ref(), &rf.iv);

	// We don't need the decryption key anymore.
    key.scrub_memory();

    match decrypted_maybe {
        Ok(decrypted) => {
            let mut encoded = String::from_utf8_lossy(decrypted.as_ref()).into_owned();

            // This should never fail. The file contents should always be valid JSON.
            let passwords = json::decode::<Schema>(encoded.as_ref()).unwrap().passwords;

			// We don't need the JSON anymore since we have the struct.
            encoded.scrub_memory();

            Ok(passwords)
        },
        Err(_) => {
            Err(PasswordError::DecryptionError)
        }
    }
}

fn save_all_passwords(master_password: &str, passwords: &Vec<Password>, file: &mut File) -> Result<(), PasswordError> {
    // This should never fail. The structs are all encodable.
    let mut schema = Schema::new(passwords.clone());
    let mut encoded_after = json::encode(&schema).unwrap();
    schema.scrub_memory();

    // Encrypt the data with a new salt and a new IV.
	let salt = generate_random_salt();
    let iv = generate_random_iv();
    let mut key = generate_encryption_key(master_password, salt);
    let encrypted_maybe = aes::encrypt(encoded_after.as_bytes(), key.as_ref(), &iv);

    // Clear the memory so no other program can see it once freed.
    key.scrub_memory();
    encoded_after.scrub_memory();

    let encrypted = match encrypted_maybe {
        Ok(val) => { val },
        Err(_) => { return Err(PasswordError::EncryptionError) }
    };

    // Save the data to the password file.
    let rf = RoosterFile::new(VERSION, salt, iv, encrypted);
    try!(rf.to_file(file));

    Ok(())
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

pub fn get_password<F: Read + Seek>(master_password: &str, app_name: &str, file: &mut F) -> Result<Password, PasswordError> {
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

pub fn has_password<F: Read + Seek>(master_password: &str, app_name: &str, file: &mut F) -> Result<bool, PasswordError> {
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
