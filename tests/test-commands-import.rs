mod helpers;

use crate::helpers::prelude::*;
use std::fs::File;
use std::io::Write;

#[test]
fn test_command_import_json() {
    let rooster_file = tempfile();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "init", "--force-for-tests"],
            &mut CursorInputOutput::new("", "\nxxxx\n"),
            &rooster_file
        )
    );

    let import_file_json = tempfile();
    File::create(import_file_json.clone()).unwrap().write_all(
        "{\"passwords\":[{\"name\":\"Youtube\",\"username\":\"yt@example.com\",\"password\":\"abcd\",\"created_at\":1605554169,\"updated_at\":1605554169}]}".as_bytes()
    ).unwrap();

    assert_eq!(
        0,
        main_with_args(
            &[
                "rooster",
                "import",
                "json",
                import_file_json.as_path().to_str().unwrap()
            ],
            &mut CursorInputOutput::new("", "xxxx\n"),
            &rooster_file
        )
    );

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "get", "-s", "youtube"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("abcd"));
    assert!(output_as_string.contains("yt@example.com"));
    assert!(output_as_string.contains("Youtube"));

    let import_file_csv = tempfile();
    File::create(import_file_csv.clone())
        .unwrap()
        .write_all("Youtube,yt@example.com,abcd".as_bytes())
        .unwrap();

    assert_eq!(
        0,
        main_with_args(
            &[
                "rooster",
                "import",
                "csv",
                import_file_csv.as_path().to_str().unwrap()
            ],
            &mut CursorInputOutput::new("", "xxxx\n"),
            &rooster_file
        )
    );

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "get", "-s", "youtube"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("abcd"));
    assert!(output_as_string.contains("yt@example.com"));
    assert!(output_as_string.contains("Youtube"));

    let import_file_1password = tempfile();
    File::create(import_file_1password.clone())
        .unwrap()
        .write_all("Note,abcd,Youtube,Login,youtube.com,yt@example.com".as_bytes())
        .unwrap();

    assert_eq!(
        0,
        main_with_args(
            &[
                "rooster",
                "import",
                "1password",
                import_file_1password.as_path().to_str().unwrap()
            ],
            &mut CursorInputOutput::new("", "xxxx\n"),
            &rooster_file
        )
    );

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "get", "-s", "youtube"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("abcd"));
    assert!(output_as_string.contains("yt@example.com"));
    assert!(output_as_string.contains("Youtube"));
}

#[test]
fn test_command_import_csv() {
    let rooster_file = tempfile();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "init", "--force-for-tests"],
            &mut CursorInputOutput::new("", "\nxxxx\n"),
            &rooster_file
        )
    );

    let import_file_csv = tempfile();
    File::create(import_file_csv.clone())
        .unwrap()
        .write_all("Youtube,yt@example.com,abcd".as_bytes())
        .unwrap();

    assert_eq!(
        0,
        main_with_args(
            &[
                "rooster",
                "import",
                "csv",
                import_file_csv.as_path().to_str().unwrap()
            ],
            &mut CursorInputOutput::new("", "xxxx\n"),
            &rooster_file
        )
    );

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "get", "-s", "youtube"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("abcd"));
    assert!(output_as_string.contains("yt@example.com"));
    assert!(output_as_string.contains("Youtube"));
}

#[test]
fn test_command_import_1password() {
    let rooster_file = tempfile();
    assert_eq!(
        0,
        main_with_args(
            &["rooster", "init", "--force-for-tests"],
            &mut CursorInputOutput::new("", "\nxxxx\n"),
            &rooster_file
        )
    );

    let import_file_1password = tempfile();
    File::create(import_file_1password.clone())
        .unwrap()
        .write_all("Note,abcd,Youtube,Login,youtube.com,yt@example.com".as_bytes())
        .unwrap();

    assert_eq!(
        0,
        main_with_args(
            &[
                "rooster",
                "import",
                "1password",
                import_file_1password.as_path().to_str().unwrap()
            ],
            &mut CursorInputOutput::new("", "xxxx\n"),
            &rooster_file
        )
    );

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        0,
        main_with_args(&["rooster", "get", "-s", "youtube"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stdout_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("abcd"));
    assert!(output_as_string.contains("yt@example.com"));
    assert!(output_as_string.contains("Youtube"));
}
