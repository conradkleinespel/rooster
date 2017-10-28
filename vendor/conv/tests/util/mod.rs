macro_rules! SL {
    ($($tts:tt)*) => { stringify!($($tts)*) };
}

macro_rules! as_expr {
    ($e:expr) => {$e};
}

macro_rules! check {
    (@ $from:ty, $to:ty=> $(;)*) => {};

    (@ $from:ty, $to:ty=> cident; $($tail:tt)*) => {
        check!(@ $from, $to=> v: '\x00';);
        check!(@ $from, $to=> v: '\x01';);
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> uident; $($tail:tt)*) => {
        check!(@ $from, $to=> v: 0;);
        check!(@ $from, $to=> v: 1;);
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> sident; $($tail:tt)*) => {
        check!(@ $from, $to=> v: -1;);
        check!(@ $from, $to=> v: 0;);
        check!(@ $from, $to=> v: 1;);
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> fident; $($tail:tt)*) => {
        check!(@ $from, $to=> v: -1.0;);
        check!(@ $from, $to=> v:  0.0;);
        check!(@ $from, $to=> v:  1.0;);
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> uidenta; $($tail:tt)*) => {
        check!(@ $from, $to=> a: 0.0;);
        check!(@ $from, $to=> a: 1.0;);

        check!(@ $from, $to=> aRTN: 0.00, 0;);
        check!(@ $from, $to=> aRTN: 0.25, 0;);
        check!(@ $from, $to=> aRTN: 0.50, 1;);
        check!(@ $from, $to=> aRTN: 0.75, 1;);
        check!(@ $from, $to=> aRTN: 1.00, 1;);

        check!(@ $from, $to=> aRNI:  0.00,  0;);
        check!(@ $from, $to=> aRNI:  0.25,  0;);
        check!(@ $from, $to=> aRNI:  0.50,  0;);
        check!(@ $from, $to=> aRNI:  0.75,  0;);
        check!(@ $from, $to=> aRNI:  1.00,  1;);

        check!(@ $from, $to=> aRPI:  0.00,  0;);
        check!(@ $from, $to=> aRPI:  0.25,  1;);
        check!(@ $from, $to=> aRPI:  0.50,  1;);
        check!(@ $from, $to=> aRPI:  0.75,  1;);
        check!(@ $from, $to=> aRPI:  1.00,  1;);

        check!(@ $from, $to=> aRTZ:  0.00,  0;);
        check!(@ $from, $to=> aRTZ:  0.25,  0;);
        check!(@ $from, $to=> aRTZ:  0.50,  0;);
        check!(@ $from, $to=> aRTZ:  0.75,  0;);
        check!(@ $from, $to=> aRTZ:  1.00,  1;);

        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> sidenta; $($tail:tt)*) => {
        check!(@ $from, $to=> a: -1.0;);
        check!(@ $from, $to=> a:  0.0;);
        check!(@ $from, $to=> a:  1.0;);

        check!(@ $from, $to=> aRTN: -1.00, -1;);
        check!(@ $from, $to=> aRTN: -0.75, -1;);
        check!(@ $from, $to=> aRTN: -0.50, -1;);
        check!(@ $from, $to=> aRTN: -0.25,  0;);
        check!(@ $from, $to=> aRTN:  0.00,  0;);
        check!(@ $from, $to=> aRTN:  0.25,  0;);
        check!(@ $from, $to=> aRTN:  0.50,  1;);
        check!(@ $from, $to=> aRTN:  0.75,  1;);
        check!(@ $from, $to=> aRTN:  1.00,  1;);

        check!(@ $from, $to=> aRNI: -1.00, -1;);
        check!(@ $from, $to=> aRNI: -0.75, -1;);
        check!(@ $from, $to=> aRNI: -0.50, -1;);
        check!(@ $from, $to=> aRNI: -0.25, -1;);
        check!(@ $from, $to=> aRNI:  0.00,  0;);
        check!(@ $from, $to=> aRNI:  0.25,  0;);
        check!(@ $from, $to=> aRNI:  0.50,  0;);
        check!(@ $from, $to=> aRNI:  0.75,  0;);
        check!(@ $from, $to=> aRNI:  1.00,  1;);

        check!(@ $from, $to=> aRPI: -1.00, -1;);
        check!(@ $from, $to=> aRPI: -0.75,  0;);
        check!(@ $from, $to=> aRPI: -0.50,  0;);
        check!(@ $from, $to=> aRPI: -0.25,  0;);
        check!(@ $from, $to=> aRPI:  0.00,  0;);
        check!(@ $from, $to=> aRPI:  0.25,  1;);
        check!(@ $from, $to=> aRPI:  0.50,  1;);
        check!(@ $from, $to=> aRPI:  0.75,  1;);
        check!(@ $from, $to=> aRPI:  1.00,  1;);

        check!(@ $from, $to=> aRTZ: -1.00, -1;);
        check!(@ $from, $to=> aRTZ: -0.75,  0;);
        check!(@ $from, $to=> aRTZ: -0.50,  0;);
        check!(@ $from, $to=> aRTZ: -0.25,  0;);
        check!(@ $from, $to=> aRTZ:  0.00,  0;);
        check!(@ $from, $to=> aRTZ:  0.25,  0;);
        check!(@ $from, $to=> aRTZ:  0.50,  0;);
        check!(@ $from, $to=> aRTZ:  0.75,  0;);
        check!(@ $from, $to=> aRTZ:  1.00,  1;);

        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> fidenta; $($tail:tt)*) => {
        check!(@ $from, $to=> a: -1.0;);
        check!(@ $from, $to=> a:  0.0;);
        check!(@ $from, $to=> a:  1.0;);
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> v: $src:expr, !$dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, v: {}, !{}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.value_into();
            assert_eq!(dst, Err($dst(src)));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> v: $src:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, v: {}", SL!($from), SL!($to), SL!($src));
            let src: $from = $src;
            let dst: Result<$to, _> = src.value_into();
            assert_eq!(dst, Ok($src as $to));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qv: *; $($tail:tt)*) => {
        {
            extern crate quickcheck;
            println!("? {} => {}, qv: *", SL!($from), SL!($to));

            fn property(v: $from) -> bool {
                let dst: Result<$to, _> = v.value_into();
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

    (@ $from:ty, $to:ty=> qv: (+-$bound:expr); $($tail:tt)*) => {
        {
            extern crate quickcheck;
            println!("? {} => {}, qv: (+- {})", SL!($from), SL!($to), SL!($bound));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError<_>> = v.value_into().map_err(From::from);
                if !(-$bound as $from <= v) {
                    dst == Err(conv::FloatError::NegOverflow(v))
                } else if !(v <= $bound as $from) {
                    dst == Err(conv::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qv: (, $bound:expr); $($tail:tt)*) => {
        {
            extern crate quickcheck;
            println!("? {} => {}, qv: (, {})", SL!($from), SL!($to), SL!($bound));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError<_>> = v.value_into().map_err(From::from);
                if !(v <= $bound as $from) {
                    dst == Err(conv::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qv: +; $($tail:tt)*) => {
        {
            extern crate quickcheck;
            println!("? {} => {}, qv: +", SL!($from), SL!($to));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError<_>> = v.value_into().map_err(From::from);
                if !(0 <= v) {
                    dst == Err(conv::FloatError::NegOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qv: +$max:ty=> $($tail:tt)*) => {
        {
            extern crate quickcheck;
            println!("? {} => {}, qv: +{}", SL!($from), SL!($to), SL!($max));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError<_>> = v.value_into().map_err(From::from);
                if !(v <= <$max>::max_value() as $from) {
                    dst == Err(conv::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qv: $bound:ty=> $($tail:tt)*) => {
        {
            extern crate quickcheck;
            println!("? {} => {}, qv: {}", SL!($from), SL!($to), SL!($bound));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError<_>> = v.value_into().map_err(From::from);
                if !(<$bound>::min_value() as $from <= v) {
                    dst == Err(conv::FloatError::NegOverflow(v))
                } else if !(v <= <$bound>::max_value() as $from) {
                    dst == Err(conv::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qv: $min:ty, $max:ty=> $($tail:tt)*) => {
        {
            extern crate quickcheck;
            println!("? {} => {}, qv: {}, {}", SL!($from), SL!($to), SL!($min), SL!($max));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError<_>> = v.value_into().map_err(From::from);
                if !(<$min>::min_value() as $from <= v) {
                    dst == Err(conv::FloatError::NegOverflow(v))
                } else if !(v <= <$max>::max_value() as $from) {
                    dst == Err(conv::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> a: $src:expr, !$dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, a: {}, !{}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_as();
            assert_eq!(dst, Err($dst(src)));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> a: $src:expr, $dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, a: {}, {}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_as();
            assert_eq!(dst, Ok($dst));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> a: $src:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, a: {}", SL!($from), SL!($to), SL!($src));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_as();
            assert_eq!(dst, Ok($src as $to));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qa: *; $($tail:tt)*) => {
        {
            println!("? {} => {}, qa: *", SL!($from), SL!($to));
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, _> = v.approx_as();
                dst == Ok(v as $to)
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qa: +; $($tail:tt)*) => {
        {
            extern crate quickcheck;
            println!("? {} => {}, qa: +", SL!($from), SL!($to));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError<_>> = v.approx_as().map_err(From::from);
                if !(0 <= v) {
                    dst == Err(conv::FloatError::NegOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qa: +$max:ty=> $($tail:tt)*) => {
        {
            extern crate quickcheck;
            println!("? {} => {}, qa: +{}", SL!($from), SL!($to), SL!($max));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError<_>> = v.approx_as().map_err(From::from);
                if !(v <= <$max>::max_value() as $from) {
                    dst == Err(conv::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qa: $bound:ty=> $($tail:tt)*) => {
        {
            extern crate quickcheck;
            println!("? {} => {}, qa: {}", SL!($from), SL!($to), SL!($bound));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError<_>> = v.approx_as().map_err(From::from);
                if !(<$bound>::min_value() as $from <= v) {
                    dst == Err(conv::FloatError::NegOverflow(v))
                } else if !(v <= <$bound>::max_value() as $from) {
                    dst == Err(conv::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> qaW: *; $($tail:tt)*) => {
        {
            extern crate quickcheck;
            println!("? {} => {}, qaW: *", SL!($from), SL!($to));

            fn property(v: $from) -> bool {
                let dst: Result<$to, _> = v.approx_as_by::<_, Wrapping>();
                dst == Ok(v as $to)
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qaW {:?}", err)
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> aRTN: $src:expr, $dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, aRTN: {}, {}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_by::<conv::RoundToNearest>();
            assert_eq!(dst, Ok($dst));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> aRNI: $src:expr, $dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, aRNI: {}, {}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_by::<conv::RoundToNegInf>();
            assert_eq!(dst, Ok($dst));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> aRPI: $src:expr, $dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, aRPI: {}, {}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_by::<conv::RoundToPosInf>();
            assert_eq!(dst, Ok($dst));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> aRTZ: $src:expr, $dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, aRTZ: {}, {}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_by::<conv::RoundToZero>();
            assert_eq!(dst, Ok($dst));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    ($from:ty, $to:ty=> $($tail:tt)*) => {
        check! { @ $from, $to=> $($tail)*; }
    };
}

macro_rules! for_bitness {
    (32 {$($bits32:tt)*} 64 {$($bits64:tt)*}) => {
        as_expr!(
            {
                #[cfg(target_pointer_width="32")]
                fn for_bitness() {
                    $($bits32)*
                }

                #[cfg(target_pointer_width="64")]
                fn for_bitness() {
                    $($bits64)*
                }

                for_bitness()
            }
        )
    };
}
