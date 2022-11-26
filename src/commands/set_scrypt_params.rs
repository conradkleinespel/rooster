use crate::password;
use rclio::{CliInputOutput, OutputType};

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    let log2_n = *matches.get_one::<u8>("log2n").unwrap();
    let r = *matches.get_one::<u32>("r").unwrap();
    let p = *matches.get_one::<u32>("p").unwrap();

    if log2_n <= 0 || r <= 0 || p <= 0 {
        io.error(
            format!("The parameters must be > 0 ({}, {}, {})", log2_n, r, p),
            OutputType::Error,
        );
        return Err(1);
    }

    if !matches.get_flag("force") && (log2_n > 20 || r > 8 || p > 1) {
        io.error("These parameters seem very high. You might be unable to open your password file ever again. Aborting.", OutputType::Error);
        io.error(
            "Run with --force to force, but make a backup of your password file first.",
            OutputType::Error,
        );
        return Err(1);
    }

    store.change_scrypt_params(log2_n, r, p);

    Ok(())
}
