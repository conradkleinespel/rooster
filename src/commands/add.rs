use crate::clip::{copy_to_clipboard, paste_keys};
use crate::password;
use rclio::CliInputOutput;
use rclio::OutputType;
use std::ops::Deref;

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    let app_name = matches.value_of("app").unwrap();
    let username = matches.value_of("username").unwrap();

    if store.has_password(app_name.deref()) {
        io.error(
            "Woops, there is already an app with that name.",
            OutputType::Error,
        );
        return Err(1);
    }

    match io.prompt_password(format!("What password do you want for \"{}\"? ", app_name)) {
        Ok(password_as_string) => {
            let password_as_string_clipboard = password_as_string.clone();
            let password =
                password::v2::Password::new(app_name.clone(), username, password_as_string);
            match store.add_password(password) {
                Ok(_) => {
                    if matches.is_present("show") {
                        io.success(
                            format!(
                                "Alright! Here is your password: {}",
                                password_as_string_clipboard.deref()
                            ),
                            OutputType::Standard,
                        );
                        return Ok(());
                    }

                    if copy_to_clipboard(&password_as_string_clipboard).is_err() {
                        io.success(
                            format!(
                                "Hmm, I tried to copy your new password to your clipboard, \
                                 but something went wrong. Don't worry, it's saved, and you \
                                 can see it with `rooster get {} --show`",
                                app_name
                            ),
                            OutputType::Standard,
                        );
                    } else {
                        io.success(
                            format!(
                                "Alright! I've saved your new password. You can paste it \
                                 anywhere with {}.",
                                paste_keys()
                            ),
                            OutputType::Standard,
                        );
                    }
                }
                Err(err) => {
                    io.error(
                        format!("Woops, I couldn't add the password (reason: {:?}).", err),
                        OutputType::Error,
                    );
                    return Err(1);
                }
            }
            Ok(())
        }
        Err(err) => {
            io.error(
                format!("\nI couldn't read the app's password (reason: {:?}).", err),
                OutputType::Error,
            );
            Err(1)
        }
    }
}
