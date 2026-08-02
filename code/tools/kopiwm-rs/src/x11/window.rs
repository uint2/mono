use crate::C;
use crate::prelude::*;

#[derive(Clone)]
pub struct Window {
    dpy: Display,
    window: C::Window,
}

impl Drop for Window {
    fn drop(&mut self) {
        if self.window == 0 {
            return;
        }

        unsafe {
            C::XDestroyWindow(self.dpy.c(), self.window);
        }
    }
}

impl Window {
    pub const fn new(dpy: Display, window: C::Window) -> Self {
        Self { dpy, window }
    }

    pub const fn c(&self) -> C::Window {
        self.window
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.window == other.window
    }
}
