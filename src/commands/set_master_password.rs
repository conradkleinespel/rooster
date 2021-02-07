use crate::password;
use crate::rclio::CliInputOutput;
use crate::rclio::OutputType;
use std::ops::Deref;

pub fn callback_exec(
    _matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    match io.prompt_password("Type your new master password: ") {
        Ok(master_password) => {
            let master_password_confirmation =
                match io.prompt_password("Type your new master password once more: ") {
                    Ok(master_password_confirmation) => master_password_confirmation,
                    Err(err) => {
                        io.error(
                            format!(
                                "I could not read your new master password (reason: {:?}).",
                                err
                            ),
                            OutputType::Error,
                        );
                        return Err(1);
                    }
                };

            if master_password != master_password_confirmation {
                io.error(
                    "The master password confirmation did not match. Aborting.",
                    OutputType::Error,
                );
                return Err(1);
            }

            store.change_master_password(master_password.deref());
        }
        Err(err) => {
            io.error(
                format!(
                    "I could not read your new master password (reason: {:?}).",
                    err
                ),
                OutputType::Error,
            );
            return Err(1);
        }
    }
    io.success(
        "Your master password has been changed.",
        OutputType::Standard,
    );
    Ok(())
}
