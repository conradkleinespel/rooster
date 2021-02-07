use crate::list;
use crate::password;
use crate::rclio::CliInputOutput;
use crate::rclio::OutputType;

pub fn callback_exec(
    _matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    let passwords = store.get_all_passwords();

    if passwords.len() == 0 {
        io.info(
            "No passwords on record yet. Add one with `rooster add <app> <username>`.",
            OutputType::Standard,
        );
    } else {
        list::print_list_of_passwords(&passwords, list::WITHOUT_NUMBERS, io);
    }

    Ok(())
}
