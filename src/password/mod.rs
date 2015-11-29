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
    Io(IoError)
}

fn upgrade_v1_v2(master_password: &str, mut old_file: TempFile) -> Result<Vec<u8>, PasswordError> {
	println!("starting v1 to v2");

	let passwords = match v1::get_all_passwords(master_password, &mut old_file) {
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
					return Ok(old_file);
				},
				_ => {
					return Err(err);
				}
			}
		}
	};

	println!("copying passwords to new file");

	let mut new_file = try!(TempFile::new().map_err(|err| PasswordError::Io(err)));

	for p in passwords.iter() {
		let v2_password = v2::Password {
			name: p.name.clone(),
		    domain: p.domain.clone(),
		    username: p.username.clone(),
		    password: p.password.clone(),
		    created_at: p.created_at,
		    updated_at: p.updated_at,
		};
		try!(v2::add_password(master_password, &v2_password, &mut new_file));
	}

	println!("copied passwords...");

	Ok(new_file)
}

fn upgrade_v2_v3(_master_password: &str, file: TempFile) -> Result<TempFile, PasswordError> {
	// 1- check version at top of file, if >= 3, skip this
	// 2- upgrade to v3
	Ok(file)
}

pub fn upgrade(master_password: &str, old_file: &mut File) -> Result<(), PasswordError> {
	static BACKUP_ERR: &'static str = "could not backup the password file before upgrade";

	let mut file_contents: Vec<u8> = Vec::new();
    try!(
        old_file.seek(SeekFrom::Start(0))
            .and_then(|_| old_file.read_to_end(&mut file_contents))
            .map_err(|err| PasswordError::Io(err))
    );

	let upgraded = upgrade_v1_v2(master_password, file_v1)
		.and_then(|file_v2| upgrade_v2_v3(master_password, file_v2))
		.and_then(|mut file_v3| {
			file_v3.seek(SeekFrom::Start(0))
				.and_then(|_| file_v3.set_len(0))
				.and_then(|_| file_v3.write_all(file_contents.deref()))
				.map_err(|err| PasswordError::Io(err))
		});
	match upgraded {
		Ok(_) => Ok(()),
		Err(err) => {
			try!(
		        old_file.seek(SeekFrom::Start(0))
					.and_then(|_| old_file.set_len(0))
		            .and_then(|_| old_file.write_all(file_contents.deref()))
		            .map_err(|err| {
						println_err!("I tried to upgrade your Rooster file to the newest version supported by your");
						println_err!("system. But there was an error ({:?}). I was not able to recover your Rooster", err);
						println_err!("file, but I have made a backup at {}.", backup.path().to_string_lossy().deref());
						PasswordError::Io(err)
					})
		    );

			println_err!("I tried to upgrade your Rooster file to the newest version supported by your");
			println_err!("system. But there was an error ({:?}). You may want to try again.", err);
			Err(err)
		}
	}
}
