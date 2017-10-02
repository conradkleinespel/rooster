extern crate xcb;

use xcb::randr;

fn main() {
    let (con, screen_num) = xcb::Connection::connect(None).unwrap();
    let screen = con.get_setup().roots().nth(screen_num as usize).unwrap();

    let randr_base = con.get_extension_data(&mut randr::id()).unwrap().first_event();
    let _ = randr::select_input(&con, screen.root(), randr::NOTIFY_MASK_CRTC_CHANGE as u16)
        .request_check();

    loop {
        con.flush();
        let event = con.wait_for_event().unwrap();

        if event.response_type() == randr_base + randr::NOTIFY {
            let ev: &randr::NotifyEvent = xcb::cast_event(&event);
            let d = ev.u().cc();
            println!("received CRTC_NOTIFY event:\n\
                     \ttimestamp: {}, window: {}, crtc: {}, mode: {}, rotation: {}\n\
                     \tx: {}, y: {}, width: {}, height: {}",
                     d.timestamp(), d.window(), d.crtc(), d.mode(), d.rotation(),
                     d.x(), d.y(), d.width(), d.height());
        }
    }
}
