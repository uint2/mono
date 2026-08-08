use crate::prelude::*;

#[derive(Debug)]
pub struct Worktree<'a>(&'a str);

impl<'a> Worktree<'a> {
    pub const fn new(path: &'a str) -> Self {
        Self(path)
    }

    pub fn as_path(&self) -> &'a Path {
        Path::new(self.0)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }
}
