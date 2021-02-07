mod helpers;

use crate::helpers::prelude::*;

#[test]
fn test_command_change() {
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
            &["rooster", "generate", "-s", "Youtube", "yt@example.com"],
            &mut CursorInputOutput::new("", "xxxx\n"),
            &rooster_file
        )
    );

    assert_eq!(
        0,
        main_with_args(
            &["rooster", "change", "-s", "youtube"],
            &mut CursorInputOutput::new("", "xxxx\nabcd\n"),
            &rooster_file
        )
    );

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "get", "-s", "youtube"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("abcd"));
    assert!(output_as_string.contains("yt@example.com"));

    // Empty passwords are disallowed
    assert_eq!(
        1,
        main_with_args(
            &["rooster", "change", "-s", "youtube"],
            &mut CursorInputOutput::new("", "xxxx\n\n"),
            &rooster_file
        )
    );
}
