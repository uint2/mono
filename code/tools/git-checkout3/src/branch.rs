use crate::prelude::*;

pub struct Branch(String);

impl fmt::Debug for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Branch {
    pub const fn new(value: String) -> Self {
        Self(value)
    }

    pub const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
