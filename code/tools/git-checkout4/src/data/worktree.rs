use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub const fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn last_component(&self) -> &'a str {
        let Some(j) = self.0.rfind(std::path::MAIN_SEPARATOR) else { return self.0 };
        &self.0[j + 1..]
    }

    pub fn pretty_split(&self) -> (&'a str, &'a str) {
        let j = self.0.rfind(std::path::MAIN_SEPARATOR).unwrap();
        self.0.split_at(j + 1)
    }
}

impl AsRef<Path> for Worktree<'_> {
    fn as_ref(&self) -> &Path {
        Path::new(self.0)
    }
}
