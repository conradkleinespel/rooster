extern crate libc;
extern crate rooster;

use rooster::io::{ReaderManager, WriterManager};
use std::env::VarError;
use std::io::Write;
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

    let stdin = std::io::stdin();
    let mut input_reader = Box::new(stdin.lock());
    let mut error_writer: Box<dyn Write> = Box::new(std::io::stderr());
    let mut output_writer: Box<dyn Write> = Box::new(std::io::stdout());
    let mut instruction_writer: Box<dyn Write> = Box::new(
        std::fs::OpenOptions::new()
            .write(true)
            .open("/dev/tty")
            .unwrap(),
    );

    let mut writer = WriterManager::new(
        &mut error_writer,
        &mut output_writer,
        &mut instruction_writer,
    );
    let mut reader = ReaderManager::new(&mut input_reader, false);

    let rooster_file_path = get_password_file_path().unwrap_or_else(|err| std::process::exit(err));

    std::process::exit(rooster::main_with_args(
        args_refs.as_slice(),
        &mut reader,
        &mut writer,
        &rooster_file_path,
    ));
}
