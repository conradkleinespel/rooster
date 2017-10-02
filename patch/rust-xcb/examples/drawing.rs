extern crate xcb;

fn main() {
    let points: &[xcb::Point] = &[
        xcb::Point::new(10, 10),
        xcb::Point::new(10, 20),
        xcb::Point::new(20, 10),
        xcb::Point::new(20, 20),
    ];
    let polyline: &[xcb::Point] = &[
        xcb::Point::new(50, 10 ),
        xcb::Point::new( 5, 20 ),     /* rest of points are relative */
        xcb::Point::new(25, -20),
        xcb::Point::new(10, 10 )
    ];
    let segments: &[xcb::Segment] = &[
        xcb::Segment::new(100, 10, 140, 30),
        xcb::Segment::new(110, 25, 130, 60)
    ];
    let rectangles: &[xcb::Rectangle] = &[
        xcb::Rectangle::new(10, 50, 40, 20),
        xcb::Rectangle::new(80, 50, 10, 40)
    ];
    let arcs: &[xcb::Arc] = &[
        xcb::Arc::new(10, 100, 60, 40, 0, 90 << 6),
        xcb::Arc::new(90, 100, 55, 40, 0, 270 << 6)
    ];


    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let foreground = conn.generate_id();

    xcb::create_gc(&conn, foreground, screen.root(), &[
            (xcb::GC_FOREGROUND, screen.black_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
    ]);

    let win = conn.generate_id();
    xcb::create_window(&conn,
        xcb::COPY_FROM_PARENT as u8,
        win,
        screen.root(),
        0, 0,
        150, 150,
        10,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(), &[
            (xcb::CW_BACK_PIXEL, screen.white_pixel()),
            (xcb::CW_EVENT_MASK,
             xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
        ]
    );
    xcb::map_window(&conn, win);
    conn.flush();


    loop {
        let event = conn.wait_for_event();
        match event {
            None => { break; }
            Some(event) => {
                let r = event.response_type() & !0x80;
                match r {
                    xcb::EXPOSE => {
                        /* We draw the points */
                        xcb::poly_point(&conn, xcb::COORD_MODE_ORIGIN as u8, win,
                            foreground, &points);

                        /* We draw the polygonal line */
                        xcb::poly_line(&conn, xcb::COORD_MODE_PREVIOUS as u8, win,
                            foreground, &polyline);

                        /* We draw the segements */
                        xcb::poly_segment(&conn, win, foreground, &segments);

                        /* We draw the rectangles */
                        xcb::poly_rectangle(&conn, win, foreground, &rectangles);

                        /* We draw the arcs */
                        xcb::poly_arc(&conn, win, foreground, &arcs);

                        /* We flush the request */
                        conn.flush();

                    },
                    xcb::KEY_PRESS => {
                        let key_press : &xcb::KeyPressEvent = xcb::cast_event(&event);
                        println!("Key '{}' pressed", key_press.detail());
                        break;
                    },
                    _ => {}
                }
            }
        }
    }
}
