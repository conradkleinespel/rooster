mod helpers;

use helpers::prelude::*;

#[test]
fn test_command_set_master_password() {
    let rooster_file = tempfile();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "init", "--force-for-tests"],
            input!("\nxxxx\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );

    assert_eq!(
        0,
        main_with_args(
            &["rooster", "set-master-password"],
            input!("xxxx\nabcd\nabcd\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );

    assert_eq!(
        1,
        main_with_args(
            &["rooster", "list"],
            input!("xxxx\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "list"],
            input!("abcd\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );
}
