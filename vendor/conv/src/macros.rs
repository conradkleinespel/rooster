/*!
This module provides convenience macros to help with implementing the conversion traits.

# `TryFrom!`

```ignore
macro_rules! TryFrom {
    (($target:ty) $enum:item) => { ... };
}
```

This macro attempts to derive an implementation of the [`TryFrom`](../trait.TryFrom.html) trait.  Specifically, it supports `enum`s consisting entirely of unitary variants, with or without explicit values.  The source type can be any integer type which the variants of the enumeration can be explicitly cast to (*i.e.* using `as`).

If a conversion fails (due to there being no matching variant for the specified integer value `src`), then the conversion returns `Err(Unrepresentable(src))` (see [`Unrepresentable`](../errors/struct.Unrepresentable.html)).

It is compatible with the [`custom_derive!`](https://crates.io/crates/custom_derive) macro.

## Example

Using `custom_derive!`:

```
#[macro_use] extern crate conv;
#[macro_use] extern crate custom_derive;

custom_derive! {
    #[derive(Debug, PartialEq, TryFrom(i32))]
    enum Colours {
        Red = 0,
        Green = 5,
        Blue
    }
}

fn main() {
    use conv::{TryFrom, Unrepresentable};

    assert_eq!(Colours::try_from(0), Ok(Colours::Red));
    assert_eq!(Colours::try_from(1), Err(Unrepresentable(1)));
    assert_eq!(Colours::try_from(5), Ok(Colours::Green));
    assert_eq!(Colours::try_from(6), Ok(Colours::Blue));
    assert_eq!(Colours::try_from(7), Err(Unrepresentable(7)));
}
```

The above is equivalent to the following:

```
#[macro_use] extern crate conv;

#[derive(Debug, PartialEq)]
enum Colours {
    Red = 0,
    Green = 5,
    Blue
}

TryFrom! { (i32) enum Colours {
    Red = 0,
    Green = 5,
    Blue
} }
# fn main() {}
```
*/

/**
See the documentation for the [`macros`](./macros/index.html#tryfrom!) module for details.
*/
#[macro_export]
macro_rules! TryFrom {
    (($prim:ty) $(pub)* enum $name:ident { $($body:tt)* }) => {
        TryFrom! {
            @collect_variants ($name, $prim),
            ($($body)*,) -> ()
        }
    };

    (
        @collect_variants ($name:ident, $prim:ty),
        ($(,)*) -> ($($var_names:ident,)*)
    ) => {
        impl $crate::TryFrom<$prim> for $name {
            type Err = $crate::errors::Unrepresentable<$prim>;
            fn try_from(src: $prim) -> Result<$name, Self::Err> {
                $(
                    if src == $name::$var_names as $prim {
                        return Ok($name::$var_names);
                    }
                )*
                Err($crate::errors::Unrepresentable(src))
            }
        }
    };

    (
        @collect_variants $fixed:tt,
        (#[$_attr:meta] $($tail:tt)*) -> $var_names:tt
    ) => {
        TryFrom! {
            @skip_meta $fixed,
            ($($tail)*) -> $var_names
        }
    };

    (
        @collect_variants $fixed:tt,
        ($var:ident $(= $_val:expr)*, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        TryFrom! {
            @collect_variants $fixed,
            ($($tail)*) -> ($($var_names)* $var,)
        }
    };

    (
        @collect_variants ($name:ident),
        ($var:ident $_struct:tt, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        const _error: () = concat!(
            "cannot derive TryFrom for ",
            stringify!($name),
            ", due to non-unitary variant ",
            stringify!($var),
            "."
        );
    };
    
    (
        @skip_meta $fixed:tt,
        (#[$_attr:meta] $($tail:tt)*) -> $var_names:tt
    ) => {
        TryFrom! {
            @skip_meta $fixed,
            ($($tail)*) -> $var_names
        }
    };

    (
        @skip_meta $fixed:tt,
        ($var:ident $($tail:tt)*) -> $var_names:tt
    ) => {
        TryFrom! {
            @collect_variants $fixed,
            ($var $($tail)*) -> $var_names
        }
    };
}
