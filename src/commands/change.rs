use crate::clip;
use crate::ffi;
use crate::list;
use crate::password;
use rclio::CliInputOutput;
use rclio::OutputType;

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    let query = matches.value_of("app").unwrap();

    let password = list::search_and_choose_password(
        store,
        query,
        list::WITH_NUMBERS,
        "Which password would like to update?",
        io,
    )
    .ok_or(1)?
    .clone();

    let password_as_string = io
        .prompt_password(format!(
            "What password do you want for \"{}\"? ",
            password.name
        ))
        .map_err(|err| {
            io.error(
                format!("\nI couldn't read the app's password (reason: {:?}).", err),
                OutputType::Error,
            );
            1
        })?;

    let password = store
        .change_password(&password.name, &|old_password: password::v2::Password| {
            password::v2::Password {
                name: old_password.name,
                username: old_password.username,
                password: password_as_string.clone(),
                created_at: old_password.created_at,
                updated_at: ffi::time(),
            }
        })
        .map_err(|err| {
            io.error(
                format!(
                    "Woops, I couldn't save the new password (reason: {:?}).",
                    err
                ),
                OutputType::Error,
            );
            1
        })?;

    let show = matches.is_present("show");
    clip::confirm_password_retrieved(show, &password, io);
    Ok(())
}
