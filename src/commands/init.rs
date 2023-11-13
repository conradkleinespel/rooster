use rclio::CliInputOutput;
use rclio::OutputType;
use std::path::PathBuf;

pub fn callback_exec(
    matches: &clap::ArgMatches,
    io: &mut impl CliInputOutput,
    rooster_file_path: &PathBuf,
) -> Result<(), i32> {
    let filename_as_string = rooster_file_path.to_string_lossy().into_owned();
    if rooster_file_path.exists() && !matches.get_flag("force-for-tests") {
        io.error(
            "Woops, there is already a Rooster file located at:",
            OutputType::Error,
        );
        io.error(format!("    {}", filename_as_string), OutputType::Error);
        io.nl(OutputType::Error);
        io.error(
            "Type `rooster --help` to see what Rooster can do for you.",
            OutputType::Error,
        );
        return Err(1);
    }

    io.title("Welcome to Rooster", OutputType::Standard);
    io.nl(OutputType::Standard);
    io.info("Rooster is a simple password manager. Let's get started! Type ENTER to continue.", OutputType::Standard);

    if let Err(err) = io.read_line() {
        io.error(
            format!("Woops, I didn't see the ENTER key (reason: {:?}).", err),
            OutputType::Error,
        );
        return Err(1);
    }

    io.title("The master password", OutputType::Standard);
    io.nl(OutputType::Standard);
    io.info(
        "With Rooster, you only need to remember one password: \
    the master password. It keeps all of you other passwords safe. The stronger it is, the better your passwords are \
                      protected."
    , OutputType::Standard);
    io.nl(OutputType::Standard);

    let master_password = io
        .prompt_password("Choose your master password: ")
        .map_err(|err| {
            io.error(
                format!("Woops, I couldn't read the master passwords ({:?}).", err),
                OutputType::Error,
            );
            1
        })?;

    if master_password.len() == 0 {
        io.error("Your master password cannot be empty.", OutputType::Error);
        return Err(1);
    }

    let store = match crate::password::v2::PasswordStore::new(master_password) {
        Ok(store) => store,
        Err(err) => {
            io.error(
                format!(
                    "Woops, I couldn't use the random number generator on your machine \
                     (reason: {:?}). Without it, I can't create a secure password file.",
                    err
                ),
                OutputType::Error,
            );
            return Err(1);
        }
    };

    let mut file = match crate::create_password_file(filename_as_string.as_str()).map_err(|_| 1) {
        Ok(file) => file,
        Err(err) => {
            io.error(
                format!(
                    "Woops, I couldn't create a new password file (reason: {:?})",
                    err
                ),
                OutputType::Error,
            );
            return Err(1);
        }
    };

    if let Err(err) = store.sync(&mut file) {
        if let Err(err) = ::std::fs::remove_file(rooster_file_path) {
            io.error(
                format!(
                    "Woops, I was able to create a new password file but couldn't save \
                     it (reason: {:?}). You may want to remove this dangling file:",
                    err
                ),
                OutputType::Error,
            );
            io.error(format!("    {}", filename_as_string), OutputType::Error);
            return Err(1);
        }
        io.error(
            format!(
                "Woops, I couldn't create a new password file (reason: {:?}).",
                err
            ),
            OutputType::Error,
        );
        return Err(1);
    }

    io.nl(OutputType::Standard);
    io.title("All done and ready to rock", OutputType::Standard);
    io.nl(OutputType::Standard);
    io.success("You passwords will be saved in:", OutputType::Standard);
    io.success(format!("    {}", filename_as_string), OutputType::Standard);
    io.nl(OutputType::Standard);
    io.info(
        "If you wish to change the location of your password file, you can set it in the \
        ROOSTER_FILE environment variable. For instance:",
        OutputType::Standard,
    );
    io.info(
        "    export ROOSTER_FILE=path/to/passwords.rooster",
        OutputType::Standard,
    );
    io.nl(OutputType::Standard);
    io.info(
        "Type `rooster --help` to see what Rooster can do for you.",
        OutputType::Standard,
    );

    Ok(())
}
