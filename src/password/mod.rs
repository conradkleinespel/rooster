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

pub mod v1;
pub mod v2;

use std::fs::File;
use std::io::{Seek, SeekFrom, Read, Write, Result as IoResult, Error as IoError, ErrorKind as IoErrorKind};
use std::ops::Deref;

#[derive(Debug)]
pub enum PasswordError {
    DecryptionError,
    EncryptionError,
    NoSuchAppError,
    AppExistsError,
    Io(IoError),
    WrongVersionError,
}

fn upgrade_v1_v2(master_password: &str, input: &[u8], v2_store: &mut v2::PasswordStore) -> Result<(), PasswordError> {
	println!("starting v1 to v2");

	let passwords = match v1::get_all_passwords(master_password, input) {
		Ok(passwords) => passwords,
		Err(err) => {
			match err {
				// The Rooster file v1 was not explicitly versioned, so we don't know if a
				// decryption error is because there was actually an error or because the
				// file uses a higher version that the v1-upgrader does not understand.
				//
				// We let this one through, so we will either get an error on a following
				// upgrader, or an error in the command specific code, or no error if
				// everything is fine.
				PasswordError::DecryptionError => {
					return Ok(());
				},
				_ => {
					return Err(err);
				}
			}
		}
	};
    println!("{:?}", passwords);

	println!("copying passwords to new file");

	for p in passwords.iter() {
		let v2_password = v2::Password {
			name: p.name.clone(),
		    domain: p.domain.clone(),
		    username: p.username.clone(),
		    password: p.password.clone(),
		    created_at: p.created_at,
		    updated_at: p.updated_at,
		};
		try!(v2_store.add_password(&v2_password));
	}

	println!("copied passwords...");

	Ok(())
}

pub fn upgrade(master_password: &str, input: &[u8], file: &mut File) -> Result<v2::PasswordStore, PasswordError> {
    let v2_store = try!(v2::PasswordStore::from_input(master_password, input));
    try!(upgrade_v1_v2(master_password, input, v2_store));
    try!(v2_store.sync(file));
    Ok(v2_store)
}
