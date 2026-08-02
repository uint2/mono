#[macro_use]
mod enum_array;

#[macro_use]
mod macros;

mod C;
mod app;
mod client;
mod config;
mod drw;
mod enums;
mod ffi2;
mod font;
mod id;
mod layout;
mod linked_list;
mod monitor;
mod nonempty;
mod numlockmask;
mod prelude;
mod rect;
mod setup;
mod toggle;
mod x11;

use prelude::*;

fn handle_cli_args() -> bool {
    let argv = std::env::args().collect::<Vec<_>>();
    match argv.len() {
        0 => panic!("How did you even get here"),
        1 => return false, // continue execution
        2 if argv[1] == "-v" => {
            println!("{NAME}-{VERSION}");
            return true;
        }
        _ => {
            println!("usage: {NAME} [-v]");
            return true;
        }
    }
}

/// Global X error handler.
static mut XERRORXLIB: Option<
    unsafe extern "C" fn(*mut C::Display, *mut C::XErrorEvent) -> c_int,
> = None;

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

fn check_locale_support() {
    let result = unsafe { libc::setlocale(libc::LC_CTYPE, ptr::null()) };
    let setlocale_ok = result != ptr::null_mut();
    let supports = unsafe { C::XSupportsLocale() } != 0;
    if !(setlocale_ok && supports) {
        log::warn!("no locale support");
    }
}

pub fn init_check_win(
    dpy: &Display,
    root: &Window,
    check_win: &Window,
    netatoms: &NetArray<C::Atom>,
) {
    let utf8string = x11::XInternAtom(dpy, "UTF8_STRING", false).unwrap();
    let check_win = check_win.c();
    let cw_ptr = &check_win as *const C::Window as *const u8;
    let dpy = dpy.c();
    let atom_wmcheck = *netatoms.get(Net::WMCheck).unwrap();
    let atom_wmname = *netatoms.get(Net::WMName).unwrap();
    let pmp = C::PropModeReplace as c_int;
    const XA_WINDOW: c_ulong = 33; // Hard-coded after referencing X documentation.
    use C::XChangeProperty as CP;
    let app_name = NAME.c_str();
    let app_name = app_name.as_ptr() as *const u8;
    let app_len = NAME.len() as c_int;
    unsafe {
        CP(dpy, check_win, atom_wmcheck, XA_WINDOW, 32, pmp, cw_ptr, 1);
        CP(dpy, check_win, atom_wmname, utf8string, 8, pmp, app_name, app_len);
        CP(dpy, root.c(), atom_wmcheck, XA_WINDOW, 32, pmp, cw_ptr, 1);
    }
}

fn try_main() -> Result<()> {
    let false = handle_cli_args() else { return Ok(()) };
    check_locale_support();

    let Some(dpy) = Display::open() else {
        return Err(log::error!("{NAME}: cannot open display"));
    };
    log::info!("Established connection to x-server");

    setup::check_other_wm(&dpy);
    setup::setup_sigaction()?;
    setup::clean_up_zombies();
    let screen = dpy.default_screen();
    let screen_size = dpy.display_size(screen);
    let root = dpy.default_root_window();
    let drw = Drw::new(dpy, root.clone(), screen, screen_size.convert());
    let fonts = Fonts::new(dpy, screen, config::FONTS);
    let colors = setup::setup_color_scheme(dpy, screen);
    let cursors = setup::setup_cursors(dpy);

    let monitors = NonEmpty::new(Monitor::new(dpy));
    let wmatoms = WM::init(&dpy);
    let netatoms = Net::init(&dpy);

    let check_win = Window::check_win(dpy, &root);
    init_check_win(&dpy, &root, &check_win, &netatoms);

    log::info!("Ran to the end of try_main()");
    Ok(())
}

fn main() -> ExitCode {
    use ExitCode as EC;

    log::init(Some(log::LevelFilter::Debug));
    log::info!("------------------------------------------------------------");
    log::info!("Started execution of kopiwm-rs!");
    log::info!("------------------------------------------------------------");

    const LOCAL_ONLY: bool = false;
    if LOCAL_ONLY {
        return EC::SUCCESS;
    }

    match try_main() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
