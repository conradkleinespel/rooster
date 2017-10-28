
# `conv`

This crate provides a number of conversion traits with more specific semantics than those provided by `as` or `From`/`Into`.

The goal with the traits provided here is to be more specific about what generic code can rely on, as well as provide reasonably self-describing alternatives to the standard `From`/`Into` traits.  For example, the although `T: From<U>` might be satisfied in generic code, this says nothing about what *kind* of conversion that represents.

In addition, `From`/`Into` provide no facility for a conversion failing, meaning that implementations may need to choose between conversions that may not be valid, or panicking; neither option is appealing in general.

**Links**

* [Latest Release](https://crates.io/crates/scan-rules/)
* [Latest Docs](https://danielkeep.github.io/rust-scan-rules/doc/scan_rules/index.html)
* [Repository](https://github.com/DanielKeep/rust-scan-rules)

## Compatibility

`conv` is compatible with Rust 1.2 and higher.

## Examples

```rust
# extern crate conv;
# use conv::*;
# fn main() {
// This *cannot* fail, so we can use `unwrap_ok` to discard the `Result`.
assert_eq!(u8::value_from(0u8).unwrap_ok(), 0u8);

// This *can* fail.  Specifically, it can overflow toward negative infinity.
assert_eq!(u8::value_from(0i8),     Ok(0u8));
assert_eq!(u8::value_from(-1i8),    Err(NegOverflow(-1)));

// This can overflow in *either* direction; hence the change to `RangeError`.
assert_eq!(u8::value_from(-1i16),   Err(RangeError::NegOverflow(-1)));
assert_eq!(u8::value_from(0i16),    Ok(0u8));
assert_eq!(u8::value_from(256i16),  Err(RangeError::PosOverflow(256)));

// We can use the extension traits to simplify this a little.
assert_eq!(u8::value_from(-1i16).unwrap_or_saturate(),  0u8);
assert_eq!(u8::value_from(0i16).unwrap_or_saturate(),   0u8);
assert_eq!(u8::value_from(256i16).unwrap_or_saturate(), 255u8);

// Obviously, all integers can be "approximated" using the default scheme (it
// doesn't *do* anything), but they can *also* be approximated with the
// `Wrapping` scheme.
assert_eq!(
    <u8 as ApproxFrom<_, DefaultApprox>>::approx_from(400u16),
    Err(PosOverflow(400)));
assert_eq!(
    <u8 as ApproxFrom<_, Wrapping>>::approx_from(400u16),
    Ok(144u8));

// This is rather inconvenient; as such, there are a number of convenience
// extension methods available via `ConvUtil` and `ConvAsUtil`.
assert_eq!(400u16.approx(),                       Err::<u8, _>(PosOverflow(400)));
assert_eq!(400u16.approx_by::<Wrapping>(),        Ok::<u8, _>(144u8));
assert_eq!(400u16.approx_as::<u8>(),              Err(PosOverflow(400)));
assert_eq!(400u16.approx_as_by::<u8, Wrapping>(), Ok(144));

// Integer -> float conversions *can* fail due to limited precision.
// Once the continuous range of exactly representable integers is exceeded, the
// provided implementations fail with overflow errors.
assert_eq!(f32::value_from(16_777_216i32), Ok(16_777_216.0f32));
assert_eq!(f32::value_from(16_777_217i32), Err(RangeError::PosOverflow(16_777_217)));

// Float -> integer conversions have to be done using approximations.  Although
// exact conversions are *possible*, "advertising" this with an implementation
// is misleading.
//
// Note that `DefaultApprox` for float -> integer uses whatever rounding
// mode is currently active (*i.e.* whatever `as` would do).
assert_eq!(41.0f32.approx(), Ok(41u8));
assert_eq!(41.3f32.approx(), Ok(41u8));
assert_eq!(41.5f32.approx(), Ok(41u8));
assert_eq!(41.8f32.approx(), Ok(41u8));
assert_eq!(42.0f32.approx(), Ok(42u8));

assert_eq!(255.0f32.approx(), Ok(255u8));
assert_eq!(256.0f32.approx(), Err::<u8, _>(FloatError::PosOverflow(256.0)));

// Sometimes, it can be useful to saturate the conversion from float to
// integer directly, then account for NaN as input separately.  The `Saturate`
// extension trait exists for this reason.
assert_eq!((-23.0f32).approx_as::<u8>().saturate(), Ok(0));
assert_eq!(302.0f32.approx_as::<u8>().saturate(), Ok(255u8));
assert!(std::f32::NAN.approx_as::<u8>().saturate().is_err());

// If you really don't care about the specific kind of error, you can just rely
// on automatic conversion to `GeneralErrorKind`.
fn too_many_errors() -> Result<(), GeneralErrorKind> {
    assert_eq!({let r: u8 = try!(0u8.value_into()); r},  0u8);
    assert_eq!({let r: u8 = try!(0i8.value_into()); r},  0u8);
    assert_eq!({let r: u8 = try!(0i16.value_into()); r}, 0u8);
    assert_eq!({let r: u8 = try!(0.0f32.approx()); r},   0u8);
    Ok(())
}
# let _ = too_many_errors();
# }
```

## Change Log

### v0.3.2

- Added integer ↔ `char` conversions.
- Added missing `isize`/`usize` → `f32`/`f64` conversions.
- Fixed the error type of `i64` → `usize` for 64-bit targets.

### v0.3.1

- Change to `unwrap_ok` for better codegen (thanks bluss).
- Fix for Rust breaking change (code in question was dodgy anyway; thanks m4rw3r).

### v0.3.0

- Added an `Error` constraint to all `Err` associated types.  This will break any user-defined conversions where the `Err` type does not implement `Error`.
- Renamed the `Overflow` and `Underflow` errors to `PosOverflow` and `NegOverflow` respectively.  In the context of floating point conversions, "underflow" usually means the value was too close to zero to correctly represent.

### v0.2.1

- Added `ConvUtil::into_as<Dst>` as a shortcut for `Into::<Dst>::into`.
- Added `#[inline]` attributes.
- Added `Saturate::saturate`, which can saturate `Result`s arising from over/underflow.

### v0.2.0

- Changed all error types to include the original input as payload.  This breaks pretty much *everything*.  Sorry about that.  On the bright side, there's now no downside to using the conversion traits for non-`Copy` types.
- Added the normal rounding modes for float → int approximations: `RoundToNearest`, `RoundToNegInf`, `RoundToPosInf`, and `RoundToZero`.
- `ApproxWith` is now subsumed by a pair of extension traits (`ConvUtil` and `ConvAsUtil`), that also have shortcuts for `TryInto` and `ValueInto` so that you can specify the destination type on the method.
