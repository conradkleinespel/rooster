mod helpers;

use crate::helpers::prelude::*;

#[test]
fn test_command_regenerate() {
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

    // Password exists
    assert_eq!(
        1,
        main_with_args(
            &["rooster", "generate", "-s", "Youtube", "yt@example.com"],
            &mut CursorInputOutput::new("", "xxxx\n"),
            &rooster_file
        )
    );

    let mut io_1 = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "get", "-s", "youtube"],
            &mut io_1,
            &rooster_file
        )
    );
    let output_1_as_vecu8 = io_1.stdout_cursor.into_inner();
    let output_1_as_string = String::from_utf8_lossy(output_1_as_vecu8.as_slice());

    assert_eq!(
        0,
        main_with_args(
            &["rooster", "regenerate", "-s", "Youtube"],
            &mut CursorInputOutput::new("", "xxxx\n"),
            &rooster_file
        )
    );

    let mut io_2 = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "get", "-s", "youtube"],
            &mut io_2,
            &rooster_file
        )
    );
    let output_2_as_vecu8 = io_2.stdout_cursor.into_inner();
    let output_2_as_string = String::from_utf8_lossy(output_2_as_vecu8.as_slice());

    assert_ne!(output_1_as_string, output_2_as_string);
}
