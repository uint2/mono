use std::process::ExitCode;

use x11rb::connection::Connection;
use x11rb::errors::ConnectError;
use x11rb::errors::ConnectionError;
use x11rb::rust_connection::RustConnection;

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

fn open_display() -> Option<(RustConnection, usize)> {
    match x11rb::connect(None) {
        Ok(v) => Some(v),
        Err(err) => {
            log::error!("{NAME}: cannot open X display.");
            log::error!("{err}");
            None
        }
    }
}

fn check_other_wm() {
    // xerrorxlib = XSetErrorHandler(xerrorstart);
    // /* this causes an error if some other window manager is running */
    // XSelectInput(dpy, DefaultRootWindow(dpy), SubstructureRedirectMask);
    // XSync(dpy, False);
    // XSetErrorHandler(xerror);
    // XSync(dpy, False);
}

fn main() -> ExitCode {
    log::init(Some(log::LevelFilter::Debug));

    log::info!("Started execution of kopiwm-rs!");

    if let Some(exit_code) = handle_cli_args() {
        return exit_code;
    }

    {
        let result = unsafe { libc::setlocale(libc::LC_CTYPE, std::ptr::null()) };
        _ = result != std::ptr::null_mut();
        // TODO: port XSupportsLocale
    }

    let Some((dpy, screen)) = open_display() else { return ExitCode::FAILURE };
    println!("{dpy:?}");

    ExitCode::SUCCESS
}
