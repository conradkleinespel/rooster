
extern crate xcb;
extern crate libc;

use xcb::ffi::*;
use xcb::ffi::randr::*;

use std::ptr;
use libc::{free, c_void};

fn main() {
    unsafe {
        //Open connection to X server
        let conn = xcb_connect(ptr::null(), ptr::null_mut());

        //Get the first X screen
        let first_screen = xcb_setup_roots_iterator(xcb_get_setup(conn)).data;

        //Generate ID for the X window
        let window_dummy = xcb_generate_id(conn);

        //Create dummy X window
        xcb_create_window(conn, 0, window_dummy, (*first_screen).root,
                          0, 0, 1, 1, 0, 0, 0, 0, ptr::null_mut());

        //Flush pending requests to the X server
        xcb_flush(conn);

        //Send a request for screen resources to the X server
        let screen_res_cookie = xcb_randr_get_screen_resources(conn,
                                                         window_dummy);

        //Receive reply from X server
        let screen_res_reply = xcb_randr_get_screen_resources_reply(conn,
                         screen_res_cookie, ptr::null_mut());

        if screen_res_reply.is_null() { panic!(); }

        //Get a pointer to the first CRTC and number of CRTCs
        //It is crucial to notice that you are in fact getting
        //an array with firstCRTC being the first element of
        //this array and crtcs_length - length of this array
        let crtcs_num = xcb_randr_get_screen_resources_crtcs_length(screen_res_reply);
        let first_crtc = xcb_randr_get_screen_resources_crtcs(screen_res_reply);

        //Array of requests to the X server for CRTC info
        let mut crtc_res_cookies = Vec::with_capacity(crtcs_num as usize);
        crtc_res_cookies.set_len(crtcs_num as usize);
        for i in 0..crtcs_num {
            crtc_res_cookies[i as usize] = xcb_randr_get_crtc_info(conn,
                      *first_crtc.offset(i as isize), 0);
        }

        for i in 0..crtcs_num {
            let reply = xcb_randr_get_crtc_info_reply(conn,
                    crtc_res_cookies[i as usize], ptr::null_mut());
            if reply.is_null() { continue; }
            {
                let reply = &*reply;
                if i != 0 { println!(""); }
                println!("CRTC[{}] INFO:", i);
                println!(" x-off\t: {}", reply.x);
                println!(" y-off\t: {}", reply.y);
                println!(" width\t: {}", reply.width);
                println!(" height\t: {}", reply.height);
            }
            free(reply as *mut c_void);
        }

        xcb_disconnect(conn);
    }
}
