mod helpers;

use helpers::prelude::*;

#[test]
fn test_command_transfer() {
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
            &["rooster", "generate", "-s", "Youtube", "yt@example.com"],
            input!("xxxx\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );

    assert_eq!(
        0,
        main_with_args(
            &["rooster", "transfer", "youtube", "videos@example.com"],
            input!("xxxx\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );

    let mut output = sink();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "list"],
            input!("xxxx\n"),
            output!(&mut sink(), &mut output, &mut sink()),
            &rooster_file
        )
    );
    let output_as_vecu8 = output.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("Youtube"));
    assert!(!output_as_string.contains("yt@example.com"));
    assert!(output_as_string.contains("videos@example.com"));
}
