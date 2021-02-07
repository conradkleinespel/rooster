extern crate serde_json;

mod helpers;

use crate::helpers::prelude::*;
use serde_json::Value;

#[test]
fn test_command_export_json() {
    let rooster_file = tempfile();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "init", "--force-for-tests"],
            &mut CursorInputOutput::new("", "\nxxxx\n"),
            &rooster_file
        )
    );

    assert_eq!(
        0,
        main_with_args(
            &["rooster", "add", "-s", "Youtube", "yt@example.com"],
            &mut CursorInputOutput::new("", "xxxx\nabcd\n"),
            &rooster_file
        )
    );

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "export", "json"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    let output_as_json = serde_json::from_str::<Value>(output_as_string.as_ref()).unwrap();
    let saved_password = output_as_json
        .as_object()
        .unwrap()
        .get("passwords")
        .unwrap()
        .as_array()
        .unwrap()
        .get(0)
        .unwrap()
        .as_object()
        .unwrap();
    assert_eq!(
        saved_password.get("password").unwrap().as_str().unwrap(),
        "abcd"
    );
    assert_eq!(
        saved_password.get("username").unwrap().as_str().unwrap(),
        "yt@example.com"
    );
    assert_eq!(
        saved_password.get("name").unwrap().as_str().unwrap(),
        "Youtube"
    );
}

#[test]
fn test_command_export_csv() {
    let rooster_file = tempfile();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "init", "--force-for-tests"],
            &mut CursorInputOutput::new("", "\nxxxx\n"),
            &rooster_file
        )
    );

    assert_eq!(
        0,
        main_with_args(
            &["rooster", "add", "-s", "Youtube", "yt@example.com"],
            &mut CursorInputOutput::new("", "xxxx\nabcd\n"),
            &rooster_file
        )
    );

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "export", "csv"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert_eq!(output_as_string, "Youtube,yt@example.com,abcd\n");
}

#[test]
fn test_command_export_1password() {
    let rooster_file = tempfile();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "init", "--force-for-tests"],
            &mut CursorInputOutput::new("", "\nxxxx\n"),
            &rooster_file
        )
    );

    assert_eq!(
        0,
        main_with_args(
            &["rooster", "add", "-s", "Youtube", "yt@example.com"],
            &mut CursorInputOutput::new("", "xxxx\nabcd\n"),
            &rooster_file
        )
    );

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "export", "1password"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert_eq!(output_as_string, "Youtube,yt@example.com,abcd\n");
}
