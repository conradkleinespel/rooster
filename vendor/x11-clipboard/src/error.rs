error_chain!{
    foreign_links {
        Io(::std::io::Error);
        Utf8(::std::string::FromUtf8Error);
        Set(::std::sync::mpsc::SendError<::xcb::Atom>);
    }

    errors {
        Lock {
            description("store lock poison")
        }
        Timeout {
            description("load selection timeout")
        }
        SetOwner {
            description("set selection owner fail")
        }
        XcbConn(err: ::xcb::base::ConnError) {
            description("xcb connection error")
            display("xcb connection error: {:?}", err)
        }
        XcbGeneric(err: ::xcb::base::GenericError) {
            description("xcb generic error")
            display("xcb generic error code: {}", err.error_code())
        }
    }
}

macro_rules! err {
    ( $kind:ident ) => {
        $crate::error::Error::from($crate::error::ErrorKind::$kind)
    };
    ( $kind:ident, $err:expr ) => {
        $crate::error::Error::from($crate::error::ErrorKind::$kind($err))
    };
}
