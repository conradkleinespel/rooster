// The code in this file is copied from the "rust-crypto" Git repository.

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::rutil::SafeVec;

pub fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, ()> {
    openssl::symm::encrypt(openssl::symm::Cipher::aes_256_cbc(), key, Some(iv), data)
        .map_err(|_| ())
}

pub fn decrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<SafeVec, ()> {
    openssl::symm::decrypt(openssl::symm::Cipher::aes_256_cbc(), key, Some(iv), data)
        .map_err(|_| ())
        .map(|vec| SafeVec::new(vec))
}
