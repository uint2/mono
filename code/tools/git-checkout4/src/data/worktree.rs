use crate::prelude::*;

/// The directory line of the stdout output of `git worktree list --porcelain`.
///
/// Should be a full path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Worktree<'a>(&'a str);

impl fmt::Display for Worktree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.0, f)
    }
}

impl<'a> Worktree<'a> {
    pub fn new(path: &'a str) -> Self {
        assert!(Path::new(path).is_absolute());
        Self(path)
    }

    pub fn as_path(&self) -> &'a Path {
        Path::new(self.0)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    /// Splits along the right-most path separator. This is to be able to
    /// highlight the direct parent directory in a different color.
    pub fn pretty_split(&self) -> (&'a str, &'a str) {
        let j = self.0.rfind(std::path::MAIN_SEPARATOR).unwrap();
        self.0.split_at(j + 1)
    }

    #[inline]
    fn is_subdirectory_of(&self, other: &Self) -> bool {
        self.as_path().starts_with(other.as_path())
    }

    /// Checks to see if a worktree still exists.
    pub fn exists(&self, all_bundles: &[Bundle]) -> bool {
        all_bundles.iter().any(|v| v.worktree.eq(self))
    }
}

impl AsRef<Path> for Worktree<'_> {
    fn as_ref(&self) -> &Path {
        Path::new(self.0)
    }
}

/// Worktrees are to be ordered by directory inclusion. A <= B if and only if
/// A is a subdirectory of B.
impl PartialOrd for Worktree<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let le = self.is_subdirectory_of(other);
        let ge = other.is_subdirectory_of(self);
        match (le, ge) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            _ => None,
        }
    }
}
