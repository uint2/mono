use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Branch<'a>(&'a str);

impl fmt::Display for Branch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.0, f)
    }
}

impl<'a> Branch<'a> {
    pub const fn new(branch: &'a str) -> Self {
        Self(branch)
    }

    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}
