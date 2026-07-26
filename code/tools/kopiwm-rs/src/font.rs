use crate::C;
use crate::prelude::*;

/// Thinnest wrapper around XftFont to manage drops.
pub(crate) struct XFont {
    dpy: Display,
    font: *mut C::XftFont,
}

impl XFont {
    pub const fn new(dpy: Display, font: *mut C::XftFont) -> Option<Self> {
        if font.is_null() { None } else { Some(Self { dpy, font }) }
    }

    pub const fn to_c(&self) -> *mut C::XftFont {
        self.font
    }

    pub fn ascent(&self) -> c_int {
        (unsafe { *self.font }).ascent
    }

    pub fn descent(&self) -> c_int {
        (unsafe { *self.font }).descent
    }
}

impl Drop for XFont {
    fn drop(&mut self) {
        unsafe { C::XftFontClose(self.dpy.to_c(), self.font) };
    }
}

/// Thinnest wrapper around FcPattern to manage drops.
pub(crate) struct XPattern(*mut C::FcPattern);

impl XPattern {
    pub const fn new(pattern: *mut C::FcPattern) -> Option<Self> {
        if pattern.is_null() { None } else { Some(Self(pattern)) }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        let pattern = unsafe { C::FcNameParse(name.c().as_ptr() as *const u8) };
        Self::new(pattern)
    }

    pub const fn to_c(&self) -> *mut C::FcPattern {
        self.0
    }
}

impl Drop for XPattern {
    fn drop(&mut self) {
        unsafe { C::FcPatternDestroy(self.0) };
    }
}

pub(crate) struct Font {
    dpy: Display,
    height: c_int,
    xfont: XFont,
    pattern: Option<XPattern>,
}

impl Font {
    pub fn from_name(dpy: Display, screen: Screen, name: &str) -> Option<Self> {
        if name.is_empty() {
            log::error!("Font name must not be empty");
            return None;
        }
        let Some(xfont) = dpy.xft_font_open_name(screen, name) else {
            log::error!("Cannot load font from name: {name}");
            return None;
        };
        let Some(pattern) = XPattern::from_name(name) else {
            log::error!("Cannot parse font name to pattern: {name}");
            todo!();
        };
        let height = xfont.ascent() + xfont.descent();
        Some(Self { dpy, height, xfont, pattern: Some(pattern) })
    }

    pub fn from_pattern(dpy: Display, pattern: XPattern) -> Option<Self> {
        let Some(xfont) = dpy.xft_font_open_pattern(&pattern) else {
            log::error!("Cannot load font from pattern");
            return None;
        };
        let height = xfont.ascent() + xfont.descent();
        Some(Self { dpy, height, xfont, pattern: Some(pattern) })
    }

    pub const fn xfont(&self) -> &XFont {
        &self.xfont
    }

    pub const fn height(&self) -> c_int {
        self.height
    }

    /// Uses `XftCharExists` to check if the character is supported.
    pub fn supports_char(&self, utf8codepoint: char) -> bool {
        let dpy = self.dpy.to_c();
        let font = self.xfont.to_c();
        let result = unsafe { C::XftCharExists(dpy, font, utf8codepoint as c_uint) };
        result != 0
    }
}

pub(crate) struct Fonts {
    fonts: Vec<Font>,
}

impl Fonts {
    pub fn new(dpy: Display, screen: Screen, fonts: &[&str]) -> Self {
        let mut vec = Vec::with_capacity(fonts.len());
        for font in fonts {
            vec.push(Font::from_name(dpy, screen, font).unwrap());
        }
        Self { fonts: vec }
    }

    pub fn find_font_that_has_char(&self, utf8codepoint: char) -> Option<&Font> {
        self.fonts.iter().find(|f| f.supports_char(utf8codepoint))
    }
}
