mod helpers;

use crate::helpers::prelude::*;

#[test]
fn test_command_get() {
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
            &["rooster", "add", "-s", "First Website", "first@example.com"],
            &mut CursorInputOutput::new("", "xxxx\nabcd\n"),
            &rooster_file
        )
    );
    assert_eq!(
        0,
        main_with_args(
            &[
                "rooster",
                "add",
                "-s",
                "Second Website",
                "second@example.com"
            ],
            &mut CursorInputOutput::new("", "xxxx\nefgh\n"),
            &rooster_file
        )
    );

    // Checking fuzzy-search and password selection
    let mut io = CursorInputOutput::new("", "xxxx\n1\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "get", "-s", "wbst"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("abcd"));
    assert!(output_as_string.contains("first@example.com"));

    // Checking fuzzy-search and password selection
    let mut io = CursorInputOutput::new("", "xxxx\n2\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "get", "-s", "wbst"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("efgh"));
    assert!(output_as_string.contains("second@example.com"));
}
