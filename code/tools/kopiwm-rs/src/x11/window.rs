use crate::C;
use crate::prelude::*;

// TODO: rename this to OwnedWindow or something that clearly differentiates
// that `XDestroyWindow` is called on this one upon `Drop`.
//
// NOTE: We do NOT implement `clone` for this struct because that would imply
// that we call `XDestroyWindow` twice.
pub struct Window {
    dpy: Display,
    window: C::Window,
}

impl Drop for Window {
    fn drop(&mut self) {
        if self.window == 0 {
            return;
        }
        unsafe { C::XDestroyWindow(self.dpy.c(), self.window) };
    }
}

impl Window {
    pub const fn new(dpy: Display, window: C::Window) -> Self {
        Self { dpy, window }
    }

    pub const fn dpy(&self) -> *mut C::Display {
        self.dpy.c()
    }

    pub const fn c(&self) -> C::Window {
        self.window
    }

    pub fn check_win(dpy: Display, root: &Window) -> Self {
        let check_win =
            unsafe { C::XCreateSimpleWindow(dpy.c(), root.c(), 0, 0, 1, 1, 0, 0, 0) };
        Self::new(dpy, check_win)
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.window == other.window
    }
}
