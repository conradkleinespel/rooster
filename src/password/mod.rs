pub mod v1;
pub mod v2;

use rutil::safe_string::SafeString;
use rutil::safe_vec::SafeVec;
use std::convert::From;
use std::io::Error as IoError;
use std::ops::Deref;

#[derive(Debug)]
pub enum PasswordError {
    DecryptionError,
    EncryptionError,
    NoSuchAppError,
    AppExistsError,
    Io(IoError),
    OutdatedRoosterBinaryError,
    InvalidJsonError,
    CorruptionError,
    CorruptionLikelyError,
    NeedUpgradeErrorFromV1,
    NoUpgradeError,
    EmptyPasswordError,
}

impl From<IoError> for PasswordError {
    fn from(err: IoError) -> PasswordError {
        PasswordError::Io(err)
    }
}

fn upgrade_v1_v2(
    v1_passwords: &[v1::Password],
    v2_store: &mut v2::PasswordStore,
) -> Result<(), PasswordError> {
    for p in v1_passwords.iter() {
        let v2_password = v2::Password {
            name: p.name.clone(),
            username: p.username.clone(),
            password: p.password.clone(),
            created_at: p.created_at,
            updated_at: p.updated_at,
        };
        v2_store.add_password(v2_password)?;
    }

    Ok(())
}

pub fn upgrade(
    master_password: SafeString,
    input: SafeVec,
) -> Result<v2::PasswordStore, PasswordError> {
    // If we can't read v1 passwords, we have a hard error, because we previously tried
    // to read the passwords as v2. Which failed. That means we can't upgrade.
    let v1_passwords = v1::get_all_passwords(master_password.deref(), input.deref())?;

    // Upgrade from v1 to v2 if we could read v1 passwords.
    let mut v2_store = v2::PasswordStore::new(master_password.clone())?;
    upgrade_v1_v2(v1_passwords.deref(), &mut v2_store)?;

    Ok(v2_store)
}
