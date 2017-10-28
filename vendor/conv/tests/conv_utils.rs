#[macro_use] extern crate conv;

use conv::prelude::*;

#[test]
fn test_approx() {
    use conv::DefaultApprox;
    assert_eq!((1.5f32).approx(), Ok(1i32));
    assert_eq!((1.5f32).approx_by::<DefaultApprox>(), Ok(1));
    assert_eq!((1.5f32).approx_as::<i32>(), Ok(1));
    assert_eq!((1.5f32).approx_as_by::<i32, DefaultApprox>(), Ok(1));
}

#[test]
fn test_into() {
    let v = "ABC".into_as::<Vec<u8>>();
    assert_eq!(&*v, &[0x41, 0x42, 0x43]);
}

#[test]
fn test_try() {
    #[derive(PartialEq, Debug)] enum ItAintRight { BabeNo, NoNo }
    TryFrom! { (u8) enum ItAintRight { BabeNo, NoNo } }

    assert_eq!(0u8.try_as::<ItAintRight>(), Ok(ItAintRight::BabeNo));
    assert_eq!(1u8.try_as::<ItAintRight>(), Ok(ItAintRight::NoNo));
    assert_eq!(2u8.try_as::<ItAintRight>(), Err(conv::Unrepresentable(2)));
}

#[test]
fn test_value() {
    assert_eq!((123u32).value_as::<u8>(), Ok(123));
}

#[test]
fn test_whizzo() {
    use conv::errors::Unrepresentable;
    assert_eq!((-1.0f32).approx_as::<u8>().saturate(), Ok::<_, Unrepresentable<_>>(0u8));
    assert_eq!((-1i32).value_as::<u8>().saturate().unwrap_ok(), 0u8);
}
