use crate::C;
use crate::prelude::*;

/// Thinnest wrapper around XftFont to manage drops.
pub struct XftFont {
    dpy: Display,
    font: NonNull<C::XftFont>,
}
c!(XftFont, font);

impl XftFont {
    pub const fn new(dpy: Display, font: *mut C::XftFont) -> Option<Self> {
        let Some(font) = NonNull::new(font) else { return None };
        Some(Self { dpy, font })
    }

    pub fn ascent(&self) -> c_int {
        unsafe { self.font.as_ref() }.ascent
    }

    pub fn descent(&self) -> c_int {
        unsafe { self.font.as_ref() }.descent
    }
}

impl Drop for XftFont {
    fn drop(&mut self) {
        unsafe { C::XftFontClose(self.dpy.c(), self.c()) };
    }
}
