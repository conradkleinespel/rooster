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

use std::old_io::fs::File;
use std::rand::{ Rng, OsRng };
use serialize::hex::ToHex;

const PASSWORD_LEN: usize = 32;

pub fn callback(args: &[String], file: &mut File) {
    let mut buffer: [u8; PASSWORD_LEN] = [0; PASSWORD_LEN];
    let mut rng = OsRng::new().unwrap();
    for i in 0 .. PASSWORD_LEN - 1 {
        buffer[i] = rng.gen_range(33, 126);
    }
    let password = String::from_utf8_lossy(&buffer).into_owned();
    println!("{}", password);
}
