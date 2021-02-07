mod helpers;

use crate::helpers::prelude::*;
use std::fs::File;
use std::io::Read;

#[test]
fn test_command_set_scrypt_params() {
    let rooster_file = tempfile();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "init", "--force-for-tests"],
            &mut CursorInputOutput::new("", "\nxxxx\n"),
            &rooster_file
        )
    );

    let mut rooster_file_contents = Vec::new();
    File::open(rooster_file.as_path())
        .unwrap()
        .read_to_end(&mut rooster_file_contents)
        .unwrap();
    assert_eq!(&rooster_file_contents[4..13], &[12, 0, 0, 0, 8, 0, 0, 0, 1]);

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        1,
        main_with_args(
            &["rooster", "set-scrypt-params", "15", "4", "0"],
            &mut io,
            &rooster_file
        )
    );
    let output_as_vecu8 = io.stderr_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("must be > 0"));

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        1,
        main_with_args(
            &["rooster", "set-scrypt-params", "21", "4", "1"],
            &mut io,
            &rooster_file
        )
    );
    let output_as_vecu8 = io.stderr_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("Run with --force to force"));

    // This takes a long time to run because parameters are high.
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "set-scrypt-params", "--force", "21", "9", "2"],
            &mut CursorInputOutput::new("", "xxxx\n"),
            &rooster_file
        )
    );
    let mut rooster_file_contents = Vec::new();
    File::open(rooster_file.as_path())
        .unwrap()
        .read_to_end(&mut rooster_file_contents)
        .unwrap();
    assert_eq!(&rooster_file_contents[4..13], &[21, 0, 0, 0, 9, 0, 0, 0, 2]);
}
