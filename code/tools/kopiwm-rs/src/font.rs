use crate::C;
use crate::prelude::*;

pub struct Font {
    dpy: Display,
    height: c_int,
    xfont: XftFont,
    pattern: Option<FcPattern>,
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
        let Some(pattern) = FcPattern::from_name(name) else {
            log::error!("Cannot parse font name to pattern: {name}");
            todo!();
        };
        let height = xfont.ascent() + xfont.descent();
        Some(Self { dpy, height, xfont, pattern: Some(pattern) })
    }

    pub fn from_pattern(dpy: Display, pattern: FcPattern) -> Option<Self> {
        let Some(xfont) = dpy.xft_font_open_pattern(&pattern) else {
            log::error!("Cannot load font from pattern");
            return None;
        };
        let height = xfont.ascent() + xfont.descent();
        Some(Self { dpy, height, xfont, pattern: Some(pattern) })
    }

    pub const fn xfont(&self) -> &XftFont {
        &self.xfont
    }

    pub const fn height(&self) -> c_int {
        self.height
    }

    /// Uses `XftCharExists` to check if the character is supported.
    pub fn supports_char(&self, utf8codepoint: char) -> bool {
        let dpy = self.dpy.c();
        let font = self.xfont.c();
        let result = unsafe { C::XftCharExists(dpy, font, utf8codepoint as c_uint) };
        result != 0
    }
}

pub struct Fonts {
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
