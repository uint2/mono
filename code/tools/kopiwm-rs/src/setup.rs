use crate::C;
use crate::prelude::*;

use nix::sys::signal::{SaFlags, SigAction, SigHandler, SigSet, Signal, sigaction};

use crate::{XERRORXLIB, xerror};

/// Startup error handler to check if another window manager is already running.
unsafe extern "C" fn xerrorstart(_: *mut C::Display, _: *mut C::XErrorEvent) -> c_int {
    panic!("{NAME}: another window manager is already running");
}

pub fn check_other_wm(dpy: &Display) {
    let handler = unsafe { C::XSetErrorHandler(Some(xerrorstart)) };
    unsafe { XERRORXLIB = handler };
    // this causes an error if some other window manager is running.
    dpy.select_input(dpy.default_root_window(), C::SubstructureRedirectMask as c_long);
    dpy.sync(false);
    unsafe { C::XSetErrorHandler(Some(xerror)) };
    dpy.sync(false);
}

pub fn setup_sigaction() -> Result<()> {
    let mut flags = SaFlags::empty();

    // Do not receive notification when child processes stop or resume.
    flags.insert(SaFlags::SA_NOCLDSTOP);

    // Do not transform children into zombies when they terminate.
    //
    // If the SA_NOCLDWAIT flag is set when establishing a handler for
    // SIGCHLD, POSIX.1 leaves it unspecified whether a SIGCHLD signal is
    // generated when a child process terminates.  On Linux, a SIGCHLD
    // signal is generated in this case; on some other implementations, it
    // is not.
    flags.insert(SaFlags::SA_NOCLDWAIT);

    // Provide behavior compatible with BSD signal semantics by making
    // certain system calls restartable across signals. This flag is
    // meaningful only when establishing a signal handler.
    flags.insert(SaFlags::SA_RESTART);

    let action = SigAction::new(SigHandler::SigIgn, flags, SigSet::empty());
    if let Err(err) = unsafe { sigaction(Signal::SIGINT, &action) } {
        return Err(log::error!("Call to sigaction failed with errno={err}"));
    }
    Ok(())
}

/// clean up any zombies (inherited from .xinitrc etc) immediately.
pub fn clean_up_zombies() {
    loop {
        let result = unsafe { libc::waitpid(-1, ptr::null_mut(), libc::WNOHANG) };
        if result <= 0 {
            break;
        }
    }
}
