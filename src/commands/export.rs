use crate::password;
use crate::password::v2::Password;
use csv::Writer;
use rclio::CliInputOutput;
use rclio::OutputType;
use rutil::rutil::safe_string::SafeString;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Cursor;
use std::ops::Deref;

#[derive(Serialize, Deserialize)]
pub struct JsonExport {
    passwords: Vec<Password>,
}

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    let subcommand_name = matches.subcommand_name().unwrap();
    let subcommand_matches = matches.subcommand_matches(subcommand_name).unwrap();

    if subcommand_name == "json" {
        export_to_json(subcommand_matches, store, io)
    } else if subcommand_name == "csv" {
        export_to_csv(subcommand_matches, store, io)
    } else if subcommand_name == "1password" {
        export_to_csv(subcommand_matches, store, io)
    } else {
        unimplemented!("Invalid export destination")
    }
}

fn export_to_csv(
    _matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    let passwords_ref = store.get_all_passwords();
    let output_cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let mut csv_writer = Writer::from_writer(output_cursor);
    for password in passwords_ref {
        match csv_writer.write_record(&[
            &password.name,
            &password.username,
            password.password.deref().as_str(),
        ]) {
            Ok(_) => {}
            Err(_) => return Err(1),
        }
    }
    io.write(
        String::from_utf8(csv_writer.into_inner().unwrap().into_inner()).unwrap(),
        OutputType::Standard,
    );

    return Ok(());
}

fn export_to_json(
    _matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
    io: &mut impl CliInputOutput,
) -> Result<(), i32> {
    let export = JsonExport {
        passwords: store
            .get_all_passwords()
            .into_iter()
            .map(|password| password.clone())
            .collect(),
    };
    let passwords_json = match serde_json::to_string(&export) {
        Ok(passwords_json) => passwords_json,
        Err(json_err) => {
            io.error(
                format!(
                    "Woops, I could not encode the passwords into JSON (reason: {:?}).",
                    json_err
                ),
                OutputType::Error,
            );
            return Err(1);
        }
    };

    let passwords = SafeString::from_string(passwords_json);
    io.write(format!("{}", passwords.deref()), OutputType::Standard);
    return Ok(());
}
