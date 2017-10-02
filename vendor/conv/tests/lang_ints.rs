extern crate conv;

#[macro_use] mod util;

use conv::*;

use conv::NegOverflow as Uf;
use conv::PosOverflow as Of;
use conv::RangeError::NegOverflow as RU;
use conv::RangeError::PosOverflow as RO;

#[test]
fn test_i8() {
    check!(i8, i8=> sident; qv: *; qa: *; qaW: *);
    check!(i8, i16=> sident; qv: *; qa: *; qaW: *);
    check!(i8, i32=> sident; qv: *; qa: *; qaW: *);
    check!(i8, i64=> sident; qv: *; qa: *; qaW: *);
    check!(i8, u8=> uident; qv: +; qa: +; qaW: *;
        v: -1, !Uf;
    );
    check!(i8, u16=> uident; qv: +; qa: +; qaW: *;
        v: -1, !Uf;
    );
    check!(i8, u32=> uident; qv: +; qa: +; qaW: *;
        v: -1, !Uf;
    );
    check!(i8, u64=> uident; qv: +; qa: +; qaW: *;
        v: -1, !Uf;
    );
    check!(i8, isize=> sident; qv: *; qa: *; qaW: *);
    check!(i8, usize=> uident; qv: +; qa: +; qaW: *;
        v: -1, !Uf;
    );
}

#[test]
fn test_i16() {
    check!(i16, i8=> sident; qv: i8=> qa: i8=> qaW: *;
        v: -129, !RU; v: 128, !RO;
    );
    check!(i16, i16=> sident; qv: *; qa: *; qaW: *);
    check!(i16, i32=> sident; qv: *; qa: *; qaW: *);
    check!(i16, i64=> sident; qv: *; qa: *; qaW: *);
    check!(i16, u8=> uident; qv: u8=> qa: +; qaW: *;
        v: -1, !RU;
    );
    check!(i16, u16=> uident; qv: u16, i16=> qa: +; qaW: *;
        v: -1, !Uf;
    );
    check!(i16, u32=> uident; qv: +; qa: +; qaW: *;
        v: -1, !Uf;
    );
    check!(i16, u64=> uident; qv: +; qa: +; qaW: *;
        v: -1, !Uf;
    );
    check!(i16, isize=> sident; qv: *; qa: *; qaW: *);
    check!(i16, usize=> uident; qv: +; qa: +; qaW: *;
        v: -1, !Uf;
    );
}

#[test]
fn test_i32() {
    check!(i32, i8=> sident; qv: i8=> qa: i8=> qaW: *;
        v: -129, !RU; v: 128, !RO;
    );
    check!(i32, i16=> sident; qv: i16=> qa: i16=> qaW: *;
        v: -32_769, !RU; v: 32_768, !RO;
    );
    check!(i32, i32=> sident; qv: *; qa: *; qaW: *);
    check!(i32, i64=> sident; qv: *; qa: *; qaW: *);
    check!(i32, u8=> uident; qv: u8=> qa: u8=> qaW: *;
        v: -1, !RU;
    );
    check!(i32, u16=> uident; qv: u16=> qa: u16=> qaW: *;
        v: -1, !RU;
    );
    check!(i32, u32=> uident; qv: +; qa: +; qaW: *;
        v: -1, !Uf;
    );
    check!(i32, u64=> uident; qv: +; qa: +; qaW: *;
        v: -1, !Uf;
    );
    for_bitness! {
        32 {
            check!(i32, isize=> sident; qv: *; qa: *; qaW: *);
            check!(i32, usize=> uident; qv: +; qa: +; qaW: *;
                v: -1, !Uf;
            );
        }
        64 {
            check!(i32, isize=> sident; qv: *; qa: *; qaW: *);
            check!(i32, usize=> uident; qv: +; qa: +; qaW: *;
                v: -1, !Uf;
            );
        }
    }
}

#[test]
fn test_i64() {
    check!(i64, i8=> sident; qv: i8=> qa: i8=> qaW: *;
        v: -129, !RU; v: 128, !RO;
    );
    check!(i64, i16=> sident; qv: i16=> qa: i16=> qaW: *;
        v: -32_769, !RU; v: 32_768, !RO;
    );
    check!(i64, i32=> sident; qv: i32=> qa: i32=> qaW: *;
        v: -2_147_483_649, !RU; v: 2_147_483_648, !RO;
    );
    check!(i64, i64=> sident; qv: *; qa: *; qaW: *;
    );
    check!(i64, u8=> uident; qv: u8=> qa: u8=> qaW: *;
        v: -1, !RU;
    );
    check!(i64, u16=> uident; qv: u16=> qa: u16=> qaW: *;
        v: -1, !RU;
    );
    check!(i64, u32=> uident; qv: u32=> qa: u32=> qaW: *;
        v: -1, !RU;
    );
    check!(i64, u64=> uident; qv: +; qa: +; qaW: *;
        v: -1, !Uf;
    );
    for_bitness! {
        32 {
            check!(i64, isize=> sident; qv: isize=> qa: isize=> qaW: *;
                v: -2_147_483_649, !RU; v: 2_147_483_648, !RO;
            );
            check!(i64, usize=> uident; qv: usize=> qa: usize=> qaW: *;
                v: -1, !RU; v: 4_294_967_296, !RO;
            );
        }
        64 {
            check!(i64, isize=> sident; qv: *; qa: *; qaW: *;
            );
            check!(i64, usize=> uident; qv: +; qa: +; qaW: *;
                v: -1, !Uf;
            );
        }
    }
}

#[test]
fn test_u8() {
    check!(u8, i8=> uident; qv: +i8=> qa: +i8=> qaW: *;
        v: 127; v: 128, !Of;
    );
    check!(u8, i16=> uident; qv: *; qa: *; qaW: *);
    check!(u8, i32=> uident; qv: *; qa: *; qaW: *);
    check!(u8, i64=> uident; qv: *; qa: *; qaW: *);
    check!(u8, u8=> uident; qv: *; qa: *; qaW: *);
    check!(u8, u16=> uident; qv: *; qa: *; qaW: *);
    check!(u8, u32=> uident; qv: *; qa: *; qaW: *);
    check!(u8, u64=> uident; qv: *; qa: *; qaW: *);
    check!(u8, isize=> uident; qv: *; qa: *; qaW: *);
    check!(u8, usize=> uident; qv: *; qa: *; qaW: *);
}

#[test]
fn test_u16() {
    check!(u16, i8=> uident; qv: +i8=> qa: +i8=> qaW: *;
        v: 128, !Of;
    );
    check!(u16, i16=> uident; qv: +i16=> qa: +i16=> qaW: *;
        v: 32_768, !Of;
    );
    check!(u16, i32=> uident; qv: *; qa: *; qaW: *);
    check!(u16, i64=> uident; qv: *; qa: *; qaW: *);
    check!(u16, u8=> uident; qv: u8=> qa: u8=> qaW: *;
        v: 256, !Of;
    );
    check!(u16, u16=> uident; qv: *; qa: *; qaW: *);
    check!(u16, u32=> uident; qv: *; qa: *; qaW: *);
    check!(u16, u64=> uident; qv: *; qa: *; qaW: *);
    check!(u16, isize=> uident; qv: *; qa: *; qaW: *);
    check!(u16, usize=> uident; qv: *; qa: *; qaW: *);
}

#[test]
fn test_u32() {
    check!(u32, i8=> uident; qv: +i8=> qa: +i8=> qaW: *;
        v: 128, !Of;
    );
    check!(u32, i16=> uident; qv: +i16=> qa: +i16=> qaW: *;
        v: 32_768, !Of;
    );
    check!(u32, i32=> uident; qv: +i32=> qa: +i32=> qaW: *;
        v: 2_147_483_648, !Of;
    );
    check!(u32, i64=> uident; qv: *; qa: *; qaW: *);
    check!(u32, u8=> uident; qv: u8=> qa: u8=> qaW: *;
        v: 256, !Of;
    );
    check!(u32, u16=> uident; qv: u16=> qa: u16=> qaW: *;
        v: 65_536, !Of;
    );
    check!(u32, u32=> uident; qv: *; qa: *; qaW: *);
    check!(u32, u64=> uident; qv: *; qa: *; qaW: *);
    for_bitness! {
        32 {
            check!(u32, isize=> uident; qv: +isize=> qa: +isize=> qaW: *;
                v: 2_147_483_647; v: 2_147_483_648, !Of;
            );
            check!(u32, usize=> uident; qv: *; qa: *; qaW: *);
        }
        64 {
            check!(u32, isize=> uident; qv: *; qa: *; qaW: *);
            check!(u32, usize=> uident; qv: *; qa: *; qaW: *);
        }
    }
}

#[test]
fn test_u64() {
    check!(u64, i8=> uident; qv: +i8=> qa: +i8=> qaW: *;
        v: 128, !Of;
    );
    check!(u64, i16=> uident; qv: +i16=> qa: +i16=> qaW: *;
        v: 32_768, !Of;
    );
    check!(u64, i32=> uident; qv: +i32=> qa: +i32=> qaW: *;
        v: 2_147_483_648, !Of;
    );
    check!(u64, i64=> uident; qv: +i64=> qa: +i64=> qaW: *;
        v: 9_223_372_036_854_775_808, !Of;
    );
    check!(u64, u8=> uident; qv: u8=> qa: u8=> qaW: *;
        v: 256, !Of;
    );
    check!(u64, u16=> uident; qv: u16=> qa: u16=> qaW: *;
        v: 65_536, !Of;
    );
    check!(u64, u32=> uident; qv: u32=> qa: u32=> qaW: *;
        v: 4_294_967_296, !Of;
    );
    check!(u64, u64=> uident; qv: *; qa: *; qaW: *);
    for_bitness! {
        32 {
            check!(u64, isize=> uident; qv: +isize=> qa: +isize=> qaW: *;
                v: 2_147_483_648, !Of;
            );
            check!(u64, usize=> uident; qv: usize=> qa: usize=> qaW: *;
                v: 4_294_967_296, !Of;
            );
        }
        64 {
            check!(u64, isize=> uident; qv: +i64=> qa: +i64=> qaW: *;
                v: 9_223_372_036_854_775_808, !Of;
            );
            check!(u64, usize=> uident; qv: *; qa: *; qaW: *);
        }
    }
}

#[test]
fn test_isize() {
    check!(isize, i8=> sident; qv: i8=> qa: i8=> qaW: *;
        v: -129, !RU; v: 128, !RO;
    );
    check!(isize, i16=> sident; qv: i16=> qa: i16=> qaW: *;
        v: -32_769, !RU; v: 32_768, !RO;
    );
    check!(isize, u8=> uident; qv: u8=> qa: u8=> qaW: *;
        v: -1, !RU; v: 256, !RO;
    );
    check!(isize, u16=> uident; qv: u16=> qa: u16=> qaW: *;
        v: -1, !RU; v: 65_536, !RO;
    );
    check!(isize, isize=> sident; qv: *; qa: *; qaW: *);
    for_bitness! {
        32 {
            check!(isize, i32=> sident; qv: *; qa: *; qaW: *);
            check!(isize, i64=> sident; qv: *; qa: *; qaW: *);
            check!(isize, u32=> uident; qv: +; qa: +; qaW: *;
                v: -1, !Uf;
            );
            check!(isize, u64=> uident; qv: +; qa: +; qaW: *;
                v: -1, !Uf;
            );
            check!(isize, usize=> uident; qv: +; qa: +; qaW: *;
                v: -1, !Uf;
            );
        }
        64 {
            check!(isize, i32=> sident; qv: *; qa: *; qaW: *);
            check!(isize, i64=> sident; qv: *; qa: *; qaW: *);
            check!(isize, u32=> uident; qv: u32=> qa: u32=> qaW: *;
                v: -1, !RU; v: 4_294_967_296, !RO;
            );
            check!(isize, u64=> uident; qv: +; qa: +; qaW: *;
                v: -1, !Uf;
            );
            check!(isize, usize=> uident; qv: +; qa: +; qaW: *;
                v: -1, !Uf;
            );
        }
    }
}

#[test]
fn test_usize() {
    check!(usize, i8=> uident; qv: +i8=> qa: +i8=> qaW: *;
        v: 128, !Of;
    );
    check!(usize, i16=> uident; qv: +i16=> qa: +i16=> qaW: *;
        v: 32_768, !Of;
    );
    check!(usize, u8=> uident; qv: u8=> qa: u8=> qaW: *;
        v: 256, !Of;
    );
    check!(usize, u16=> uident; qv: u16=> qa: u16=> qaW: *;
        v: 65_536, !Of;
    );
    check!(usize, usize=> uident; qv: *; qa: *; qaW: *);
    for_bitness! {
        32 {
            check!(usize, i32=> uident; qv: +i32=> qa: +i32=> qaW: *);
            check!(usize, i64=> uident; qv: *; qa: *; qaW: *);
            check!(usize, u32=> uident; qv: *; qa: *; qaW: *);
            check!(usize, u64=> uident; qv: *; qa: *; qaW: *);
            check!(usize, isize=> uident; qv: +isize=> qa: +isize=> qaW: *);
        }
        64 {
            check!(usize, i32=> uident; qv: +i32=> qa: +i32=> qaW: *);
            check!(usize, i64=> uident; qv: +i64=> qa: +i64=> qaW: *);
            check!(usize, u32=> uident; qv: u32=> qa: u32=> qaW: *;
                v: 4_294_967_296, !Of;
            );
            check!(usize, u64=> uident; qv: *; qa: *; qaW: *);
            check!(usize, isize=> uident; qv: +isize=> qa: +isize=> qaW: *);
        }
    }
}

#[test]
fn test_i_to_f() {
    check!(i8,  f32=> sident; qv: *; qa: *);
    check!(i16, f32=> sident; qv: *; qa: *);
    check!(i32, f32=> sident; qv: (+-16_777_216); qa: *;
        v: -16_777_217, !RU; v: 16_777_217, !RO;
    );
    check!(i64, f32=> sident; qv: (+-16_777_216); qa: *;
        v: -16_777_217, !RU; v: 16_777_217, !RO;
    );
    check!(isize, f32=> sident; qv: (+-16_777_216); qa: *;
        v: -16_777_217, !RU; v: 16_777_217, !RO;
    );

    check!(u8,  f32=> uident; qv: *; qa: *);
    check!(u16, f32=> uident; qv: *; qa: *);
    check!(u32, f32=> uident; qv: (, 16_777_216); qa: *;
        v: 16_777_217, !Of;
    );
    check!(u64, f32=> uident; qv: (, 16_777_216); qa: *;
        v: 16_777_217, !Of;
    );
    check!(usize, f32=> uident; qv: (, 16_777_216); qa: *;
        v: 16_777_217, !Of;
    );

    check!(i8,  f64=> sident; qv: *; qa: *);
    check!(i16, f64=> sident; qv: *; qa: *);
    check!(i32, f64=> sident; qv: *; qa: *);
    check!(i64, f64=> sident; qv: (+-9_007_199_254_740_992); qa: *;
        v: -9_007_199_254_740_993, !RU; v: 9_007_199_254_740_993, !RO;
    );
    for_bitness! {
        32 {
            check!(isize, f64=> sident; qv: *; qa: *);
        }
        64 {
            check!(i64, f64=> sident; qv: (+-9_007_199_254_740_992); qa: *;
                v: -9_007_199_254_740_993, !RU; v: 9_007_199_254_740_993, !RO;
            );
        }
    }

    check!(u8,  f64=> uident; qv: *; qa: *);
    check!(u16, f64=> uident; qv: *; qa: *);
    check!(u32, f64=> uident; qv: *; qa: *);
    check!(u64, f64=> uident; qv: (, 9_007_199_254_740_992); qa: *;
        v: 9_007_199_254_740_993, !Of;
    );
    for_bitness! {
        32 {
            check!(usize, f64=> uident; qv: *; qa: *);
        }
        64 {
            check!(u64, f64=> uident; qv: (, 9_007_199_254_740_992); qa: *;
                v: 9_007_199_254_740_993, !Of;
            );
        }
    }
}
