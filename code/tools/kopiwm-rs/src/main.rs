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

// fn open_display() -> Option<(RustConnection, usize)> {
//     match x11rb::connect(None) {
//         Ok(v) => Some(v),
//         Err(err) => {
//             log::error!("{NAME}: cannot open X display.");
//             log::error!("{err}");
//             None
//         }
//     }
// }

/// Global X error handler.
static mut XERRORXLIB: Option<
    unsafe extern "C" fn(*mut C::Display, *mut C::XErrorEvent) -> c_int,
> = None;

/// Startup error handler to check if another window manager is already running.
unsafe extern "C" fn xerrorstart(_: *mut C::Display, _: *mut C::XErrorEvent) -> c_int {
    panic!("{NAME}: another window manager is already running");
}

unsafe extern "C" fn xerror(dpy: *mut C::Display, event: *mut C::XErrorEvent) -> c_int {
    0
}

fn check_other_wm(dpy: &x11::Display) {
    let handler = unsafe { C::XSetErrorHandler(Some(xerrorstart)) };
    unsafe { XERRORXLIB = handler };
    dpy.select_input(dpy.default_root_window(), C::SubstructureRedirectMask as c_long);

    // C::XSelectInput();
    // xerrorxlib = XSetErrorHandler(xerrorstart);
    // /* this causes an error if some other window manager is running */
    // XSelectInput(dpy, DefaultRootWindow(dpy), SubstructureRedirectMask);
    // XSync(dpy, False);
    // XSetErrorHandler(xerror);
    // XSync(dpy, False);
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

    // let Some((dpy, screen)) = open_display() else { return ExitCode::FAILURE };
    // println!("{dpy:?}");

    EC::SUCCESS
}
