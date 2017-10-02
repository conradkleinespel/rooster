extern crate xcb;

use std::iter::{Iterator};
use std::{thread, time};
use std::sync::Arc;

fn main() {
    let (conn, screen_num) = {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        (Arc::new(conn), screen_num)
    };
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let window = conn.generate_id();

    let values = [
        (xcb::CW_BACK_PIXEL, screen.black_pixel()),
        (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS |
            xcb::EVENT_MASK_STRUCTURE_NOTIFY | xcb::EVENT_MASK_PROPERTY_CHANGE),
    ];

    xcb::create_window(&conn,
        xcb::COPY_FROM_PARENT as u8,
        window,
        screen.root(),
        0, 0,
        320, 240,
        10,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &values);

    xcb::map_window(&conn, window);

    {
        let conn = conn.clone();
        thread::spawn(move || {
            let mut blink = false;
            loop {
                let title = if blink { "Basic Threaded Window ;-)" }
                else { "Basic Threaded Window :-)" };

                let c = xcb::change_property_checked(&conn, xcb::PROP_MODE_REPLACE as u8, window,
                        xcb::ATOM_WM_NAME, xcb::ATOM_STRING, 8, title.as_bytes());

                if conn.has_error().is_err() || c.request_check().is_err() {
                    break;
                }

                blink = !blink;
                thread::sleep(time::Duration::from_millis(500));
            }
        });
    }

    conn.flush();

    loop {
        let event = conn.wait_for_event();
        match event {
            None => { break; }
            Some(event) => {
                let r = event.response_type();
                if r == xcb::PROPERTY_NOTIFY as u8 {
                    let prop_notify: &xcb::PropertyNotifyEvent = xcb::cast_event(&event);
                    if prop_notify.atom() == xcb::ATOM_WM_NAME {
                        // retrieving title
                        let cookie = xcb::get_property(&conn, false, window, xcb::ATOM_WM_NAME,
                                xcb::ATOM_STRING, 0, 1024);
                        if let Ok(reply) = cookie.get_reply() {
                            println!("title changed to \"{}\"", std::str::from_utf8(reply.value()).unwrap());
                        }
                    }
                }
                else if r == xcb::KEY_PRESS as u8 {
                    let key_press: &xcb::KeyPressEvent = xcb::cast_event(&event);

                    println!("Key '{}' pressed", key_press.detail());

                    if key_press.detail() == 0x18 { // Q (on qwerty)
                        break;
                    }
                }
            }
        }
    }
}
