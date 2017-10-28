extern crate conv;

#[macro_use] mod util;

use conv::*;

macro_rules! cty {
    ($e:expr, $t:ty) => {
        { let v: $t = $e; v }
    };
}

#[test]
fn test_unwraps() {
    assert_eq!(cty!(0i16.value_into().unwrap(), i32), 0);
    assert_eq!(cty!(127i16.value_into().unwrap(), i8), 127);
    assert_eq!(cty!(128i16.value_into().unwrap_or_saturate(), i8), 127);
    assert_eq!(cty!(128i16.approx().unwrap_or_saturate(), i8), 127);
    assert_eq!(cty!(128i16.approx_by::<Wrapping>().unwrap_or_saturate(), i8), -128);

    assert_eq!(cty!(16_777_216i32.value_into().unwrap(), f32), 16_777_216.0);
    assert_eq!(cty!(16_777_216i32.value_into().unwrap_or_inf(), f32), 16_777_216.0);
    assert_eq!(cty!(16_777_217i32.value_into().unwrap_or_inf(), f32), std::f32::INFINITY);
    assert_eq!(cty!((-16_777_217i32).value_into().unwrap_or_inf(), f32), std::f32::NEG_INFINITY);

    assert_eq!(cty!(16_777_216i32.value_into().unwrap_or_invalid(), f32), 16_777_216.0);
    assert!(cty!(16_777_217i32.value_into().unwrap_or_invalid(), f32).is_nan());
    assert!(cty!((-16_777_217i32).value_into().unwrap_or_invalid(), f32).is_nan());

    assert_eq!(cty!(0u8.value_into().unwrap_ok(), u16), 0);
}
