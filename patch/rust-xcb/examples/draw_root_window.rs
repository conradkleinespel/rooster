
extern crate xcb;

use std::thread;
use std::time::Duration;


fn main() {
  let rectangles: &[xcb::Rectangle] = &[
      xcb::Rectangle::new(200, 200, 400, 400)
  ];

  let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
  let setup = conn.get_setup();
  let screen = setup.roots().nth(screen_num as usize).unwrap();

  let gc = conn.generate_id();
  
  xcb::create_gc(&conn, gc, screen.root(), &[
      (xcb::GC_FUNCTION, xcb::GX_XOR),
      (xcb::GC_FOREGROUND, screen.white_pixel()),
      (xcb::GC_BACKGROUND, screen.black_pixel()),
      (xcb::GC_LINE_WIDTH, 1),
      (xcb::GC_LINE_STYLE, xcb::LINE_STYLE_ON_OFF_DASH),
      (xcb::GC_GRAPHICS_EXPOSURES, 0)
  ]);

  //loop {
    xcb::poly_rectangle(&conn, screen.root(), gc, &rectangles);
    xcb::map_window(&conn, screen.root());
    conn.flush();
  //}
  thread::sleep(Duration::from_secs(5));
}