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

extern crate clipboard;
#[cfg(target_os="linux")]
extern crate unix_daemonize;

use clipboard::ClipboardProvider;

fn do_copy(data: String) -> Result<clipboard::ClipboardContext, ()> {
    let mut context = clipboard::ClipboardContext::new().map_err(|_| ())?;
    context.set_contents(data).map_err(|_| ())?;
    Ok(context)
}

fn get_data_from_args() -> String {
    std::env::args().nth(1).unwrap()
}

// On Linux, using X, we need to run the clipboard handler in a subprocess because
// the X selection needs a daemon that makes the copied text available.
#[cfg(target_os="linux")]
fn main() {
    unix_daemonize::daemonize_redirect(Some("/dev/null"),
                                       Some("/dev/null"),
                                       unix_daemonize::ChdirMode::ChdirRoot)
            .unwrap();

    let data = get_data_from_args();
    let mut context = do_copy(data.clone()).unwrap();

    // Keep the process alive as long as the data is still in the clipboard, which means no other
    // copy has been made.
    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));

        match context.get_contents() {
            // If the clipboard has been set by another program, we'll stop this
            // background process.
            Ok(data_in_clipboard) => {
                if data_in_clipboard != data {
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }
    }
}

#[cfg(not(target_os="linux"))]
fn main() {
    do_copy(get_data_from_args()).unwrap();
}
