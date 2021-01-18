mod helpers;

use helpers::prelude::*;

#[test]
fn test_password_retry_ok() {
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
            &["rooster", "list"],
            input!("nok\nnok\nxxxx\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
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
            input!("\nxxxx\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );

    let mut output = sink();
    assert_eq!(
        1,
        main_with_args(
            &["rooster", "list"],
            input!("nok\nnok\nnok\n"),
            output!(&mut output, &mut sink(), &mut sink()),
            &rooster_file
        )
    );
    let output_as_vecu8 = output.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("Decryption of your Rooster file keeps failing"));
}
