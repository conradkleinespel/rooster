use crate::clip;

use crate::list;
use crate::password;
use rclio::CliInputOutput;

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    let show = matches.get_flag("show");
    let query = matches.get_one::<String>("app").unwrap();

    let prompt = format!(
        "Which password would you like {}? ",
        if show {
            "to see"
        } else {
            "to copy to your clipboard"
        },
    );
    let password =
        list::search_and_choose_password(store, query, list::WITH_NUMBERS, &prompt, io).ok_or(1)?;

    clip::confirm_password_retrieved(show, &password, io);

    Ok(())
}
