use crate::prelude::*;

/// When this window gets dropped, the destructor in X11 get called.
/// In this sense, we own this window.
pub struct OwnedWindow {
    inner_x11: C::Window,
}

impl Drop for OwnedWindow {
    fn drop(&mut self) {
        if self.inner_x11 == 0 {
            return;
        }
        unsafe { C::XDestroyWindow(dpy.c(), self.window) };
    }
}

/// Nothing happens when this window gets dropped.
pub struct WindowRef {
    inner_x11: C::Window,
}
