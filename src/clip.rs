// Copyright 2013-2017 The Rooster Developers
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

use password;
use safe_string::SafeString;
use std::ops::Deref;

// On Windows and Mac, we'll use the native solutions provided by the OS libraries
#[cfg(any(windows, macos))]
pub fn copy_to_clipboard(s: &SafeString) -> Result<(), ()> {
    use clipboard::ClipboardProvider;
    use clipboard::ClipboardContext;

    let mut context: ClipboardContext = ClipboardProvider::new().map_err(|_| ())?;
    context.set_contents(s.deref().to_owned()).map_err(|_| ())?;
    Ok(())
}

// On UNIX, the most stable way to copy to the clipboard is using one of the existing
// and battle tested tools: xsel and xclip.
#[cfg(all(unix, not(macos), not(windows))]
pub fn copy_to_clipboard(s: &SafeString) -> Result<(), ()> {
    use shell_escape;
    use std::process::Command;

    let password = SafeString::new(shell_escape::escape(s.deref().into()).into());

    match which("xsel") {
        Some(xsel) => {
            let shell = format!(
                "printf '%s' {} | {} -ib 2> /dev/null",
                password.deref(),
                xsel.to_string_lossy()
            );
            if Command::new("sh")
                .args(&["-c", shell.as_str()])
                .status()
                .map_err(|_| ())?
                .success()
            {
                Ok(())
            } else {
                Err(())
            }
        }
        None => {
            match which("xclip") {
                Some(xclip) => {
                    let shell = format!(
                        "printf '%s' {} | {} -selection clipboard 2> /dev/null",
                        password.deref(),
                        xclip.to_string_lossy()
                    );
                    if Command::new("sh")
                        .args(&["-c", shell.as_str()])
                        .status()
                        .map_err(|_| ())?
                        .success()
                    {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                None => Err(()),
            }
        }
    }
}

#[cfg(target_os = "macos")]
pub fn paste_keys() -> String {
    "Cmd+V".to_string()
}

#[cfg(not(target_os = "macos"))]
pub fn paste_keys() -> String {
    "Ctrl+V".to_string()
}

pub fn confirm_password_retrieved(show: bool, password: &password::v2::Password) {
    if show {
        println_ok!(
            "Alright! Here is your password for {}: {}",
            password.name,
            password.password.deref()
        );
    } else {
        if copy_to_clipboard(&password.password).is_err() {
            println_ok!(
                "Hmm, I tried to copy your new password to your clipboard, but \
                         something went wrong. You can see it with `rooster get '{}' --show`",
                password.name,
            );
        } else {
            println_ok!(
                "Alright! You can paste your {} password anywhere with {}.",
                password.name,
                paste_keys()
            );
        }
    }
}
