use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Worktree<'a>(&'a str);

impl fmt::Display for Worktree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.0, f)
    }
}

impl<'a> Worktree<'a> {
    pub const fn new(path: &'a str) -> Self {
        Self(path)
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_path(&self) -> &'a Path {
        Path::new(self.0)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }
}
