mod helpers;

use crate::helpers::prelude::*;

#[test]
fn test_command_set_master_password() {
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
            &["rooster", "set-master-password"],
            &mut CursorInputOutput::new("", "xxxx\nabcd\nabcd\n"),
            &rooster_file
        )
    );

    assert_eq!(
        1,
        main_with_args(
            &["rooster", "list"],
            &mut CursorInputOutput::new("", "xxxx\n"),
            &rooster_file
        )
    );
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "list"],
            &mut CursorInputOutput::new("", "abcd\n"),
            &rooster_file
        )
    );
}
