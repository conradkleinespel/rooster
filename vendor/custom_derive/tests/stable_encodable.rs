/*
Copyright ⓒ 2015 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate custom_derive;
extern crate rustc_serialize;

macro_rules! StableEncodable {
    (
        () $(pub)* enum $name:ident < $($tail:tt)*
    ) => {
        StableEncodable! {
            @extract_gen_args (enum $name),
            ($($tail)*)
            -> bounds(), ty_clss(where)
        }
    };

    (
        () $(pub)* enum $name:ident { $($body:tt)* }
    ) => {
        StableEncodable! {
            @impl enum $name,
            bounds(),
            ty_clss(),
            { $($body)* }
        }
    };

    (
        () $(pub)* struct $name:ident { $($body:tt)* }
    ) => {
        StableEncodable! {
            @impl struct $name,
            bounds(),
            ty_clss(),
            { $($body)* }
        }
    };

    (
        () $(pub)* struct $name:ident < $($tail:tt)*
    ) => {
        StableEncodable! {
            @extract_gen_args (struct $name),
            ($($tail)*)
            -> bounds(), ty_clss(where)
        }
    };

    (
        @impl enum $name:ident,
        bounds($($bounds:tt)*),
        ty_clss($($ty_clss:tt)*),
        { $($body:tt)* }
    ) => {
        StableEncodable! {
            @parse_variants (enum $name, bounds($($bounds)*), ty_clss($($ty_clss)*)),
            0usize, ($($body)*,) -> ()
        }
    };

    (
        @impl struct $name:ident,
        bounds($($bounds:tt)*),
        ty_clss($($ty_clss:tt)*),
        { $($fnames:ident: $_ftys:ty),* $(,)* }
    ) => {
        StableEncodable! {
            @as_item
            impl<$($bounds)*> rustc_serialize::Encodable for $name<$($bounds)*>
            $($ty_clss)* {
                fn encode<StableEncodableEncoder>(
                    &self,
                    s: &mut StableEncodableEncoder
                ) -> Result<(), StableEncodableEncoder::Error>
                where StableEncodableEncoder: rustc_serialize::Encoder {
                    const NUM_FIELDS: usize = StableEncodable!(@count_tts $($fnames)*);
                    try!(s.emit_struct(stringify!($name), NUM_FIELDS, |s| {
                        // Poor man's enumerate!($($fnames)):
                        let mut idx = 0;
                        $(
                            try!(s.emit_struct_field(stringify!($fnames), idx, |s| {
                                self.$fnames.encode(s)
                            }));
                            idx += 1;
                        )*
                        let _ = idx;
                        Ok(())
                    }));
                    Ok(())
                }
            }
        }
    };

    (@as_item $i:item) => {$i};

    (
        @extract_gen_args ($kind:ident $name:ident),
        (> { $($tail:tt)* })
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @impl $kind $name,
            bounds($($bounds)*),
            ty_clss($($ty_clss)*),
            { $($tail)* }
        }
    };

    (
        @extract_gen_args $fixed:tt,
        ($ty_name:ident: $($tail)*)
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @skip_inline_bound $fixed,
            ($($tail)*)
            -> bounds($($bounds)* $ty_name:),
               ty_clss($($ty_clss)* $ty_name: ::rustc_serialize::Encodable,)
        }
    };

    (
        @extract_gen_args $fixed:tt,
        ($ty_name:ident $($tail:tt)*)
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @extract_gen_args $fixed,
            ($($tail)*)
            -> bounds($($bounds)* $ty_name),
               ty_clss($($ty_clss)* $ty_name: ::rustc_serialize::Encodable,)
        }
    };

    (
        @extract_gen_args $fixed:tt,
        (, $($tail:tt)*)
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @extract_gen_args $fixed,
            ($($tail)*)
            -> bounds($($bounds)* ,), ty_clss($($ty_clss)*)
        }
    };

    (
        @extract_gen_args $fixed:tt,
        ($lt:tt $($tail:tt)*)
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @extract_gen_args $fixed,
            ($($tail)*)
            -> bounds($($bounds)* $lt), ty_clss($($ty_clss)*)
        }
    };

    (
        @skip_inline_bound $fixed:tt,
        (, $($tail:tt)*)
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @extract_gen_args $fixed,
            ($($tail)*)
            -> bounds($($bounds)* ,), ty_clss($($ty_clss)*)
        }
    };

    (
        @skip_inline_bound $fixed:tt,
        (> { $($tail:tt)* })
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @impl $fixed,
            bounds($($bounds)*),
            ty_clss($($ty_clss)*),
            { $($tail)* }
        }
    };

    (
        @parse_variants (enum $name:ident, bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)),
        $_id:expr, ($(,)*) -> ($($variants:tt)*)
    ) => {
        StableEncodable! {
            @as_item
            impl<$($bounds)*> rustc_serialize::Encodable for $name<$($bounds)*>
            $($ty_clss)* {
                fn encode<StableEncodableEncoder>(
                    &self,
                    s: &mut StableEncodableEncoder)
                -> Result<(), StableEncodableEncoder::Error>
                where StableEncodableEncoder: rustc_serialize::Encoder {
                    s.emit_enum(stringify!($name), |s| {
                        $(
                            StableEncodable!(@encode_variant $name, $variants, self, s);
                        )*
                        unreachable!();
                    })
                }
            }
        }
    };

    (
        @parse_variants $fixed:tt,
        $id:expr, ($var_name:ident, $($tail:tt)*) -> ($($variants:tt)*)
    ) => {
        StableEncodable! {
            @parse_variants $fixed,
            ($id + 1usize), ($($tail)*) -> ($($variants)* ($var_name, $id))
        }
    };

    (
        @parse_variants $fixed:tt,
        $id:expr, ($var_name:ident($(,)*), $($tail:tt)*) -> ($($variants:tt)*)
    ) => {
        StableEncodable! {
            @parse_variants $fixed,
            ($id + 1usize), ($($tail)*) -> ($($variants)*
                ($var_name, $id))
        }
    };

    (
        @parse_variants $fixed:tt,
        $id:expr, ($var_name:ident($_vta:ty), $($tail:tt)*) -> ($($variants:tt)*)
    ) => {
        StableEncodable! {
            @parse_variants $fixed,
            ($id + 1usize), ($($tail)*) -> ($($variants)*
                ($var_name, $id, (a)))
        }
    };

    (
        @parse_variants $fixed:tt,
        $id:expr, ($var_name:ident($_vta:ty, $_vtb:ty), $($tail:tt)*) -> ($($variants:tt)*)
    ) => {
        StableEncodable! {
            @parse_variants $fixed,
            ($id + 1usize), ($($tail)*) -> ($($variants)*
                ($var_name, $id, (a, b)))
        }
    };

    (
        @parse_variants $fixed:tt,
        $id:expr, ($var_name:ident($_vta:ty, $_vtb:ty, $_vtc:ty), $($tail:tt)*) -> ($($variants:tt)*)
    ) => {
        StableEncodable! {
            @parse_variants $fixed,
            ($id + 1usize), ($($tail)*) -> ($($variants)*
                ($var_name, $id, (a, b, c)))
        }
    };

    (
        @parse_variants $fixed:tt,
        $id:expr, ($var_name:ident { $($vfn:ident: $_vft:ty),* $(,)* }, $($tail:tt)*) -> ($($variants:tt)*)
    ) => {
        StableEncodable! {
            @parse_variants $fixed,
            ($id + 1usize), ($($tail)*) -> ($($variants)*
                ($var_name, $id, {$($vfn),*}))
        }
    };

    (
        @encode_variant $name:ident,
        ($var_name:ident, $var_id:expr),
        $self_:expr, $s:ident
    ) => {
        {
            if let $name::$var_name = *$self_ {
                return $s.emit_enum_variant(stringify!($var_name), $var_id, 0, |_| Ok(()));
            }
        }
    };

    (
        @encode_variant $name:ident,
        ($var_name:ident, $var_id:expr, ($($tup_elems:ident),*)),
        $self_:expr, $s:ident
    ) => {
        {
            if let $name::$var_name($(ref $tup_elems),*) = *$self_ {
                return $s.emit_enum_variant(
                    stringify!($var_name),
                    $var_id,
                    StableEncodable!(@count_tts $($tup_elems)*),
                    |s| {
                        let mut idx = 0;
                        $(
                            try!(s.emit_enum_variant_arg(idx, |s| $tup_elems.encode(s)));
                            idx += 1;
                        )*
                        let _ = idx;
                        Ok(())
                    }
                );
            }
        }
    };

    (
        @encode_variant $name:ident,
        ($var_name:ident, $var_id:expr, {$($str_fields:ident),*}),
        $self_:expr, $s:ident
    ) => {
        {
            if let $name::$var_name { $(ref $str_fields),* } = *$self_ {
                return $s.emit_enum_struct_variant(
                    stringify!($var_name),
                    $var_id,
                    StableEncodable!(@count_tts $($str_fields)*),
                    |s| {
                        let mut idx = 0;
                        $(
                            try!(s.emit_enum_struct_variant_field(
                                stringify!($str_fields),
                                idx,
                                |s| $str_fields.encode(s)
                            ));
                            idx += 1;
                        )*
                        let _ = idx;
                        Ok(())
                    }
                );
            }
        }
    };

    (@count_tts) => {0usize};
    (@count_tts $_tt:tt $($tail:tt)*) => {1usize + StableEncodable!(@count_tts $($tail)*)};
}

custom_derive! {
    #[derive(Debug, StableEncodable)]
    struct LazyEg<A> { a: A, b: i32, c: (u8, u8, u8) }
}

custom_derive! {
    #[derive(Clone, StableEncodable)]
    enum Wonky<S> { Flim, Flam, Flom(i32), Bees { say: S } }
}

#[test]
fn test_stable_encodable() {
    macro_rules! json {
        ($e:expr) => (rustc_serialize::json::encode(&$e).unwrap());
    }

    let lazy_eg = LazyEg {
        a: String::from("Oh hai!"),
        b: 42,
        c: (1, 3, 0),
    };
    assert_eq!(&*json!(lazy_eg), r#"{"a":"Oh hai!","b":42,"c":[1,3,0]}"#);

    assert_eq!(&*json!(Wonky::Flim::<()>), r#""Flim""#);
    assert_eq!(&*json!(Wonky::Flam::<()>), r#""Flam""#);
    assert_eq!(&*json!(Wonky::Flom::<()>(42)), r#"{"variant":"Flom","fields":[42]}"#);
    assert_eq!(&*json!(Wonky::Bees{say:"aaaaah!"}), r#"{"variant":"Bees","fields":["aaaaah!"]}"#);
}
