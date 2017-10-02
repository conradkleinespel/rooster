
extern crate xcb;
extern crate libc;

use xcb::ffi::*;
use xcb::ffi::xkb::*;

use std::ptr::{null, null_mut};
use libc::{c_int, c_char, c_void};

fn main() {
    unsafe {

        let mut screen_num: c_int = 0;
        let c = xcb_connect(null(), &mut screen_num);
        if c.is_null() { panic!(); }

        // generally useful to retrieve the first event from this
        // extension. event response_type will be set on this
        let _first_ev = {
            xcb_prefetch_extension_data(c, &mut xcb_xkb_id);

            let reply = xcb_get_extension_data(c, &mut xcb_xkb_id);
            if reply.is_null() || (*reply).present == 0 {
                panic!("XKB extension not supported by X server");
            }

            (*reply).first_event
        };

        // we need at least xcb-xkb-1.0 to be available on client
        // machine
        {
            let cookie = xcb_xkb_use_extension(c, 1, 0);
            let reply = xcb_xkb_use_extension_reply(c, cookie, null_mut());
            if reply.is_null() {
                panic!("could not get xkb extension supported version");
            }
            if (*reply).supported == 0 {
                libc::free(reply as *mut c_void);
                panic!("xkb-1.0 is not supported");
            }
            libc::free(reply as *mut c_void);
        }

        // we now select what events we want to receive
        // such as map change, keyboard hotplug ...
        // note that key strokes are given directly by
        // the XCB_KEY_PRESS event from xproto, not by xkb
        {
            let map_parts =
                XCB_XKB_MAP_PART_KEY_TYPES |
                XCB_XKB_MAP_PART_KEY_SYMS |
                XCB_XKB_MAP_PART_MODIFIER_MAP |
                XCB_XKB_MAP_PART_EXPLICIT_COMPONENTS |
                XCB_XKB_MAP_PART_KEY_ACTIONS |
                XCB_XKB_MAP_PART_KEY_BEHAVIORS |
                XCB_XKB_MAP_PART_VIRTUAL_MODS |
                XCB_XKB_MAP_PART_VIRTUAL_MOD_MAP;

            let events =
                XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY |
                XCB_XKB_EVENT_TYPE_MAP_NOTIFY |
                XCB_XKB_EVENT_TYPE_STATE_NOTIFY;

            let cookie = xcb_xkb_select_events_checked(c,
                XCB_XKB_ID_USE_CORE_KBD as u16,
                events as u16, 0, events as u16,
                map_parts as u16, map_parts as u16, null());
            let err = xcb_request_check(c, cookie);
            if !err.is_null() {
                panic!("failed to select notify events from xcb xkb");
            }
        }


        // proceed with create_window and event loop...

    }
}

