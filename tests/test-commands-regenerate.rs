mod helpers;

use helpers::prelude::*;

#[test]
fn test_command_regenerate() {
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

    // Password exists
    assert_eq!(
        1,
        main_with_args(
            &["rooster", "generate", "-s", "Youtube", "yt@example.com"],
            input!("xxxx\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );

    let mut output_1 = sink();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "get", "-s", "youtube"],
            input!("xxxx\n"),
            output!(&mut sink(), &mut output_1, &mut sink()),
            &rooster_file
        )
    );
    let output_1_as_vecu8 = output_1.into_inner();
    let output_1_as_string = String::from_utf8_lossy(output_1_as_vecu8.as_slice());

    assert_eq!(
        0,
        main_with_args(
            &["rooster", "regenerate", "-s", "Youtube"],
            input!("xxxx\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );

    let mut output_2 = sink();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "get", "-s", "youtube"],
            input!("xxxx\n"),
            output!(&mut sink(), &mut output_2, &mut sink()),
            &rooster_file
        )
    );
    let output_2_as_vecu8 = output_2.into_inner();
    let output_2_as_string = String::from_utf8_lossy(output_2_as_vecu8.as_slice());

    assert_ne!(output_1_as_string, output_2_as_string);
}
