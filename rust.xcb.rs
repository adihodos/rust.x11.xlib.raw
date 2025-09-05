#![crate_name = "rust_xcb"]
#![allow(non_camel_case_types)]

#[repr(C)]
struct xcb_connection_t {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

type xcb_window_t = u32;
type xcb_colormap_t = u32;
type xcb_visualid_t = u32;
type xcb_keycode_t = u8;
type xcb_timestamp_t = u32;
type xcb_atom_t = u32;
type xcb_font_t = u32;
type xcb_gcontext_t = u32;
type xcb_drawable_t = u32;

const XCB_WINDOW_CLASS_INPUT_OUTPUT: i32 = 1;
const XCB_CW_BACK_PIXEL: u32 = 2u32;
const XCB_CW_EVENT_MASK: u32 = 2048;

const XCB_EVENT_MASK_KEY_PRESS: u32 = 1;
const XCB_EVENT_MASK_EXPOSURE: u32 = 32768;
const XCB_EVENT_MASK_STRUCTURE_NOTIFY: u32 = 131072;

const XCB_KEY_PRESS: u8 = 2;
const XCB_EXPOSE: u8 = 12;
const XCB_CLIENT_MESSAGE: u8 = 33;

const XCB_PROP_MODE_REPLACE: u8 = 0;

const XCB_ATOM_ATOM: xcb_atom_t = 4;
const XCB_ATOM_STRING: xcb_atom_t = 31;
const XCB_ATOM_WM_NAME: xcb_atom_t = 39;

const XCB_GC_FOREGROUND: u32 = 4;
const XCB_GC_BACKGROUND: u32 = 8;
const XCB_GC_LINE_WIDTH: u32 = 16;
const XCB_GC_GRAPHICS_EXPOSURES: u32 = 65536;

#[repr(C)]
#[derive(Copy, Clone)]
union xcb_client_message_data_t {
    data8: [u8; 20],
    data16: [u16; 10],
    data32: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct xcb_client_message_event_t {
    response_type: u8,
    format: u8,
    sequence: u16,
    window: xcb_window_t,
    type_: xcb_atom_t,
    data: xcb_client_message_data_t,
}

// const XKB_KEY_Escape: u8 = 0xff1b;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct xcb_screen_t {
    root: xcb_window_t,
    default_colormap: xcb_colormap_t,
    white_pixel: u32,
    black_pixel: u32,
    current_input_masks: u32,
    width_in_pixels: u16,
    height_in_pixels: u16,
    width_in_millimeters: u16,
    height_in_millimeters: u16,
    min_installed_maps: u16,
    max_installed_maps: u16,
    root_visual: xcb_visualid_t,
    backing_stores: u8,
    save_unders: u8,
    root_depth: u8,
    allowed_depths_len: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct xcb_setup_t {
    status: u8,
    pad0: u8,
    protocol_major_version: u16,
    protocol_minor_version: u16,
    length: u16,
    release_number: u32,
    resource_id_base: u32,
    resource_id_mask: u32,
    motion_buffer_size: u32,
    vendor_len: u16,
    maximum_request_length: u16,
    roots_len: u8,
    pixmap_formats_len: u8,
    image_byte_order: u8,
    bitmap_format_bit_order: u8,
    bitmap_format_scanline_unit: u8,
    bitmap_format_scanline_pad: u8,
    min_keycode: xcb_keycode_t,
    max_keycode: xcb_keycode_t,
    pad1: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct xcb_screen_iterator_t {
    data: *const xcb_screen_t,
    rem: i32,
    index: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct xcb_void_cookie_t {
    sequence: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct xcb_intern_atom_cookie_t {
    sequence: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct xcb_intern_atom_reply_t {
    response_type: u8,
    pad0: u8,
    sequence: u16,
    length: u32,
    atom: xcb_atom_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct xcb_key_press_event_t {
    response_type: u8,
    detail: xcb_keycode_t,
    sequence: u16,
    time: xcb_timestamp_t,
    root: xcb_window_t,
    event: xcb_window_t,
    child: xcb_window_t,
    root_x: i16,
    root_y: i16,
    event_x: i16,
    event_y: i16,
    state: u16,
    same_screen: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct xcb_generic_event_t {
    response_type: u8,
    pad0: u8,
    sequence: u16,
    pad: [u32; 7],
    full_sequence: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct xcb_generic_error_t {
    response_type: u8,
    error_code: u8,
    sequence: u16,
    resource_id: u32,
    minor_code: u16,
    major_code: u8,
    pad0: u8,
    pad: [u32; 5],
    full_sequence: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct xcb_rectangle_t {
    x: i16,
    y: i16,
    width: u16,
    height: u16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct xcb_arc_t {
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    angle1: i16,
    angle2: i16,
}

#[link(name = "xcb")]
unsafe extern "C" {
    fn xcb_connect(displayname: *const u8, screenp: *mut i32) -> *mut xcb_connection_t;
    fn xcb_get_file_descriptor(c: *mut xcb_connection_t) -> i32;
    fn xcb_disconnect(c: *mut xcb_connection_t);
    fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
    fn xcb_setup_roots_iterator(r: *const xcb_setup_t) -> xcb_screen_iterator_t;
    fn xcb_screen_next(i: *mut xcb_screen_iterator_t);
    fn xcb_generate_id(c: *mut xcb_connection_t) -> xcb_window_t;
    fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        _class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_wait_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> i32;
    fn xcb_intern_atom(
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const u8,
    ) -> xcb_intern_atom_cookie_t;
    fn xcb_intern_atom_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_intern_atom_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_intern_atom_reply_t;
    fn xcb_change_property(
        conn: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        data_len: u32,
        data: *const core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;

    fn xcb_create_gc_checked(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const core::ffi::c_void,
    ) -> xcb_void_cookie_t;

    fn xcb_change_gc(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const u8,
    ) -> xcb_void_cookie_t;

    fn xcb_poly_fill_rectangle(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;

    fn xcb_poly_rectangle(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;

    fn xcb_poly_arc(
        c: *const xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t;

}

mod libc {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union epoll_data_t {
        pub ptr: *mut core::ffi::c_void,
        pub fd: i32,
        pub u32_: u32,
        pub u64_: u64,
    }

    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    pub struct epoll_event {
        pub events: u32,
        pub data: epoll_data_t,
    }

    pub const EAGAIN: i32 = 11;

    pub const F_GETFL: i32 = 3;
    pub const F_SETFL: i32 = 4;
    pub const O_NONBLOCK: i32 = 04000;

    pub const EPOLL_CTL_ADD: i32 = 1;

    pub const EPOLLIN: i32 = 1;
    pub const EPOLLERR: i32 = 8;
    pub const EPOLLHUP: i32 = 10;
    pub const EPOLLET: i32 = 1 << 31;

    unsafe extern "C" {
        pub fn free(ptr: *mut core::ffi::c_void);
        pub fn fcntl(fildes: i32, cmd: i32, ...) -> i32;

        pub fn epoll_create(size: i32) -> i32;
        pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, epoll_event: *mut epoll_event) -> i32;
        pub fn epoll_wait(epfd: i32, events: *mut epoll_event, maxevents: i32, timeout: i32)
            -> i32;

        pub fn read(fd: i32, buf: *mut u8, count: usize) -> i32;
    }

    #[link(name = "pthread")]
    unsafe extern "C" {
        fn __errno_location() -> *mut i32;
    }

    pub fn errno() -> i32 {
        unsafe { *__errno_location() }
    }
}

fn get_xcb_atom_by_name(xcb_conn: *mut xcb_connection_t, atom_name: &[u8]) -> xcb_atom_t {
    unsafe {
        let cookie = xcb_intern_atom(
            xcb_conn,
            false as u8,
            atom_name.len() as u16,
            atom_name.as_ptr(),
        );
        let atom_reply = xcb_intern_atom_reply(xcb_conn, cookie, core::ptr::null_mut());
        assert_ne!(atom_reply, core::ptr::null_mut());
        let atomid = (*atom_reply).atom;
        libc::free(atom_reply as *mut _);
        atomid
    }
}

mod xrandr {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct xcb_randr_get_monitors_cookie_t {
        sequence: u32,
    }

    use super::{xcb_atom_t, xcb_connection_t, xcb_generic_error_t, xcb_timestamp_t, xcb_window_t};

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct xcb_randr_get_monitors_reply_t {
        response_type: u8,
        pad0: u8,
        sequence: u16,
        length: u32,
        timestamp: xcb_timestamp_t,
        nmonitors: u32,
        noutputs: u32,
        pad1: [u8; 12],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct xcb_randr_monitor_info_t {
        pub name: xcb_atom_t,
        pub primary: u8,
        pub automatic: u8,
        pub noutput: u16,
        pub x: i16,
        pub y: i16,
        pub width: u16,
        pub height: u16,
        pub width_in_millimeters: u32,
        pub height_in_millimeters: u32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct xcb_randr_monitor_info_iterator_t {
        pub data: *const xcb_randr_monitor_info_t,
        pub rem: i32,
        pub index: i32,
    }

    #[link(name = "xcb-randr")]
    #[link(name = "xcb-util")]
    unsafe extern "C" {
        pub fn xcb_randr_get_monitors(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            get_active: u8,
        ) -> xcb_randr_get_monitors_cookie_t;

        pub fn xcb_randr_get_monitors_reply(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_monitors_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_monitors_reply_t;

        pub fn xcb_randr_get_monitors_monitors_iterator(
            r: *const xcb_randr_get_monitors_reply_t,
        ) -> xcb_randr_monitor_info_iterator_t;
    }
}

struct SimpleDrawing {
    gc: xcb_gcontext_t,
}

impl SimpleDrawing {
    fn new(xcb: *mut xcb_connection_t, window: xcb_window_t) -> Option<Self> {
        let gc = unsafe { xcb_generate_id(xcb) };
        let gc_attrs = 0u32;
        let gc_cookie = unsafe {
            xcb_create_gc_checked(
                xcb,
                gc,
                window,
                XCB_GC_GRAPHICS_EXPOSURES,
                &gc_attrs as *const _ as *const core::ffi::c_void,
            )
        };
        let gc_err = unsafe { xcb_request_check(xcb, gc_cookie) };
        if !gc_err.is_null() {
            dbg!(unsafe { *gc_err });
            return None;
        }

        Some(Self { gc })
    }

    fn draw(&mut self, xcb: *mut xcb_connection_t, drawable: xcb_drawable_t) {
        unsafe {
            let fg = 0x34eb43_u32;
            let bg = 0x0d0704_u32;
            let values: [u32; 3] = [fg, bg, 8];
            xcb_change_gc(
                xcb,
                self.gc,
                XCB_GC_FOREGROUND | XCB_GC_BACKGROUND | XCB_GC_LINE_WIDTH,
                values.as_ptr() as *const _,
            );

            let rectangle = xcb_rectangle_t {
                x: 0,
                y: 0,
                width: 400,
                height: 160,
            };

            xcb_poly_fill_rectangle(xcb, drawable, self.gc, 1, &rectangle as *const _);

            let outline_rects: [xcb_rectangle_t; 2] = [
                xcb_rectangle_t {
                    x: 100,
                    y: 200,
                    width: 500,
                    height: 200,
                },
                xcb_rectangle_t {
                    x: 1000,
                    y: 1000,
                    width: 480,
                    height: 120,
                },
            ];

            xcb_poly_rectangle(
                xcb,
                drawable,
                self.gc,
                outline_rects.len() as u32,
                outline_rects.as_ptr(),
            );

            //
            // arcs

            let values: [u32; 2] = [0xbf1b28, 4];
            xcb_change_gc(
                xcb,
                self.gc,
                XCB_GC_FOREGROUND | XCB_GC_LINE_WIDTH,
                values.as_ptr() as *const _,
            );

            let arcs = [
                xcb_arc_t {
                    x: 512,
                    y: 512,
                    width: 256,
                    height: 256,
                    angle1: 0 << 6,
                    angle2: 120 << 6,
                },
                xcb_arc_t {
                    x: 512,
                    y: 512,
                    width: 512,
                    height: 128,
                    angle1: 0 << 6,
                    angle2: 360 << 6,
                },
            ];

            xcb_poly_arc(xcb, drawable, self.gc, arcs.len() as u32, arcs.as_ptr());
        }
    }
}

fn main() {
    eprintln!(
        "Sizeof epoll_event = {}, align = {}",
        std::mem::size_of::<libc::epoll_event>(),
        std::mem::align_of::<libc::epoll_event>()
    );

    let (xcb_conn, screen) = unsafe {
        let mut screen = 0i32;
        (
            xcb_connect(core::ptr::null(), &mut screen as *mut _),
            screen,
        )
    };

    dbg!(screen);

    if xcb_conn.is_null() {
        return;
    }

    let xcb_fd = unsafe { xcb_get_file_descriptor(xcb_conn) };
    dbg!(xcb_fd);

    let xcb_setup = unsafe { xcb_get_setup(xcb_conn) };
    dbg!(unsafe { *xcb_setup });

    let roots_iter = unsafe {
        let iter = xcb_setup_roots_iterator(xcb_setup);
        core::slice::from_raw_parts(iter.data, iter.rem as usize)
    };

    let windowid = unsafe { xcb_generate_id(xcb_conn) };
    dbg!(windowid);

    let screen = roots_iter.first().unwrap();
    dbg!(screen);

    let monitors_repl_cookie =
        unsafe { xrandr::xcb_randr_get_monitors(xcb_conn, screen.root, true as u8) };
    let monitors_repl = unsafe {
        xrandr::xcb_randr_get_monitors_reply(xcb_conn, monitors_repl_cookie, core::ptr::null_mut())
    };
    if monitors_repl.is_null() {
        return;
    }

    let monitors = unsafe {
        let xcb_iter = xrandr::xcb_randr_get_monitors_monitors_iterator(monitors_repl);
        core::slice::from_raw_parts(xcb_iter.data, xcb_iter.rem as usize)
    };

    for mon in monitors {
        println!(
            "Monitor: Origin: {}x{}, Size: {}x{}",
            mon.x, mon.y, mon.width, mon.height
        );
    }

    let cw_mask = XCB_CW_BACK_PIXEL | XCB_CW_EVENT_MASK;
    let cw_values: [u32; 2] = [
        0x302e2e,
        XCB_EVENT_MASK_KEY_PRESS | XCB_EVENT_MASK_STRUCTURE_NOTIFY | XCB_EVENT_MASK_EXPOSURE,
    ];

    let (width, height) = monitors
        .iter()
        .nth(0)
        .map(|m| (m.width, m.height))
        .unwrap_or((1200, 1200));

    let _ = unsafe {
        xcb_create_window(
            xcb_conn,
            screen.root_depth,
            windowid,
            screen.root,
            0,
            0,
            width,
            height,
            1,
            XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
            screen.root_visual,
            cw_mask,
            cw_values.as_ptr() as *const _,
        )
    };

    let atom_wm_delete = get_xcb_atom_by_name(xcb_conn, b"WM_DELETE_WINDOW\0");
    let atom_wm_protocols = get_xcb_atom_by_name(xcb_conn, b"WM_PROTOCOLS\0");

    dbg!(atom_wm_delete);
    dbg!(atom_wm_protocols);

    unsafe {
        xcb_change_property(
            xcb_conn,
            XCB_PROP_MODE_REPLACE,
            windowid,
            atom_wm_protocols,
            XCB_ATOM_ATOM,
            32,
            1,
            &atom_wm_delete as *const _ as *const core::ffi::c_void,
        );
        let window_title = b"XCB + Rust\0";
        xcb_change_property(
            xcb_conn,
            XCB_PROP_MODE_REPLACE,
            windowid,
            XCB_ATOM_WM_NAME,
            XCB_ATOM_STRING,
            8,
            window_title.len() as u32,
            window_title.as_ptr() as *const _,
        );
    }

    let mut app_context = SimpleDrawing::new(xcb_conn, windowid).unwrap();

    unsafe {
        xcb_map_window(xcb_conn, windowid);
        xcb_flush(xcb_conn);
    }

    #[derive(Copy, Clone)]
    struct XcbStateContext {
        fd: i32,
        // marker_: core::marker::PhantomPinned,
    }

    let mut xcb_state_ctx = XcbStateContext {
        fd: xcb_fd,
        // marker_: core::marker::PhantomPinned {},
    };

    let (epoll_fd,) = unsafe {
        let epoll_fd = libc::epoll_create(16);
        assert_ne!(epoll_fd, -1);

        let fd_flags = libc::fcntl(xcb_fd, libc::F_GETFL, 0);
        dbg!(fd_flags);

        let res = libc::fcntl(xcb_fd, libc::F_SETFL, fd_flags | libc::O_NONBLOCK);
        assert_ne!(res, -1);

        let mut ev = libc::epoll_event {
            events: (libc::EPOLLIN | libc::EPOLLET) as u32,
            data: libc::epoll_data_t {
                ptr: &mut xcb_state_ctx as *mut _ as *mut _,
            },
        };
        let op_res = libc::epoll_ctl(epoll_fd, libc::EPOLL_CTL_ADD, xcb_state_ctx.fd, &mut ev);
        assert_ne!(op_res, -1);
        (epoll_fd,)
    };

    let mut events_buffer: [libc::epoll_event; 8] =
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

    'main_loop: loop {
        unsafe {
            let events_count = libc::epoll_wait(
                epoll_fd,
                events_buffer.as_mut_slice().as_mut_ptr(),
                core::mem::size_of_val(&events_buffer) as i32,
                0,
            );
            if events_count < 0 {
                eprintln!("Epoll error!");
                continue;
            }
            if events_count == 0 {
                continue;
            }

            for e in events_buffer[..events_count as usize].iter() {
                //
                // event available on descriptor
                if (e.events & libc::EPOLLIN as u32) != 0 {
                    let context = e.data.ptr as *const XcbStateContext;
                    let mut generic_event = core::mem::MaybeUninit::<xcb_generic_event_t>::uninit();
                    //
                    // read one xcb event and process it
                    'read_xcb_fd: loop {
                        let bytes_read = libc::read(
                            (*context).fd,
                            generic_event.as_mut_ptr() as *mut u8,
                            core::mem::size_of_val(&generic_event),
                        );

                        if bytes_read == 0 {
                            eprintln!("0 bytes read ... MonkaHMMMM ...");
                            break 'read_xcb_fd;
                        }
                        if bytes_read == -1 {
                            if libc::errno() == libc::EAGAIN {
                                break 'read_xcb_fd;
                            }

                            eprintln!("Error reading from xcb_fd {}", libc::errno());
                            break 'read_xcb_fd;
                        }

                        let generic_event = generic_event.assume_init();
                        let event_type = generic_event.response_type & 0x7F;
                        match event_type {
                            XCB_KEY_PRESS => {
                                let key_event =
                                    *(&generic_event as *const _ as *const xcb_key_press_event_t);
                                dbg!(key_event);
                                if key_event.detail == 9 {
                                    break 'main_loop;
                                }
                            }

                            XCB_CLIENT_MESSAGE => {
                                let cli_msg = *(&generic_event as *const _
                                    as *const xcb_client_message_event_t);
                                if cli_msg.data.data32[0] == atom_wm_delete {
                                    xcb_destroy_window(xcb_conn, windowid);
                                    break 'main_loop;
                                }
                            }

                            XCB_EXPOSE => {
                                app_context.draw(xcb_conn, windowid);
                            }

                            _ => {}
                        }
                        xcb_flush(xcb_conn);
                    }
                }
                //
                // TODO: maybe handle this better ?
                if (e.events & libc::EPOLLERR as u32) != 0 {}
            }
        }

        #[cfg(feature = "classic_event_processing")]
        {
            let e = unsafe { xcb_wait_for_event(xcb_conn) };
            if e.is_null() {
                break 'main_loop;
            }
            dbg!(event_type);
            match event_type {
                XCB_KEY_PRESS => unsafe {
                    let key_event = *(e as *const xcb_key_press_event_t);
                    dbg!(key_event);
                    if key_event.detail == 9 {
                        break 'main_loop;
                    }
                },

                XCB_CLIENT_MESSAGE => unsafe {
                    let cli_msg = *(e as *const xcb_client_message_event_t);
                    if cli_msg.data.data32[0] == atom_wm_delete {
                        xcb_destroy_window(xcb_conn, windowid);
                        break 'main_loop;
                    }
                },

                XCB_EXPOSE => {
                    context.draw(xcb_conn, windowid);
                    unsafe {
                        xcb_flush(xcb_conn);
                    }
                }

                _ => {}
            }

            unsafe {
                free(e as *mut _);
            }
        }
    }

    unsafe {
        xcb_disconnect(xcb_conn);
    }
}
