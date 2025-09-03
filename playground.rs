#![allow(non_camel_case_types)]

#[repr(C)]
struct Display {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
struct Visual {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

type Pixmap = u64;
type Colormap = u64;
type Cursor = u64;
type Atom = u64;
type Window = u64;
type Time = u64;
type Status = i32;
type Bool = i32;
type KeySym = u64;
type KeyCode = u8;

#[repr(C)]
struct XSetWindowAttributes {
    background_pixmap: Pixmap,
    background_pixel: u64,
    border_pixmap: Pixmap,
    border_pixel: u64,
    bit_gravity: i32,
    win_gravity: i32,
    backing_store: i32,
    backing_planes: u64,
    backing_pixel: u64,
    save_under: i32,
    event_mask: i64,
    do_not_propagate_mask: i64,
    override_redirect: i32,
    colormap: Colormap,
    cursor: Cursor,
}

impl XSetWindowAttributes {
    pub fn new() -> Self {
        Self {
            background_pixmap: 0,
            background_pixel: 0,
            border_pixmap: 0,
            border_pixel: 0,
            bit_gravity: 0,
            win_gravity: 0,
            backing_store: 0,
            backing_planes: 0,
            backing_pixel: 0,
            save_under: 0,
            event_mask: 0,
            do_not_propagate_mask: 0,
            override_redirect: 0,
            colormap: 0,
            cursor: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct XKeyEvent {
    type_: i32,
    serial: u64,
    send_event: Bool,
    display: *mut Display,
    window: Window,
    root: Window,
    subwindow: Window,
    time: Time,
    x: i32,
    y: i32,
    x_root: i32,
    y_root: i32,
    state: u32,
    keycode: u32,
    same_screen: Bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct XExposeEvent {
    type_: i32,
    serial: u32,
    send_event: Bool,
    display: *mut Display,
    window: Window,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    count: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct XConfigureEvent {
    type_: i32,
    serial: u32,
    send_event: Bool,
    display: *mut Display,
    event: Window,
    window: Window,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    border_width: i32,
    above: Window,
    override_redirect: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct XClientMessageEvent {
    type_: i32,
    serial: u64,
    send_event: Bool,
    display: *mut Display,
    window: Window,
    message_type: Atom,
    format: i32,
    data: [core::ffi::c_long; 5],
}

#[repr(C)]
union XEvent {
    type_: i32,
    xkey: XKeyEvent,
    xexpose: XExposeEvent,
    xconfigure: XConfigureEvent,
    xclientmessage: XClientMessageEvent,
    pad_: [core::ffi::c_long; 24],
}

impl XEvent {
    fn new() -> Self {
        unsafe { core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

const CopyFromParent: u32 = 0;
const ShiftMask: u32 = 1 << 0;

const NoEventMask: i32 = 0;
const KeyPressMask: i32 = 1 << 0;
const KeyReleaseMask: i32 = 1 << 1;
const ButtonPressMask: i32 = 1 << 2;
const ButtonReleaseMask: i32 = 1 << 3;
const EnterWindowMask: i32 = 1 << 4;
const LeaveWindowMask: i32 = 1 << 5;
const PointerMotionMask: i32 = 1 << 6;
const PointerMotionHintMask: i32 = 1 << 7;
const Button1MotionMask: i32 = 1 << 8;
const Button2MotionMask: i32 = 1 << 9;
const Button3MotionMask: i32 = 1 << 10;
const Button4MotionMask: i32 = 1 << 11;
const Button5MotionMask: i32 = 1 << 12;
const ButtonMotionMask: i32 = 1 << 13;
const ExposureMask: i32 = 1 << 15;
const VisibilityChangeMask: i32 = 1 << 16;
const StructureNotifyMask: i32 = 1 << 17;
const ResizeRedirectMask: i32 = 1 << 18;
const SubstructureNotifyMask: i32 = 1 << 19;
const SubstructureRedirectMask: i32 = 1 << 20;
const FocusChangeMask: i32 = 1 << 21;

const CWBackPixmap: i32 = 1 << 0;
const CWBackPixel: i32 = 1 << 1;
const CWBorderPixmap: i32 = 1 << 2;
const CWBorderPixel: i32 = 1 << 3;
const CWBitGravity: i32 = 1 << 4;
const CWWinGravity: i32 = 1 << 5;
const CWBackingStore: i32 = 1 << 6;
const CWBackingPlanes: i32 = 1 << 7;
const CWBackingPixel: i32 = 1 << 8;
const CWOverrideRedirect: i32 = 1 << 9;
const CWSaveUnder: i32 = 1 << 10;
const CWEventMask: i32 = 1 << 11;
const CWDontPropagate: i32 = 1 << 12;
const CWColormap: i32 = 1 << 13;
const CWCursor: i32 = 1 << 14;

const ConfigureNotify: i32 = 22;
const KeyPress: i32 = 2;
const KeyRelease: i32 = 3;
const ClientMessage: i32 = 33;

const XK_Escape: KeySym = 0xff1b;

#[link(name = "X11")]
unsafe extern "C" {
    fn XOpenDisplay(name: *const i8) -> *mut Display;
    fn XDefaultRootWindow(display: *mut Display) -> Window;
    fn XCreateWindow(
        display: *mut Display,
        parent: Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        depth: i32,
        class: u32,
        visual: *mut Visual,
        valuemask: u64,
        attributes: *mut XSetWindowAttributes,
    ) -> Window;
    fn XStoreName(display: *mut Display, window: Window, window_name: *const u8) -> i32;
    fn XListProperties(display: *mut Display, w: Window, num_prop_return: *mut i32) -> *mut Atom;
    fn XFree(ptr: *mut core::ffi::c_void) -> i32;
    fn XGetAtomName(display: *mut Display, atom: Atom) -> *mut u8;
    fn XInternAtom(display: *mut Display, atom_name: *const u8, only_if_exists: i32) -> Atom;
    fn XSetWMProtocols(
        display: *mut Display,
        w: Window,
        protocols: *const Atom,
        count: i32,
    ) -> Status;
    fn XMapRaised(display: *mut Display, w: Window) -> i32;
    fn XNextEvent(display: *mut Display, event_return: *mut XEvent) -> i32;
    fn XKeycodeToKeysym(display: *mut Display, keycode: KeyCode, index: i32) -> KeySym;
    fn XDestroyWindow(display: *mut Display, w: Window) -> i32;

}

unsafe extern "C" {
    fn write(fildes: i32, buf: *const u8, nbyte: usize) -> isize;
}

fn write_str(msg: &[u8]) {
    unsafe {
        write(1, msg.as_ptr(), msg.len());
    }
}

pub fn main() {
    let display = unsafe { XOpenDisplay(core::ptr::null_mut()) };
    let root_window = unsafe { XDefaultRootWindow(display) };

    let mut window_attribs = XSetWindowAttributes {
        background_pixel: 0x00aade87,
        event_mask: (StructureNotifyMask | KeyPressMask | KeyReleaseMask) as i64,
        ..XSetWindowAttributes::new()
    };

    let window = unsafe {
        XCreateWindow(
            display,
            root_window,
            0,
            0,
            1200,
            1200,
            0,
            CopyFromParent as i32,
            CopyFromParent,
            core::ptr::null_mut(),
            (CWBackPixel | CWEventMask) as u64,
            &mut window_attribs as *mut _,
        )
    };

    if window == 0 {
        return;
    }

    let atom_wm_delete = unsafe {
        XStoreName(display, window, b"X11 + Rust\0".as_ptr());
        let wm_delete_window = XInternAtom(display, b"WM_DELETE_WINDOW\0".as_ptr(), false as i32);
        dbg!(wm_delete_window);
        let res = XSetWMProtocols(display, window, &wm_delete_window as *const _, 1);
        assert_ne!(res, 0);
        XMapRaised(display, window);
        wm_delete_window
    };

    'main_loop: loop {
        let mut xevent = XEvent::new();
        unsafe {
            XNextEvent(display, &mut xevent as *mut _);
        }

        unsafe {
            match xevent.type_ {
                KeyPress => {
                    dbg!(xevent.xkey);
                    let key_index = xevent.xkey.state & ShiftMask;
                    let key_sym =
                        XKeycodeToKeysym(display, xevent.xkey.keycode as KeyCode, key_index as i32);
                    if key_sym == XK_Escape {
                        break 'main_loop;
                    }
                }
                ClientMessage => {
                    dbg!(xevent.xclientmessage);
                    if xevent.xclientmessage.data[0] == atom_wm_delete as core::ffi::c_long {
                        XDestroyWindow(display, window);
                        break 'main_loop;
                    }
                }
                _ => {}
            }
        }
    }

    write_str(b"That\'s all she wrote, folks!\0");
}
