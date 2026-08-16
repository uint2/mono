use crate::C;
use crate::prelude::*;

pub struct Cursor {
    cursor: C::Cursor,
}

impl Cursor {
    pub fn new(cursor: c_uint) -> Self {
        let cursor = unsafe { C::XCreateFontCursor(dpy.c(), cursor) };
        Self { cursor }
    }

    pub const fn cursor(&self) -> C::Cursor {
        self.cursor
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { C::XFreeCursor(dpy.c(), self.cursor) };
    }
}
