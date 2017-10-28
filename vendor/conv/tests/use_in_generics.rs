//! Are conversions easily usable in generic code?
extern crate conv;

use conv::prelude::*;

#[test]
fn test_generic_unwrap() {
    fn do_conv<T, U>(t: T) -> U
    where T: ValueInto<U> {
        t.value_into().unwrap()
    }

    assert_eq!({let x: u8 = do_conv(42i32); x}, 42u8);
}
