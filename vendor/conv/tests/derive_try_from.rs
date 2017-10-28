#[macro_use] extern crate conv;

use conv::{TryFrom, Unrepresentable};

#[derive(Debug, PartialEq)]
enum Get { Up, Down, AllAround }

TryFrom! { (u8)
    enum Get {
        Up,
        /// And
        Down,
        /** And */
        AllAround
    }
}

#[derive(Debug, PartialEq)]
enum GottaGo { GetAway, Fast = 9000, Faster = 9001 }

TryFrom! { (u16)
    enum GottaGo {
        GetAway,
        Fast = 9000,
        /// This show was stupid.
        Faster = 9001
    }
}

#[test]
fn test_try_from() {
    assert_eq!(Get::try_from(0u8), Ok(Get::Up));
    assert_eq!(Get::try_from(1u8), Ok(Get::Down));
    assert_eq!(Get::try_from(2u8), Ok(Get::AllAround));
    assert_eq!(Get::try_from(3u8), Err(Unrepresentable(3u8)));

    assert_eq!(GottaGo::try_from(0u16), Ok(GottaGo::GetAway));
    assert_eq!(GottaGo::try_from(1u16), Err(Unrepresentable(1u16)));
    assert_eq!(GottaGo::try_from(2u16), Err(Unrepresentable(2u16)));
    assert_eq!(GottaGo::try_from(3u16), Err(Unrepresentable(3u16)));
    assert_eq!(GottaGo::try_from(8999u16), Err(Unrepresentable(8999u16)));
    assert_eq!(GottaGo::try_from(9000u16), Ok(GottaGo::Fast));
    assert_eq!(GottaGo::try_from(9001u16), Ok(GottaGo::Faster));
    assert_eq!(GottaGo::try_from(9002u16), Err(Unrepresentable(9002u16)));
}
