use crate::prelude::*;

const IGNORED_ATTRS: [&str; 2] = ["locked", "prunable"];

/// This is the worktree representation on the CLI. To separate the concept of
/// the worktree being a directory that can check out git branches, with the
/// whole bundle of state that `git worktree` prints out, we shall call this
/// a `Bundle`.
///
/// For more information, see [git docs](https://git-scm.com/docs/git-worktree).
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Bundle<'a> {
    pub worktree: Worktree<'a>,
    pub HEAD: Option<&'a str>,
    pub branch: Option<Branch<'a>>,
    pub detached: bool,
    pub bare: bool,
}

impl<'a> Bundle<'a> {
    pub const fn new(worktree: Worktree<'a>) -> Self {
        Self { worktree, HEAD: None, branch: None, detached: false, bare: false }
    }
}

/// Test setters.
#[cfg(test)]
impl<'a> Bundle<'a> {
    pub const fn head(mut self, value: &'a str) -> Self {
        self.HEAD = Some(value);
        self
    }

    pub const fn branch(mut self, value: &'a str) -> Self {
        self.branch = Some(Branch::new(value));
        self
    }

    pub const fn detached(mut self, value: bool) -> Self {
        self.detached = value;
        self
    }

    pub const fn bare(mut self, value: bool) -> Self {
        self.bare = value;
        self
    }
}

impl<'a> Bundle<'a> {
    pub fn parse_all(mut text: &'a str) -> Vec<Self> {
        let mut vec = vec![];
        while let Some((worktree, next)) = Self::parse(text) {
            vec.push(worktree);
            text = next;
            if text.bytes().all(|v| v.is_ascii_whitespace()) {
                break;
            }
        }
        vec
    }

    pub fn parse(text: &'a str) -> Option<(Self, &'a str)> {
        let text = text
            .strip_prefix("worktree")
            .expect("git worktree stdout should start with \"worktree\".");
        let text = text.trim_start();
        let (worktree, mut text) = text
            .split_once('\n')
            .expect("git worktree path should follow after \"worktree\".");

        let mut bundle = Bundle::new(Worktree::new(worktree));

        while let Some((line, remaining)) = text.split_once('\n') {
            if line.is_empty() {
                return Some((bundle, remaining.trim_start()));
            }
            match line.split_once(' ') {
                Some(("HEAD", value)) => bundle.HEAD = Some(value),
                Some(("branch", value)) => bundle.branch = Some(Branch::new(value)),
                Some((key, value)) => {
                    log::info!("ignored attribute: key: {key}, value: {value}")
                }
                None => match line {
                    "detached" => bundle.detached = true,
                    "bare" => bundle.bare = true,
                    value => log::info!("ignored attribute: {value}"),
                },
            }
            text = remaining;
        }
        panic!("Unreachable state");
    }
}
