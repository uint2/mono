use crate::C;
use crate::prelude::*;

pub struct Cursor {
    dpy: Display,
    cursor: C::Cursor,
}

impl Cursor {
    pub fn new(dpy: Display, cursor: c_uint) -> Self {
        let cursor = unsafe { C::XCreateFontCursor(dpy.c(), cursor) };
        Self { dpy, cursor }
    }

    pub const fn cursor(&self) -> C::Cursor {
        self.cursor
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { C::XFreeCursor(self.dpy.c(), self.cursor) };
    }
}
