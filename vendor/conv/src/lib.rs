/*!
This crate provides a number of conversion traits with more specific semantics than those provided by `as` or `From`/`Into`.

The goal with the traits provided here is to be more specific about what generic code can rely on, as well as provide reasonably self-describing alternatives to the standard `From`/`Into` traits.  For example, the although `T: From<U>` might be satisfied, it imposes no restrictions on the *kind* of conversion being implemented.  As such, the traits in this crate try to be very specific about what conversions are allowed.  This makes them less generally applicable, but more useful where they *do* apply.

In addition, `From`/`Into` requires all conversions to succeed or panic.  All conversion traits in this crate define an associated error type, allowing code to react to failed conversions as appropriate.

<style type="text/css">
.link-block { font-family: "Fira Sans"; }
.link-block > p { display: inline-block; }
.link-block > p > strong { font-weight: 500; margin-right: 1em; }
.link-block > ul { display: inline-block; padding: 0; list-style: none; }
.link-block > ul > li {
  font-size: 0.8em;
  background-color: #eee;
  border: 1px solid #ccc;
  padding: 0.3em;
  display: inline-block;
}
</style>
<span></span><div class="link-block">

**Links**

* [Latest Release](https://crates.io/crates/scan-rules/)
* [Latest Docs](https://danielkeep.github.io/rust-scan-rules/doc/scan_rules/index.html)
* [Repository](https://github.com/DanielKeep/rust-scan-rules)

<span></span></div>

## Compatibility

`conv` is compatible with Rust 1.2 and higher.

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

# Overview

The following traits are used to define various conversion semantics:

- [`ApproxFrom`](./trait.ApproxFrom.html)/[`ApproxInto`](./trait.ApproxInto.html) - approximate conversions, with selectable approximation scheme (see [`ApproxScheme`](./trait.ApproxScheme.html)).
- [`TryFrom`](./trait.TryFrom.html)/[`TryInto`](./trait.TryInto.html) - general, potentially failing value conversions.
- [`ValueFrom`](./trait.ValueFrom.html)/[`ValueInto`](./trait.ValueInto.html) - exact, value-preserving conversions.

When *defining* a conversion, try to implement the `*From` trait variant where possible.  When *using* a conversion, try to depend on the `*Into` trait variant where possible.  This is because the `*Into` traits automatically use `*From` implementations, but not the reverse.  Implementing `*From` and using `*Into` ensures conversions work in as many contexts as possible.

These extension methods are provided to help with some common cases:

- [`ConvUtil::approx_as<Dst>`](./trait.ConvUtil.html#method.approx_as) - approximates to `Dst` with the `DefaultApprox` scheme.
- [`ConvUtil::approx_as_by<Dst, S>`](./trait.ConvUtil.html#method.approx_as_by) - approximates to `Dst` with the scheme `S`.
- [`ConvUtil::into_as<Dst>`](./trait.ConvUtil.html#method.into_as) - converts to `Dst` using `Into::into`.
- [`ConvUtil::try_as<Dst>`](./trait.ConvUtil.html#method.try_as) - converts to `Dst` using `TryInto::try_into`.
- [`ConvUtil::value_as<Dst>`](./trait.ConvUtil.html#method.value_as) - converts to `Dst` using `ValueInto::value_into`.
- [`ConvAsUtil::approx`](./trait.ConvAsUtil.html#method.approx) - approximates to an inferred destination type with the `DefaultApprox` scheme.
- [`ConvAsUtil::approx_by<S>`](./trait.ConvAsUtil.html#method.approx_by) - approximates to an inferred destination type with the scheme `S`.
- [`Saturate::saturate`](./errors/trait.Saturate.html#tymethod.saturate) - saturates on overflow.
- [`UnwrapOk::unwrap_ok`](./errors/trait.UnwrapOk.html#tymethod.unwrap_ok) - unwraps results from conversions that cannot fail.
- [`UnwrapOrInf::unwrap_or_inf`](./errors/trait.UnwrapOrInf.html#tymethod.unwrap_or_inf) - saturates to ±∞ on failure.
- [`UnwrapOrInvalid::unwrap_or_invalid`](./errors/trait.UnwrapOrInvalid.html#tymethod.unwrap_or_invalid) - substitutes the target type's "invalid" sentinel value on failure.
- [`UnwrapOrSaturate::unwrap_or_saturate`](./errors/trait.UnwrapOrSaturate.html#tymethod.unwrap_or_saturate) - saturates to the maximum or minimum value of the target type on failure.

A macro is provided to assist in implementing conversions:

- [`TryFrom!`](./macros/index.html#tryfrom!) - derives an implementation of [`TryFrom`](./trait.TryFrom.html).

If you are implementing your own types, you may also be interested in the traits contained in the [`misc`](./misc/index.html) module.

## Provided Implementations

The crate provides several blanket implementations:

- `*From<A> for A` (all types can be converted from and into themselves).
- `*Into<Dst> for Src where Dst: *From<Src>` (`*From` implementations imply a matching `*Into` implementation).

Conversions for the builtin numeric (integer and floating point) types are provided.  In general, `ValueFrom` conversions exist for all pairs except for float → integer (since such a conversion is generally unlikely to *exactly* succeed) and `f64 → f32` (for the same reason).  `ApproxFrom` conversions with the `DefaultApprox` scheme exist between all pairs.  `ApproxFrom` with the `Wrapping` scheme exist between integers.

## Errors

A number of error types are defined in the [`errors`](./errors/index.html) module.  Generally, conversions use whichever error type most *narrowly* defines the kinds of failures that can occur.  For example:

- `ValueFrom<u8> for u16` cannot possibly fail, and as such it uses `NoError`.
- `ValueFrom<i8> for u16` can *only* fail with a negative overflow, thus it uses the `NegOverflow` type.
- `ValueFrom<i32> for u16` can overflow in either direction, hence it uses `RangeError`.
- Finally, `ApproxFrom<f32> for u16` can overflow (positive or negative), or attempt to convert NaN; `FloatError` covers those three cases.

Because there are *numerous* error types, the `GeneralError` enum is provided.  `From<E, T> for GeneralError<T>` exists for each error type `E<T>` defined by this crate (even for `NoError`!), allowing errors to be translated automatically by `try!`.  In fact, all errors can be "expanded" to *all* more general forms (*e.g.* `NoError` → `NegOverflow`, `PosOverflow` → `RangeError` → `FloatError`).

Aside from `NoError`, the various error types wrap the input value that you attempted to convert.  This is so that non-`Copy` types do not need to be pre-emptively cloned prior to conversion, just in case the conversion fails.  A downside is that this means there are many, *many* incompatible error types.

To help alleviate this, there is also `GeneralErrorKind`, which is simply `GeneralError<T>` without the payload, and all errors can be converted into it directly.

The reason for not just using `GeneralErrorKind` in the first place is to statically reduce the number of potential error cases you need to deal with.  It also allows the `Unwrap*` extension traits to be defined *without* the possibility for runtime failure (*e.g.* you cannot use `unwrap_or_saturate` with a `FloatError`, because what do you do if the error is `NotANumber`; saturate to max or to min?  Or panic?).

# Examples

```
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

*/

#![deny(missing_docs)]

#[macro_use] extern crate custom_derive;

// Exported macros.
pub mod macros;

pub use errors::{
    NoError, GeneralError, GeneralErrorKind, Unrepresentable,
    NegOverflow, PosOverflow,
    FloatError, RangeError, RangeErrorKind,
    Saturate,
    UnwrapOk, UnwrapOrInf, UnwrapOrInvalid, UnwrapOrSaturate,
};

use std::error::Error;

/**
Publicly re-exports the most generally useful set of items.

Usage of the prelude should be considered **unstable**.  Although items will likely *not* be removed without bumping the major version, new items *may* be added, which could potentially cause name conflicts in user code.
*/
pub mod prelude {
    pub use super::{
        ApproxFrom, ApproxInto,
        ValueFrom, ValueInto,
        GeneralError, GeneralErrorKind,
        Saturate,
        UnwrapOk, UnwrapOrInf, UnwrapOrInvalid, UnwrapOrSaturate,
        ConvUtil, ConvAsUtil,
        RoundToNearest, RoundToZero, Wrapping,
    };
}

macro_rules! as_item {
    ($($i:item)*) => {$($i)*};
}

macro_rules! item_for_each {
    (
        $( ($($arg:tt)*) ),* $(,)* => { $($exp:tt)* }
    ) => {
        macro_rules! body {
            $($exp)*
        }

        $(
            body! { $($arg)* }
        )*
    };
}

pub mod errors;
pub mod misc;

mod impls;

/**
This trait is used to perform a conversion that is permitted to approximate the result, but *not* to wrap or saturate the result to fit into the destination type's representable range.

Where possible, prefer *implementing* this trait over `ApproxInto`, but prefer *using* `ApproxInto` for generic constraints.

# Details

All implementations of this trait must provide a conversion that can be separated into two logical steps: an approximation transform, and a representation transform.

The "approximation transform" step involves transforming the input value into an approximately equivalent value which is supported by the target type *without* taking the target type's representable range into account.  For example, this might involve rounding or truncating a floating point value to an integer, or reducing the accuracy of a floating point value.

The "representation transform" step *exactly* rewrites the value from the source type's binary representation into the destination type's binary representation.  This step *may not* transform the value in any way.  If the result of the approximation is not representable, the conversion *must* fail.

The major reason for this formulation is to exactly define what happens when converting between floating point and integer types.  Often, it is unclear what happens to floating point values beyond the range of the target integer type.  Do they saturate, wrap, or cause a failure?

With this formulation, it is well-defined: if a floating point value is outside the representable range, the conversion fails.  This allows users to distinguish between approximation and range violation, and act accordingly.
*/
pub trait ApproxFrom<Src, Scheme=DefaultApprox>: Sized where Scheme: ApproxScheme {
    /// The error type produced by a failed conversion.
    type Err: Error;

    /// Convert the given value into an approximately equivalent representation.
    fn approx_from(src: Src) -> Result<Self, Self::Err>;
}

impl<Src, Scheme> ApproxFrom<Src, Scheme> for Src where Scheme: ApproxScheme {
    type Err = NoError;
    fn approx_from(src: Src) -> Result<Self, Self::Err> {
        Ok(src)
    }
}

/**
This is the dual of `ApproxFrom`; see that trait for information.

Where possible, prefer *using* this trait over `ApproxFrom` for generic constraints, but prefer *implementing* `ApproxFrom`.
*/
pub trait ApproxInto<Dst, Scheme=DefaultApprox> where Scheme: ApproxScheme {
    /// The error type produced by a failed conversion.
    type Err: Error;

    /// Convert the subject into an approximately equivalent representation.
    fn approx_into(self) -> Result<Dst, Self::Err>;
}

impl<Dst, Src, Scheme> ApproxInto<Dst, Scheme> for Src
where
    Dst: ApproxFrom<Src, Scheme>,
    Scheme: ApproxScheme,
{
    type Err = Dst::Err;
    fn approx_into(self) -> Result<Dst, Self::Err> {
        ApproxFrom::approx_from(self)
    }
}

/**
This trait is used to mark approximation scheme types.
*/
pub trait ApproxScheme {}

/**
The "default" approximation scheme.  This scheme does whatever would generally be expected of a lossy conversion, assuming no additional context or instruction is given.

This is a double-edged sword: it has the loosest semantics, but is far more likely to exist than more complicated approximation schemes.
*/
pub enum DefaultApprox {}
impl ApproxScheme for DefaultApprox {}

/**
This scheme is used to convert a value by "wrapping" it into a narrower range.

In abstract, this can be viewed as the opposite of rounding: rather than preserving the most significant bits of a value, it preserves the *least* significant bits of a value.
*/
pub enum Wrapping {}
impl ApproxScheme for Wrapping {}

/**
This scheme is used to convert a value by rounding it to the nearest representable value, with ties rounding away from zero.
*/
pub enum RoundToNearest {}
impl ApproxScheme for RoundToNearest {}

/**
This scheme is used to convert a value by rounding it toward negative infinity to the nearest representable value.
*/
pub enum RoundToNegInf {}
impl ApproxScheme for RoundToNegInf {}

/**
This scheme is used to convert a value by rounding it toward positive infinity to the nearest representable value.
*/
pub enum RoundToPosInf {}
impl ApproxScheme for RoundToPosInf {}

/**
This scheme is used to convert a value by rounding it toward zero to the nearest representable value.
*/
pub enum RoundToZero {}
impl ApproxScheme for RoundToZero {}

/**
This trait is used to perform a conversion between different semantic types which might fail.

Where possible, prefer *implementing* this trait over `TryInto`, but prefer *using* `TryInto` for generic constraints.

# Details

Typically, this should be used in cases where you are converting between values whose ranges and/or representations only partially overlap.  That the conversion may fail should be a reasonably expected outcome.  A standard example of this is converting from integers to enums of unitary variants.
*/
pub trait TryFrom<Src>: Sized {
    /// The error type produced by a failed conversion.
    type Err: Error;

    /// Convert the given value into the subject type.
    fn try_from(src: Src) -> Result<Self, Self::Err>;
}

impl<Src> TryFrom<Src> for Src {
    type Err = NoError;
    fn try_from(src: Src) -> Result<Self, Self::Err> {
        Ok(src)
    }
}

/**
This is the dual of `TryFrom`; see that trait for information.

Where possible, prefer *using* this trait over `TryFrom` for generic constraints, but prefer *implementing* `TryFrom`.
*/
pub trait TryInto<Dst> {
    /// The error type produced by a failed conversion.
    type Err: Error;

    /// Convert the subject into the destination type.
    fn try_into(self) -> Result<Dst, Self::Err>;
}

impl<Src, Dst> TryInto<Dst> for Src where Dst: TryFrom<Src> {
    type Err = Dst::Err;
    fn try_into(self) -> Result<Dst, Self::Err> {
        TryFrom::try_from(self)
    }
}

/**
This trait is used to perform an exact, value-preserving conversion.

Where possible, prefer *implementing* this trait over `ValueInto`, but prefer *using* `ValueInto` for generic constraints.

# Details

Implementations of this trait should be reflexive, associative and commutative (in the absence of conversion errors).  That is, all possible cycles of `ValueFrom` conversions (for which each "step" has a defined implementation) should produce the same result, with a given value either being "round-tripped" exactly, or an error being produced.
*/
pub trait ValueFrom<Src>: Sized {
    /// The error type produced by a failed conversion.
    type Err: Error;

    /// Convert the given value into an exactly equivalent representation.
    fn value_from(src: Src) -> Result<Self, Self::Err>;
}

impl<Src> ValueFrom<Src> for Src {
    type Err = NoError;
    fn value_from(src: Src) -> Result<Self, Self::Err> {
        Ok(src)
    }
}

/**
This is the dual of `ValueFrom`; see that trait for information.

Where possible, prefer *using* this trait over `ValueFrom` for generic constraints, but prefer *implementing* `ValueFrom`.
*/
pub trait ValueInto<Dst> {
    /// The error type produced by a failed conversion.
    type Err: Error;
    
    /// Convert the subject into an exactly equivalent representation.
    fn value_into(self) -> Result<Dst, Self::Err>;
}

impl<Src, Dst> ValueInto<Dst> for Src where Dst: ValueFrom<Src> {
    type Err = Dst::Err;
    fn value_into(self) -> Result<Dst, Self::Err> {
        ValueFrom::value_from(self)
    }
}

/**
This extension trait exists to simplify using various conversions.

If there is more than one implementation for a given type/trait pair, a simple call to `*_into` may not be uniquely resolvable.  Due to the position of the type parameter (on the trait itself), it is cumbersome to specify the destination type.  A similar problem exists for approximation schemes.

See also the [`ConvAsUtil`](./trait.ConvAsUtil.html) trait.

> **Note**: There appears to be a bug in `rustdoc`'s output.  This trait is implemented *for all* types, though the methods are only available for types where the appropriate conversions are defined.
*/
pub trait ConvUtil {
    /// Approximate the subject to a given type with the default scheme.
    fn approx_as<Dst>(self) -> Result<Dst, Self::Err>
    where Self: Sized + ApproxInto<Dst> {
        self.approx_into()
    }

    /// Approximate the subject to a given type with a specific scheme.
    fn approx_as_by<Dst, Scheme>(self) -> Result<Dst, Self::Err>
    where
        Self: Sized + ApproxInto<Dst, Scheme>,
        Scheme: ApproxScheme,
    {
        self.approx_into()
    }

    /// Convert the subject to a given type.
    fn into_as<Dst>(self) -> Dst
    where Self: Sized + Into<Dst> {
        self.into()
    }

    /// Attempt to convert the subject to a given type.
    fn try_as<Dst>(self) -> Result<Dst, Self::Err>
    where Self: Sized + TryInto<Dst> {
        self.try_into()
    }

    /// Attempt a value conversion of the subject to a given type.
    fn value_as<Dst>(self) -> Result<Dst, Self::Err>
    where Self: Sized + ValueInto<Dst> {
        self.value_into()
    }
}

impl<T> ConvUtil for T {}

/**
This extension trait exists to simplify using various conversions.

If there is more than one `ApproxFrom` implementation for a given type, a simple call to `approx_into` may not be uniquely resolvable.  Due to the position of the scheme parameter (on the trait itself), it is cumbersome to specify which scheme you wanted.

The destination type is inferred from context.

See also the [`ConvUtil`](./trait.ConvUtil.html) trait.

> **Note**: There appears to be a bug in `rustdoc`'s output.  This trait is implemented *for all* types, though the methods are only available for types where the appropriate conversions are defined.
*/
pub trait ConvAsUtil<Dst> {
    /// Approximate the subject with the default scheme.
    fn approx(self) -> Result<Dst, Self::Err>
    where Self: Sized + ApproxInto<Dst> {
        self.approx_into()
    }

    /// Approximate the subject with a specific scheme.
    fn approx_by<Scheme>(self) -> Result<Dst, Self::Err>
    where
        Self: Sized + ApproxInto<Dst, Scheme>,
        Scheme: ApproxScheme,
    {
        self.approx_into()
    }
}

impl<T, Dst> ConvAsUtil<Dst> for T {}
