mod C;
mod prelude;
mod x11;

use prelude::*;

use std::process::ExitCode;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn handle_cli_args() -> Option<ExitCode> {
    let argv = std::env::args().collect::<Vec<_>>();
    match argv.len() {
        0 => panic!("How did you even get here"),
        1 => return None, // continue execution
        2 if argv[1] == "-v" => {
            println!("{NAME}-{VERSION}");
            return Some(ExitCode::SUCCESS);
        }
        _ => {
            println!("usage: {NAME} [-v]");
            return Some(ExitCode::SUCCESS);
        }
    }
}

/// Global X error handler.
static mut XERRORXLIB: Option<
    unsafe extern "C" fn(*mut C::Display, *mut C::XErrorEvent) -> c_int,
> = None;

/// Startup error handler to check if another window manager is already running.
unsafe extern "C" fn xerrorstart(_: *mut C::Display, _: *mut C::XErrorEvent) -> c_int {
    panic!("{NAME}: another window manager is already running");
}

unsafe extern "C" fn xerror(dpy: *mut C::Display, event: *mut C::XErrorEvent) -> c_int {
    const BAD_WINDOW: u8 = C::BadWindow as u8;
    const BAD_MATCH: u8 = C::BadMatch as u8;
    const BAD_DRAWABLE: u8 = C::BadDrawable as u8;
    const BAD_ACCESS: u8 = C::BadAccess as u8;

    const SET_INPUT_FOCUS: u8 = C::X_SetInputFocus as u8;
    const POLY_TEXT_8: u8 = C::X_PolyText8 as u8;
    const POLY_FILL_RECTANGLE: u8 = C::X_PolyFillRectangle as u8;
    const POLY_SEGMENT: u8 = C::X_PolySegment as u8;
    const CONFIGURE_WINDOW: u8 = C::X_ConfigureWindow as u8;
    const GRAB_BUTTON: u8 = C::X_GrabButton as u8;
    const GRAB_KEY: u8 = C::X_GrabKey as u8;
    const COPY_AREA: u8 = C::X_CopyArea as u8;

    let ev = unsafe { event.read() };

    match (ev.error_code, ev.request_code) {
        (BAD_WINDOW, _) => return 0,
        (BAD_MATCH, SET_INPUT_FOCUS | CONFIGURE_WINDOW) => return 0,
        (BAD_DRAWABLE, COPY_AREA) => return 0,
        (BAD_DRAWABLE, POLY_TEXT_8 | POLY_FILL_RECTANGLE | POLY_SEGMENT) => return 0,
        (BAD_ACCESS, GRAB_BUTTON | GRAB_KEY) => return 0,
        _ => {}
    }
    log::error!(
        "{NAME}: fatal error: request code={}, error code={}",
        ev.request_code,
        ev.error_code,
    );
    let Some(xerrorlib) = (unsafe { XERRORXLIB }) else {
        panic!("{NAME}: xerrorlib handler missing")
    };
    unsafe { xerrorlib(dpy, event) }
}

fn check_other_wm(dpy: &x11::Display) {
    let handler = unsafe { C::XSetErrorHandler(Some(xerrorstart)) };
    unsafe { XERRORXLIB = handler };
    // this causes an error if some other window manager is running.
    dpy.select_input(dpy.default_root_window(), C::SubstructureRedirectMask as c_long);
    dpy.sync(false);
    unsafe { C::XSetErrorHandler(Some(xerror)) };
    dpy.sync(false);
}

fn check_locale_support() {
    let result = unsafe { libc::setlocale(libc::LC_CTYPE, ptr::null()) };
    let setlocale_ok = result != ptr::null_mut();
    let supports = unsafe { C::XSupportsLocale() } != 0;
    if !(setlocale_ok && supports) {
        log::warn!("no locale support");
    }
}

const LOCAL_ONLY: bool = false;
fn safe_local_testing() {}

fn main() -> ExitCode {
    use ExitCode as EC;

    if LOCAL_ONLY {
        safe_local_testing();
        return EC::SUCCESS;
    }

    log::init(Some(log::LevelFilter::Debug));

    log::info!("Started execution of kopiwm-rs!");

    if let Some(exit_code) = handle_cli_args() {
        return exit_code;
    }
    check_locale_support();

    let Some(dpy) = x11::Display::open() else {
        log::error!("{NAME}: cannot open display");
        return EC::FAILURE;
    };
    check_other_wm(&dpy);

    EC::SUCCESS
}
