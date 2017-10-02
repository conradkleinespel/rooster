extern crate conv;

#[macro_use] mod util;

use conv::*;

use conv::PosOverflow as Of;
use conv::Unrepresentable as Ur;

macro_rules! check {
    (@ $from:ty, $to:ty=> $(;)*) => {};

    (@ $from:ty, $to:ty=> try cident; $($tail:tt)*) => {
        check!(@ $from, $to=> try v: '\x00';);
        check!(@ $from, $to=> try v: '\x01';);
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> try uident; $($tail:tt)*) => {
        check!(@ $from, $to=> try v: 0;);
        check!(@ $from, $to=> try v: 1;);
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> try v: $src:expr, !$dst:expr; $($tail:tt)*) => {
        {
            let src: $from = $src;
            let dst: Result<$to, _> = src.try_into();
            assert_eq!(dst, Err($dst(src)));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> try v: $src:expr; $($tail:tt)*) => {
        {
            let src: $from = $src;
            let dst: Result<$to, _> = src.try_into();
            assert_eq!(dst, Ok($src as $to));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qt: *; $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, _> = v.try_into();
                dst == Ok(v as $to)
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    ($from:ty, $to:ty=> $($tail:tt)*) => {
        check! { @ $from, $to=> $($tail)*; }
    };
}

#[test]
fn test_i_to_c() {
    check!(u8, char => try uident; qt: *);

    /*
    `char` is a pain because `u8` is the *only* type you can cast directly from.  So, the `check!` macro is *basically useless*.

    Also, `char` has a great big hole in the middle, which makes things more interesting.

    Instead, we're just going to make sure that the conversions *exist* and have the expected error type.
    */
    macro_rules! check_i_to_c {
        ($($ts:ty),* $(,)*) => {
            $(
                {
                    let v: $ts = 0;
                    let r: Result<char, Ur<$ts>> = TryFrom::try_from(v);
                    assert_eq!(r, Ok('\x00'));
                }
            )*
        };
    }
    check_i_to_c!(i8, i16, i32, i64, isize, u16, u32, u64, usize);
}

#[test]
fn test_c_to_i() {
    check!(char, i8=> try cident;
        try v: '\u{80}', !Of;
    );
    check!(char, i16=> try cident;
        try v: '\u{8000}', !Of;
    );
    check!(char, i32=> try cident;);
    check!(char, i64=> try cident;);
    check!(char, u8=> try cident;
        try v: '\u{100}', !Of;
    );
    check!(char, u16=> try cident;
        try v: '\u{10000}', !Of;
    );
    check!(char, u32=> try cident;);
    check!(char, u64=> try cident;);
    for_bitness! {
        32 {
            check!(char, isize=> try cident;
                try v: '\u{10ffff}';
            );
            check!(char, usize=> try cident;);
        }
        64 {
            check!(char, i64=> try cident;);
            check!(char, u64=> try cident;);
        }
    }
}
