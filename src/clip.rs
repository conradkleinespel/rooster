use crate::password;
use rclio::{CliInputOutput, OutputType};
use rutil::rutil::safe_string::SafeString;

use std::ops::Deref;

// On Windows and Mac, we'll use the native solutions provided by the OS libraries
#[cfg(any(windows, target_os = "macos"))]
pub fn copy_to_clipboard(s: &SafeString) -> Result<(), ()> {
    use clipboard::ClipboardContext;
    use clipboard::ClipboardProvider;

    let mut context: ClipboardContext = ClipboardProvider::new().map_err(|_| ())?;
    context.set_contents(s.deref().to_owned()).map_err(|_| ())?;
    Ok(())
}

// On UNIX, the most stable way to copy to the clipboard is using one of the existing
// and battle tested tools: xsel and xclip.
#[cfg(all(unix, not(target_os = "macos")))]
pub fn copy_to_clipboard(s: &SafeString) -> Result<(), ()> {
    use crate::quale::which;
    use crate::shell_escape;
    use std::env;
    use std::process::Command;

    let password = SafeString::from_string(shell_escape::escape(s.deref().into()).into());

    fn wayland_clipboards(password: &SafeString) -> Result<(), ()> {
        match which("wl-copy") {
            Some(wl_copy) => {
                let shell = format!(
                    "printf '%s' {} | {} 2> /dev/null",
                    password.deref(),
                    wl_copy.to_string_lossy()
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

    fn x11_clipboards(password: &SafeString) -> Result<(), ()> {
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
            None => match which("xclip") {
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
            },
        }
    }

    match env::var_os("XDG_SESSION_TYPE") {
        Some(s) if s == "wayland" => {
            let s = wayland_clipboards(&password);
            match s {
                Ok(_) => Ok(()),
                Err(_) => x11_clipboards(&password),
            }
        }
        _ => x11_clipboards(&password),
    }
}

#[cfg(target_os = "macos")]
pub fn paste_keys() -> &'static str {
    "Cmd+V"
}

#[cfg(not(target_os = "macos"))]
pub fn paste_keys() -> &'static str {
    "Ctrl+V"
}

pub fn confirm_password_retrieved(
    show: bool,
    password: &password::v2::Password,
    io: &mut impl CliInputOutput,
) {
    if show {
        io.success(
            format!("Alright! Here is your password for {}:", password.name),
            OutputType::Standard,
        );
        io.success(
            format!("Username: {}", password.username),
            OutputType::Standard,
        );
        io.success(
            format!("Password: {}", password.password.deref()),
            OutputType::Standard,
        );
    } else {
        if copy_to_clipboard(&password.password).is_err() {
            io.success(
                format!(
                    "Hmm, I tried to copy your new password to your clipboard, but \
                     something went wrong. You can see it with `rooster get '{}' --show`",
                    password.name
                ),
                OutputType::Standard,
            );
        } else {
            io.success(
                format!("Alright! Here is your password for {}:", password.name),
                OutputType::Standard,
            );
            io.success(
                format!("Username: {}", password.username),
                OutputType::Standard,
            );
            io.success(
                format!(
                    "Password: ******** (copied to clipboard, paste with {})",
                    paste_keys()
                ),
                OutputType::Standard,
            );
        }
    }
}
