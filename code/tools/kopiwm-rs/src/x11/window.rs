use crate::C;
use crate::prelude::*;

/// TODO: rename this to OwnedWindow or something that clearly differentiates
/// that `XDestroyWindow` is called on this one upon `Drop`.
///
/// NOTE: We do NOT implement `clone` for this struct because that would imply
/// that we call `XDestroyWindow` twice.
pub struct Window {
    window: C::Window,
}

impl Drop for Window {
    fn drop(&mut self) {
        if self.window == 0 {
            return;
        }
        unsafe { C::XDestroyWindow(dpy.c(), self.window) };
    }
}

impl Window {
    pub const fn new(window: C::Window) -> Self {
        Self { window }
    }

    pub const fn c(&self) -> C::Window {
        self.window
    }

    pub fn check_win(root: &Window) -> Self {
        let check_win =
            unsafe { C::XCreateSimpleWindow(dpy.c(), root.c(), 0, 0, 1, 1, 0, 0, 0) };
        Self::new(check_win)
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.window == other.window
    }
}
