// Copyright 2014-2017 The Rooster Developers
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

use io::{ReaderManager, WriterManager};
use password;
use std::io::{BufRead, Write};

pub fn callback_exec<
    R: BufRead,
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
>(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    reader: &mut ReaderManager<R>,
    writer: &mut WriterManager<ErrorWriter, OutputWriter, InstructionWriter>,
) -> Result<(), i32> {
    let log2_n = matches
        .value_of("log2n")
        .unwrap()
        .trim()
        .parse::<u8>()
        .unwrap();
    let r = matches
        .value_of("r")
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();
    let p = matches
        .value_of("p")
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();

    if log2_n > 20 || r > 8 || p > 1 {
        writer.error().error("These parameters seem very high. You might be unable to open your password file ever again. Aborting.");
        return Err(1);
    }

    store.change_scrypt_params(log2_n, r, p);

    Ok(())
}
