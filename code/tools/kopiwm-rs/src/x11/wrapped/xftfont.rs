use crate::C;
use crate::prelude::*;

/// Thinnest wrapper around XftFont to manage drops.
pub struct XftFont {
    font: NonNull<C::XftFont>,
}
c!(XftFont, font);

impl Drop for XftFont {
    fn drop(&mut self) {
        unsafe { C::XftFontClose(dpy.c(), self.c()) };
    }
}

impl XftFont {
    pub const fn new(font: *mut C::XftFont) -> Option<Self> {
        let Some(font) = NonNull::new(font) else { return None };
        Some(Self { font })
    }

    pub const fn ascent(&self) -> c_int {
        unsafe { self.font.as_ref() }.ascent
    }

    pub const fn descent(&self) -> c_int {
        unsafe { self.font.as_ref() }.descent
    }
}
