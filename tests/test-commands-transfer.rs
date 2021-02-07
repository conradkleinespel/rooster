mod helpers;

use crate::helpers::prelude::*;

#[test]
fn test_command_transfer() {
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
            &["rooster", "transfer", "youtube", "videos@example.com"],
            &mut CursorInputOutput::new("", "xxxx\n"),
            &rooster_file
        )
    );

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "list"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("Youtube"));
    assert!(!output_as_string.contains("yt@example.com"));
    assert!(output_as_string.contains("videos@example.com"));
}
