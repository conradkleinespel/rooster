mod helpers;

use helpers::prelude::*;

#[test]
fn test_command_export_json() {
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
            &["rooster", "add", "-s", "Youtube", "yt@example.com"],
            input!("xxxx\nabcd\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );

    let mut output = sink();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "export", "json"],
            input!("xxxx\n"),
            output!(&mut sink(), &mut output, &mut sink()),
            &rooster_file
        )
    );
    let output_as_vecu8 = output.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("\"password\":\"abcd\""));
    assert!(output_as_string.contains("\"username\":\"yt@example.com\""));
    assert!(output_as_string.contains("\"name\":\"Youtube\""));
}

#[test]
fn test_command_export_csv() {
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
            &["rooster", "add", "-s", "Youtube", "yt@example.com"],
            input!("xxxx\nabcd\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );

    let mut output = sink();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "export", "csv"],
            input!("xxxx\n"),
            output!(&mut sink(), &mut output, &mut sink()),
            &rooster_file
        )
    );
    let output_as_vecu8 = output.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert_eq!(output_as_string, "Youtube,yt@example.com,abcd\n");
}

#[test]
fn test_command_export_1password() {
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
            &["rooster", "add", "-s", "Youtube", "yt@example.com"],
            input!("xxxx\nabcd\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );

    let mut output = sink();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "export", "1password"],
            input!("xxxx\n"),
            output!(&mut sink(), &mut output, &mut sink()),
            &rooster_file
        )
    );
    let output_as_vecu8 = output.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert_eq!(output_as_string, "Youtube,yt@example.com,abcd\n");
}
