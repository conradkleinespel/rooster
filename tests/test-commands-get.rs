mod helpers;

use helpers::prelude::*;

#[test]
fn test_command_get() {
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
            &["rooster", "add", "-s", "First Website", "first@example.com"],
            input!("xxxx\nabcd\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
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
            input!("xxxx\nefgh\n"),
            output!(&mut sink(), &mut sink(), &mut sink()),
            &rooster_file
        )
    );

    // Checking fuzzy-search and password selection
    let mut output = sink();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "get", "-s", "wbst"],
            input!("xxxx\n1\n"),
            output!(&mut sink(), &mut output, &mut sink()),
            &rooster_file
        )
    );
    let output_as_vecu8 = output.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("abcd"));
    assert!(output_as_string.contains("first@example.com"));

    // Checking fuzzy-search and password selection
    let mut output = sink();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "get", "-s", "wbst"],
            input!("xxxx\n2\n"),
            output!(&mut sink(), &mut output, &mut sink()),
            &rooster_file
        )
    );
    let output_as_vecu8 = output.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("efgh"));
    assert!(output_as_string.contains("second@example.com"));
}
