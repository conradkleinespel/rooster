
extern crate xcb;
extern crate libc;

use xcb::xkb;

fn main() {

    let (conn, _) = xcb::Connection::connect(None).unwrap();

    conn.prefetch_extension_data(xkb::id());

    // generally useful to retrieve the first event from this
    // extension. event response_type will be set on this
    let _first_ev = match conn.get_extension_data(xkb::id()) {
        Some(r) => r.first_event(),
        None => { panic!("XKB extension not supported by X server!"); }
    };


    // we need at least xcb-xkb-1.0 to be available on client
    // machine
    {
        let cookie = xkb::use_extension(&conn, 1, 0);

        match cookie.get_reply() {
            Ok(r) => {
                if !r.supported() {
                    panic!("xkb-1.0 is not supported");
                }
            },
            Err(_) => {
                panic!("could not get xkb extension supported version");
            }
        };
    }

    // we now select what events we want to receive
    // such as map change, keyboard hotplug ...
    // note that key strokes are given directly by
    // the XCB_KEY_PRESS event from xproto, not by xkb
    {
        let map_parts =
            xkb::MAP_PART_KEY_TYPES |
            xkb::MAP_PART_KEY_SYMS |
            xkb::MAP_PART_MODIFIER_MAP |
            xkb::MAP_PART_EXPLICIT_COMPONENTS |
            xkb::MAP_PART_KEY_ACTIONS |
            xkb::MAP_PART_KEY_BEHAVIORS |
            xkb::MAP_PART_VIRTUAL_MODS |
            xkb::MAP_PART_VIRTUAL_MOD_MAP;

        let events =
            xkb::EVENT_TYPE_NEW_KEYBOARD_NOTIFY |
            xkb::EVENT_TYPE_MAP_NOTIFY |
            xkb::EVENT_TYPE_STATE_NOTIFY;

        let cookie = xkb::select_events_checked(&conn,
            xkb::ID_USE_CORE_KBD as u16,
            events as u16, 0, events as u16,
            map_parts as u16, map_parts as u16, None);


        cookie.request_check().expect("failed to select notify events from xcb xkb");

    }


    // proceed with create_window and event loop...


}
