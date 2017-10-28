/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
**Note**: This crate has been superseded by `macro-attr`.

This crate provides a macro that enables the use of custom `derive` attributes.

To use it, make sure you link to the crate like so:

```rust
#[macro_use] extern crate custom_derive;
# macro_rules! Dummy { (() struct $name:ident;) => {}; }
# custom_derive! { #[derive(Clone, Dummy)] struct Foo; }
# fn main() { let _ = Foo; }
```

> **Note**: the `custom_derive!` macro itself is not documented, as the automatic documentation for it would be uselessly huge and incomprehensible.

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

* [Latest Release](https://crates.io/crates/custom_derive/)
* [Latest Docs](https://docs.rs/crate/custom_derive/)
* [Repository](https://github.com/DanielKeep/rust-custom-derive/tree/custom_derive-master/)

<span></span></div>

# Usage

The macro should be used to wrap an entire *single* `enum` or `struct` declaration, including its attributes (both `derive` and others).  All derivation attributes which the macro does *not* recognise will be assumed to be custom, and treated accordingly.

`custom_derive!` assumes that custom derivations are implemented as macros (of the same name).  For example, here is a simple derivation macro:

```rust
#[macro_use] extern crate custom_derive;

trait TypeName {
    fn type_name() -> &'static str;
}

trait ReprType {
    type Repr;
}

macro_rules! TypeName {
    (() $(pub)* enum $name:ident $($tail:tt)*) => { TypeName! { @impl $name } };
    (() $(pub)* struct $name:ident $($tail:tt)*) => { TypeName! { @impl $name } };

    (@impl $name:ident) => {
        impl TypeName for $name {
            fn type_name() -> &'static str { stringify!($name) }
        }
    };
}

macro_rules! TryFrom {
    (($repr:ty) $(pub)* enum $name:ident $($tail:tt)*) => {
        impl ReprType for $name {
            type Repr = $repr;
        }
    };
}

custom_derive! {
    #[allow(dead_code)]
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, TryFrom(u8), TypeName)]
    enum Foo { A, B }
}

fn main() {
    let foo = Foo::B;
    let v = foo as <Foo as ReprType>::Repr;
    let msg = format!("{}: {:?} ({:?})", Foo::type_name(), foo, v);
    assert_eq!(msg, "Foo: B (1)");
}
```

First, note that `custom_derive!` passes any arguments on the derivation attribute to the macro.  In the case of attributes *without* any arguments, `()` is passed instead.

Secondly, the macro is passed the entire item, *sans* attributes.  It is the derivation macro's job to parse the item correctly.

Third, each derivation macro is expected to result in zero or more items, not including the item itself.  As a result, it is *not* possible to mutate the item in any way, or attach additional attributes to it.

Finally, `@impl` is merely a trick to pack multiple, different functions into a single macro.  The sequence has no special meaning; it is simply *distinct* from the usual invocation syntax.
*/
#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
#[macro_export]
macro_rules! custom_derive {
    /*

    > **Convention**: a capture named `$fixed` is used for any part of a recursive rule that is needed in the terminal case, but is not actually being *used* for the recursive part.  This avoids having to constantly repeat the full capture pattern (and makes changing it easier).

    # Primary Invocation Forms

    These need to catch any valid form of struct or enum.

    */
    (
        $(#[$($attrs:tt)*])*
        enum $($it:tt)*
    ) => {
        custom_derive! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (enum $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        pub $($it:tt)*
    ) => {
        custom_derive! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (pub $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        struct $($it:tt)*
    ) => {
        custom_derive! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (struct $($it)*)
        }
    };

    /*

    # `@split_attrs`

    This is responsible for dividing all attributes on an item into two groups:

    - `#[derive(...)]`
    - Everything else.

    As part of this, it also explodes `#[derive(A, B(..), C, ...)]` into `A, B(..), C, ...`.  This is to simplify the next stage.

    */
    (
        @split_attrs
        (),
        $non_derives:tt,
        $derives:tt,
        $it:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            { $non_derives, $it },
            $derives,
            (),
            ()
        }
    };

    (
        @split_attrs
        (#[derive($($new_drv:ident $(($($new_drv_args:tt)*))*),* $(,)*)], $(#[$($attrs:tt)*],)*),
        $non_derives:tt,
        ($($derives:ident,)*),
        $it:tt
    ) => {
        custom_derive! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            ($($derives,)* $($new_drv $(($($new_drv_args)*))*,)*),
            $it
        }
    };

    (
        @split_attrs
        (#[$new_attr:meta], $(#[$($attrs:tt)*],)*),
        ($($non_derives:tt)*),
        $derives:tt,
        $it:tt
    ) => {
        custom_derive! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            ($($non_derives)* #[$new_attr],),
            $derives,
            $it
        }
    };

    /*

    # `@split_derive_attrs`

    This is responsible for taking the list of derivation attributes and splitting them into "built-in" and "custom" groups.

    The list of built-in derives currently supported is: Clone, Hash, RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Send, Sync, Copy.

    Anything not on that list is considered "custom".

    And yes, as far as I can see, we *have* to have a separate rule for each of those.  What I wouldn't give for an alternation pattern...
    */

    (@split_derive_attrs
        { ($(#[$($non_derives:tt)*],)*), ($($it:tt)*) },
        (), (), ($($user_drvs:tt)*)
    ) => {
        custom_derive! {
            @as_item
            $(#[$($non_derives)*])*
            $($it)*
        }

        custom_derive! {
            @expand_user_drvs
            ($($user_drvs)*), ($($it)*)
        }
    };

    (@split_derive_attrs
        { ($(#[$($non_derives:tt)*],)*), ($($it:tt)*) },
        (), ($($bi_drvs:ident,)+), ($($user_drvs:tt)*)
    ) => {
        custom_derive! {
            @as_item
            #[derive($($bi_drvs,)+)]
            $(#[$($non_derives)*])*
            $($it)*
        }

        custom_derive! {
            @expand_user_drvs
            ($($user_drvs)*), ($($it)*)
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Hash, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Hash,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Clone, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Clone,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (RustcEncodable, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* RustcEncodable,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (RustcDecodable, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* RustcDecodable,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (PartialEq, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* PartialEq,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Eq, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Eq,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (PartialOrd, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* PartialOrd,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Ord, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Ord,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Debug, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Debug,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Default, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Default,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Send ,$($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Send,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Sync, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Sync,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Copy, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Copy,), $user_drvs
        }
    };

    /*

    ## Custom Derivations

    Now we can handle the custom derivations.  There are two forms we care about: those *with* an argument, and those *without*.

    The *reason* we care is that, in order to simplify the derivation macros, we want to detect the argument-less case and generate an empty pair of parens.

    */
    (@split_derive_attrs
        $fixed:tt,
        ($new_user:ident, $($tail:tt)*), $bi_drvs:tt, ($($user_drvs:tt)*)
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, ($($user_drvs)* $new_user(),)
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        ($new_user:ident ($($new_user_args:tt)*), $($tail:tt)*), $bi_drvs:tt, ($($user_drvs:tt)*)
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, ($($user_drvs)* $new_user($($new_user_args)*),)
        }
    };

    /*

    # `@expand_user_drvs`

    Finally, we have a recursive rule for expanding user derivations.  This is basically just using the derivation name as a macro identifier.

    This *has* to be recursive because we need to expand two independent repetition sequences simultaneously, and this causes `macro_rules!` to throw a wobbly.  Don't want that.  So, recursive it is.

    */
    (@expand_user_drvs
        (), ($($it:tt)*)
    ) => {};

    (@expand_user_drvs
        ($user_drv:ident $arg:tt, $($tail:tt)*), ($($it:tt)*)
    ) => {
        $user_drv! { $arg $($it)* }
        custom_derive! {
            @expand_user_drvs
            ($($tail)*), ($($it)*)
        }
    };

    /*

    # Miscellaneous Rules

    */
    (@as_item $($i:item)*) => {$($i)*};
}
