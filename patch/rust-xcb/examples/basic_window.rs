extern crate xcb;

use std::iter::{Iterator};

fn main() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let window = conn.generate_id();

    let values = [
        (xcb::CW_BACK_PIXEL, screen.white_pixel()),
        (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
    ];

    xcb::create_window(&conn,
        xcb::COPY_FROM_PARENT as u8,
        window,
        screen.root(),
        0, 0,
        150, 150,
        10,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &values);

    xcb::map_window(&conn, window);

    let title = "Basic Window";
    // setting title
    xcb::change_property(&conn, xcb::PROP_MODE_REPLACE as u8, window,
            xcb::ATOM_WM_NAME, xcb::ATOM_STRING, 8, title.as_bytes());

    conn.flush();

    // retrieving title
    let cookie = xcb::get_property(&conn, false, window, xcb::ATOM_WM_NAME,
            xcb::ATOM_STRING, 0, 1024);
    if let Ok(reply) = cookie.get_reply() {
        assert_eq!(std::str::from_utf8(reply.value()).unwrap(), title);
    } else {
        panic!("could not retrieve window title!");
    }

    // retrieving a few atoms
    let (wm_state, wm_state_maxv, wm_state_maxh) = {
        let cook = xcb::intern_atom(&conn, true, "_NET_WM_STATE");
        let cook_maxv = xcb::intern_atom(&conn, true, "_NET_WM_STATE_MAXIMIZED_VERT");
        let cook_maxh = xcb::intern_atom(&conn, true, "_NET_WM_STATE_MAXIMIZED_HORZ");

        (cook.get_reply().unwrap().atom(),
            cook_maxv.get_reply().unwrap().atom(),
            cook_maxh.get_reply().unwrap().atom())
    };

    let mut maximized = false;

    loop {
        let event = conn.wait_for_event();
        match event {
            None => { break; }
            Some(event) => {
                let r = event.response_type();
                if r == xcb::KEY_PRESS as u8 {
                    let key_press : &xcb::KeyPressEvent = xcb::cast_event(&event);

                    println!("Key '{}' pressed", key_press.detail());

                    if key_press.detail() == 0x3a { // M (on qwerty)

                        // toggle maximized

                        // ClientMessageData is a memory safe untagged union
                        let data = xcb::ClientMessageData::from_data32([
                            if maximized { 0 } else { 1 },
                            wm_state_maxv, wm_state_maxh,
                            0, 0
                        ]);

                        let ev = xcb::ClientMessageEvent::new(32, window,
                            wm_state, data);

                        xcb::send_event(&conn, false, screen.root(),
                            xcb::EVENT_MASK_STRUCTURE_NOTIFY, &ev);

                        conn.flush();

                        maximized = !maximized;
                    }
                    else if key_press.detail() == 0x18 { // Q (on qwerty)
                        break;
                    }
                }
            }
        }
    }
}
