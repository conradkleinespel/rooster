error_chain!{
    foreign_links {
        Io(::std::io::Error);
        Utf8(::std::string::FromUtf8Error);
        Set(::std::sync::mpsc::SendError<::xcb::Atom>);
        XcbConn(::xcb::base::ConnError);
        XcbGeneric(::xcb::base::GenericError);
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
