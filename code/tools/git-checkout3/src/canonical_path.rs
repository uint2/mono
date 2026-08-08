#![allow(unused)]

use crate::prelude::*;

/// A (absolute) path in canonical form. Makes it safe for comparisons.
#[derive(PartialEq, Eq, Hash)]
pub struct CanonicalPath(PathBuf);

impl fmt::Debug for CanonicalPath {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for CanonicalPath {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0.display(), f)
    }
}

impl CanonicalPath {
    #[cfg(test)]
    pub const EMPTY: Self = Self(PathBuf::new());

    pub fn new(path: &Path) -> Option<Self> {
        path.canonicalize().ok().map(Self)
    }

    #[cfg(test)]
    pub unsafe fn new_unchecked(path: &Path) -> Option<Self> {
        Some(Self(path.to_path_buf()))
    }

    #[cfg(test)]
    pub fn mock(path: &str) -> Self {
        Self(PathBuf::from(path))
    }

    pub fn current_dir() -> Result<Self, ()> {
        std::env::current_dir()
            .map_err(|_| eprintln!("Unable to get current directory."))
            .map(|v| CanonicalPath::new(&v).unwrap())
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.0.join(path)
    }

    pub fn len(&self) -> usize {
        self.0.as_os_str().len()
    }

    pub fn exists(&self) -> bool {
        self.0.exists()
    }

    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }

    pub fn strip_prefix(&self, other: &Self) -> Option<&Path> {
        if self <= other {
            return Some(self.0.strip_prefix(&other.0).unwrap());
        }
        None
    }
}

/// If A is a subdirectory of B, then we write A <= B. If A is a strict
/// subdirectory of B, then we write A < B.
///
/// Not all paths can be compared, and so we shall NOT implement Ord.
impl PartialOrd for CanonicalPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs = self.0.as_path();
        let rhs = other.0.as_path();
        if lhs == rhs {
            return Some(Ordering::Equal);
        } else if lhs.starts_with(rhs) {
            // lhs is a subdirectory of rhs.
            return Some(Ordering::Less);
        } else if rhs.starts_with(lhs) {
            // rhs is a subdirectory of lhs.
            return Some(Ordering::Greater);
        } else {
            None
        }
    }
}
