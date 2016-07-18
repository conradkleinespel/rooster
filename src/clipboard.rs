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

use std::process::Command;
use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

pub fn copy_to_clipboard(s: &str) -> IoResult<()> {
    Command::new("rooster-clipboard").arg(s).status().and_then(|status| {
        if status.success() {
            Ok(())
        } else {
            Err(IoError::new(IoErrorKind::Other, "rooster-clipboard crashed"))
        }
    })
}

#[cfg(target_os="macos")]
pub fn paste_keys() -> String {
    "Cmd+V".to_string()
}

#[cfg(not(target_os="macos"))]
pub fn paste_keys() -> String {
    "Ctrl+V".to_string()
}
