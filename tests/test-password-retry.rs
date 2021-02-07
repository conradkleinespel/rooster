mod helpers;

use crate::helpers::prelude::*;

#[test]
fn test_password_retry_ok() {
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
            &["rooster", "list"],
            &mut CursorInputOutput::new("", "nok\nnok\nxxxx\n"),
            &rooster_file
        )
    );
}

#[test]
fn test_password_retry_nok() {
    let rooster_file = tempfile();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "init", "--force-for-tests"],
            &mut CursorInputOutput::new("", "\nxxxx\n"),
            &rooster_file
        )
    );

    let mut io = CursorInputOutput::new("", "nok\nnok\nnok\n");
    assert_eq!(
        1,
        main_with_args(&["rooster", "list"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stderr_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("Decryption of your Rooster file keeps failing"));
}
