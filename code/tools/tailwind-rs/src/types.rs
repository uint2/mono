use core::fmt;

/// A color like "sky", "green", "emerald", etc.
#[derive(PartialEq, Eq, Hash)]
pub struct Color(String);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Color {
    pub fn new(x: &str) -> Self {
        Self(x.to_string())
    }

    pub const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// A shade like 50, 100, 200, ..., 900.
#[derive(PartialEq, Eq, Hash)]
pub struct Shade(String);

impl fmt::Display for Shade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Shade {
    pub fn new(x: &str) -> Self {
        Self(x.to_string())
    }

    pub const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
