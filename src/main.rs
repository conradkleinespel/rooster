use rclio::RegularInputOutput;
use std::env::VarError;
use std::path::PathBuf;

const ROOSTER_FILE_ENV_VAR: &'static str = "ROOSTER_FILE";
const ROOSTER_FILE_DEFAULT: &'static str = ".passwords.rooster";

fn get_password_file_path() -> Result<PathBuf, i32> {
    // First, look for the ROOSTER_FILE environment variable.
    match std::env::var(ROOSTER_FILE_ENV_VAR) {
        Ok(filename) => Ok(PathBuf::from(filename)),
        Err(VarError::NotPresent) => {
            // If the environment variable is not there, we'll look in the default location:
            // ~/.passwords.rooster
            let mut file_default = PathBuf::from(
                dirs::home_dir()
                    .ok_or(1)?
                    .as_os_str()
                    .to_os_string()
                    .into_string()
                    .map_err(|_| 1)?,
            );
            file_default.push(ROOSTER_FILE_DEFAULT);
            Ok(file_default)
        }
        Err(VarError::NotUnicode(_)) => Err(1),
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let args_refs = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    let rooster_file_path = get_password_file_path().unwrap_or_else(|err| std::process::exit(err));

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();

    std::process::exit(rooster::main_with_args(
        args_refs.as_slice(),
        &mut RegularInputOutput::new(
            stdin.lock(),
            stdout.lock(),
            stderr.lock(),
            false,
        ),
        &rooster_file_path,
    ));
}
