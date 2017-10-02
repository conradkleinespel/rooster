
extern crate x11;
extern crate xcb;
extern crate gl;
extern crate libc;

use xcb::dri2;

use x11::xlib;
use x11::glx::*;

use std::ptr::null_mut;
use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_void};


const GLX_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
const GLX_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;

type GlXCreateContextAttribsARBProc =
    unsafe extern "C" fn (dpy: *mut xlib::Display, fbc: GLXFBConfig,
            share_context: GLXContext, direct: xlib::Bool,
            attribs: *const c_int) -> GLXContext;


unsafe fn load_gl_func (name: &str) -> *mut c_void {
    let cname = CString::new(name).unwrap();
    let ptr: *mut c_void = std::mem::transmute(glXGetProcAddress(
            cname.as_ptr() as *const u8
    ));
    if ptr.is_null() {
        panic!("could not load {}", name);
    }
    ptr
}

fn check_glx_extension(glx_exts: &str, ext_name: &str) -> bool {
    for glx_ext in glx_exts.split(" ") {
        if glx_ext == ext_name {
            return true;
        }
    }
    false
}

static mut ctx_error_occurred: bool = false;
unsafe extern "C" fn ctx_error_handler(
        _dpy: *mut xlib::Display,
        _ev: *mut xlib::XErrorEvent) -> i32 {
    ctx_error_occurred = true;
    0
}


unsafe fn check_gl_error() {
    let err = gl::GetError();
    if err != gl::NO_ERROR {
        println!("got gl error {}", err);
    }
}

// returns the glx version in a decimal form
// eg. 1.3  => 13
fn glx_dec_version(dpy: *mut xlib::Display) -> i32 {
    let mut maj: c_int = 0;
    let mut min: c_int = 0;
    unsafe {
        if glXQueryVersion(dpy,
                &mut maj as *mut c_int,
                &mut min as *mut c_int) == 0 {
            panic!("cannot get glx version");
        }
    }
    (maj*10 + min) as i32
}


fn get_glxfbconfig(dpy: *mut xlib::Display, screen_num: i32,
        visual_attribs: &[i32]) -> GLXFBConfig {
    unsafe {
        let mut fbcount: c_int = 0;
        let fbcs = glXChooseFBConfig(dpy, screen_num,
                visual_attribs.as_ptr(),
                &mut fbcount as *mut c_int);

        if fbcount == 0 {
            panic!("could not find compatible fb config");
        }
        // we pick the first from the list
        let fbc = *fbcs;
        xlib::XFree(fbcs as *mut c_void);
        fbc
    }
}


fn main() { unsafe {
    let (conn, screen_num) = xcb::Connection::connect_with_xlib_display().unwrap();
    conn.set_event_queue_owner(xcb::EventQueueOwner::Xcb);

    if glx_dec_version(conn.get_raw_dpy()) < 13 {
        panic!("glx-1.3 is not supported");
    }

    let fbc = get_glxfbconfig(conn.get_raw_dpy(), screen_num, &[
            GLX_X_RENDERABLE    , 1,
            GLX_DRAWABLE_TYPE   , GLX_WINDOW_BIT,
            GLX_RENDER_TYPE     , GLX_RGBA_BIT,
            GLX_X_VISUAL_TYPE   , GLX_TRUE_COLOR,
            GLX_RED_SIZE        , 8,
            GLX_GREEN_SIZE      , 8,
            GLX_BLUE_SIZE       , 8,
            GLX_ALPHA_SIZE      , 8,
            GLX_DEPTH_SIZE      , 24,
            GLX_STENCIL_SIZE    , 8,
            GLX_DOUBLEBUFFER    , 1,
            0
    ]);

    let vi: *const xlib::XVisualInfo =
            glXGetVisualFromFBConfig(conn.get_raw_dpy(), fbc);

    let dri2_ev = {
        conn.prefetch_extension_data(dri2::id());
        match conn.get_extension_data(dri2::id()) {
            None => { panic!("could not load dri2 extension") },
            Some(r) => { r.first_event() }
        }
    };

    let (wm_protocols, wm_delete_window) = {
        let pc = xcb::intern_atom(&conn, false, "WM_PROTOCOLS");
        let dwc = xcb::intern_atom(&conn, false, "WM_DELETE_WINDOW");

        let p = match pc.get_reply() {
            Ok(p) => p.atom(),
            Err(_) => panic!("could not load WM_PROTOCOLS atom")
        };
        let dw = match dwc.get_reply() {
            Ok(dw) => dw.atom(),
            Err(_) => panic!("could not load WM_DELETE_WINDOW atom")
        };
        (p, dw)
    };

    let setup = conn.get_setup();
    let screen = setup.roots().nth((*vi).screen as usize).unwrap();

    let cmap = conn.generate_id();
    let win = conn.generate_id();

    xcb::create_colormap(&conn, xcb::COLORMAP_ALLOC_NONE as u8,
            cmap, screen.root(), (*vi).visualid as u32);

    let cw_values = [
        (xcb::CW_BACK_PIXEL, screen.white_pixel()),
        (xcb::CW_BORDER_PIXEL, screen.black_pixel()),
        (xcb::CW_EVENT_MASK,
            xcb::EVENT_MASK_KEY_PRESS | xcb::EVENT_MASK_EXPOSURE),
        (xcb::CW_COLORMAP, cmap)
    ];

    xcb::create_window(&conn, (*vi).depth as u8, win, screen.root(), 0, 0, 640, 480,
            0, xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
            (*vi).visualid as u32, &cw_values);

    xlib::XFree(vi as *mut c_void);

    let title = "XCB OpenGL";
    xcb::change_property(&conn,
            xcb::PROP_MODE_REPLACE as u8,
            win,
            xcb::ATOM_WM_NAME,
            xcb::ATOM_STRING,
            8, title.as_bytes());

    let protocols = [wm_delete_window];
    xcb::change_property(&conn, xcb::PROP_MODE_REPLACE as u8,
            win, wm_protocols, xcb::ATOM_ATOM, 32, &protocols);

    xcb::map_window(&conn, win);
    conn.flush();
    xlib::XSync(conn.get_raw_dpy(), xlib::False);

    let glx_exts = CStr::from_ptr(
        glXQueryExtensionsString(conn.get_raw_dpy(), screen_num))
        .to_str().unwrap();

    if !check_glx_extension(&glx_exts, "GLX_ARB_create_context") {
        panic!("could not find GLX extension GLX_ARB_create_context");
    }

    // with glx, no need of a current context is needed to load symbols
    // otherwise we would need to create a temporary legacy GL context
    // for loading symbols (at least glXCreateContextAttribsARB)
    let glx_create_context_attribs: GlXCreateContextAttribsARBProc =
        std::mem::transmute(load_gl_func("glXCreateContextAttribsARB"));

    // loading all other symbols
    gl::load_with(|n| load_gl_func(&n));

    if !gl::GenVertexArrays::is_loaded() {
        panic!("no GL3 support available!");
    }

    // installing an event handler to check if error is generated
    ctx_error_occurred = false;
    let old_handler = xlib::XSetErrorHandler(Some(ctx_error_handler));

    let context_attribs: [c_int; 5] = [
        GLX_CONTEXT_MAJOR_VERSION_ARB as c_int, 3,
        GLX_CONTEXT_MINOR_VERSION_ARB as c_int, 0,
        0
    ];
    let ctx = glx_create_context_attribs(conn.get_raw_dpy(), fbc, null_mut(),
            xlib::True, &context_attribs[0] as *const c_int);

    conn.flush();
    xlib::XSync(conn.get_raw_dpy(), xlib::False);
    xlib::XSetErrorHandler(std::mem::transmute(old_handler));

    if ctx.is_null() || ctx_error_occurred {
        panic!("error when creating gl-3.0 context");
    }

    if glXIsDirect(conn.get_raw_dpy(), ctx) == 0 {
        panic!("obtained indirect rendering context")
    }

    loop {
        if let Some(ev) = conn.wait_for_event() {
            let ev_type = ev.response_type() & !0x80;
            match ev_type {
                xcb::EXPOSE => {
                    glXMakeCurrent(conn.get_raw_dpy(), win as xlib::XID, ctx);
                    gl::ClearColor(0.5f32, 0.5f32, 1.0f32, 1.0f32);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::Flush();
                    check_gl_error();
                    glXSwapBuffers(conn.get_raw_dpy(), win as xlib::XID);
                    glXMakeCurrent(conn.get_raw_dpy(), 0, null_mut());
                },
                xcb::KEY_PRESS => {
                    break;
                },
                xcb::CLIENT_MESSAGE => {
                    let cmev = xcb::cast_event::<xcb::ClientMessageEvent>(&ev);
                    if cmev.type_() == wm_protocols && cmev.format() == 32 {
                        let protocol = cmev.data().data32()[0];
                        if protocol == wm_delete_window {
                            break;
                        }
                    }
                },
                _ => {
                    // following stuff is not obvious at all, but is necessary
                    // to handle GL when XCB owns the event queue
                    if ev_type == dri2_ev || ev_type == dri2_ev+1 {
                        // these are libgl dri2 event that need special handling
                        // see https://bugs.freedesktop.org/show_bug.cgi?id=35945#c4
                        // and mailing thread starting here:
                        // http://lists.freedesktop.org/archives/xcb/2015-November/010556.html

                        if let Some(proc_) =
                                xlib::XESetWireToEvent(conn.get_raw_dpy(),
                                        ev_type as i32, None) {
                            xlib::XESetWireToEvent(conn.get_raw_dpy(),
                                    ev_type as i32, Some(proc_));
                            let raw_ev = ev.ptr;
                            (*raw_ev).sequence =
                                xlib::XLastKnownRequestProcessed(
                                        conn.get_raw_dpy()) as u16;
                            let mut dummy: xlib::XEvent = std::mem::zeroed();
                            proc_(conn.get_raw_dpy(),
                                &mut dummy as *mut xlib::XEvent,
                                raw_ev as *mut xlib::xEvent);
                        }

                    }
                }
            }
            conn.flush();
        }
        else {
            break;
        }
    }

    // only to make sure that rs_client generate correct names for DRI2
    // (used to be "*_DRI_2_*")
    // should be in a "compile tests" section instead of example
    let _ = xcb::ffi::dri2::XCB_DRI2_ATTACHMENT_BUFFER_ACCUM;

    glXDestroyContext(conn.get_raw_dpy(), ctx);

    xcb::unmap_window(&conn, win);
    xcb::destroy_window(&conn, win);
    xcb::free_colormap(&conn, cmap);
    conn.flush();
}}
