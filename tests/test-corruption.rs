mod helpers;

use crate::helpers::prelude::*;
use std::fs::File;
use std::io::Write;

#[test]
fn test_corruption() {
    let rooster_file = tempfile();

    // Creates corrupted file
    //
    // Corrupting a file can be done with:
    //     bbe -e 'r 77 X' original.rooster > corrupted.rooster
    // where X is a different char than the one that was at index 77 (the signature start)
    File::create(rooster_file.clone())
        .unwrap()
        .write_all(&[
            0o000, 0o000, 0o000, 0o002, 0o014, 0o000, 0o000, 0o000, 0o010, 0o000, 0o000, 0o000,
            0o001, 0o106, 0o147, 0o156, 0o171, 0o125, 0o131, 0o203, 0o076, 0o153, 0o235, 0o076,
            0o010, 0o323, 0o004, 0o356, 0o144, 0o264, 0o115, 0o336, 0o243, 0o114, 0o055, 0o223,
            0o045, 0o054, 0o146, 0o247, 0o204, 0o167, 0o354, 0o026, 0o171, 0o356, 0o052, 0o316,
            0o314, 0o013, 0o021, 0o302, 0o034, 0o362, 0o364, 0o151, 0o170, 0o057, 0o030, 0o123,
            0o262, 0o327, 0o054, 0o202, 0o327, 0o210, 0o007, 0o036, 0o044, 0o347, 0o250, 0o271,
            0o325, 0o144, 0o262, 0o115, 0o125, 0o141, 0o344, 0o277, 0o364, 0o352, 0o111, 0o037,
            0o223, 0o377, 0o272, 0o120, 0o365, 0o234, 0o174, 0o241, 0o116, 0o144, 0o062, 0o253,
            0o070, 0o377, 0o171, 0o175, 0o021, 0o314, 0o225, 0o164, 0o063, 0o166, 0o343, 0o075,
            0o363, 0o125, 0o307, 0o271, 0o134, 0o152, 0o353, 0o116, 0o130, 0o150, 0o005, 0o007,
            0o353, 0o102, 0o361, 0o205, 0o207, 0o175, 0o247, 0o277, 0o072, 0o323, 0o143, 0o236,
            0o171, 0o152, 0o360, 0o004, 0o120, 0o315, 0o143, 0o066, 0o311, 0o046, 0o011, 0o377,
            0o101, 0o231, 0o221, 0o214, 0o135, 0o350, 0o176, 0o062, 0o045, 0o166, 0o160, 0o167,
            0o237,
        ])
        .unwrap();

    let mut io = CursorInputOutput::new("", "xxxx\n");
    assert_eq!(
        1,
        main_with_args(&["rooster", "list"], &mut io, &rooster_file)
    );
    let output_as_vecu8 = io.stderr_cursor.into_inner();
    let output_as_string = String::from_utf8_lossy(output_as_vecu8.as_slice());
    assert!(output_as_string.contains("Your Rooster file is corrupted"));
}
