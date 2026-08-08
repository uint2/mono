use crate::prelude::*;

/// A unique git worktree.
#[derive(PartialEq, Eq, Hash)]
pub struct Worktree<'a>(&'a str);

impl<'a> Worktree<'a> {
    pub const fn new(path: &'a str) -> Self {
        Self(path)
    }
}

#[derive(Debug)]
pub struct WorktreeState<'a> {
    /// Absolute path to the worktree. Canonicalized.
    pub abs_path: CanonicalPath,
    /// Absolute path to the worktree. Raw `git worktree` output.
    pub abs_path_str: &'a str,
    /// The branch. Parsed from one of
    /// * "HEAD <SHA-1>",
    /// * "bare".
    pub head: Option<&'a str>,
    /// The branch. Parsed from one of
    /// * "branch refs/heads/main",
    /// * "detached".
    ///
    /// The other cases are just not considered. We really only care when the
    /// branch ref actually exists.
    pub branch: Option<&'a str>,
}

impl<'a> WorktreeState<'a> {
    pub fn directory(&self) -> &str {
        self.abs_path_str.rsplit_once(std::path::MAIN_SEPARATOR).unwrap().1
    }

    #[cfg(test)]
    pub fn mock(
        abs_path: &'a str,
        head: Option<&'a str>,
        branch: Option<&'a str>,
    ) -> Self {
        Self {
            abs_path: CanonicalPath::mock(abs_path),
            abs_path_str: abs_path,
            head,
            branch,
        }
    }

    pub fn parse(text: &'a str) -> Result<Vec<Self>, ()> {
        let mut worktrees = vec![];
        /// Looking For.
        enum LF {
            /// Looking for "worktree".
            Worktree,
            /// Looking for "HEAD". Might see "bare".
            Head,
            /// Looking for "branch", followed by an absolute path.
            /// Might see "detached".
            Directory,
        }
        let mut state = LF::Worktree;
        for line in text.lines() {
            match state {
                LF::Worktree => {
                    let Some(line) = line.strip_prefix("worktree") else {
                        eprintln!(
                            "The first line of each worktree must start with \"worktree\"."
                        );
                        return Err(());
                    };
                    let abs_path_str = line.trim_start();
                    let abs_path = CanonicalPath::new(Path::new(abs_path_str)).unwrap();
                    worktrees.push(WorktreeState {
                        abs_path,
                        abs_path_str,
                        head: None,
                        branch: None,
                    });
                    state = LF::Head;
                }
                LF::Head => {
                    if line.trim() == "bare" {
                        state = LF::Directory;
                        continue;
                    }
                    let Some(line) = line.strip_prefix("HEAD") else {
                        eprintln!(
                            "The second line of each worktree must start with \"HEAD\"."
                        );
                        return Err(());
                    };
                    worktrees.last_mut().unwrap().head = Some(line.trim_start());
                    state = LF::Directory;
                }
                LF::Directory if line.is_empty() => state = LF::Worktree,
                LF::Directory => {
                    if let Some(line) = line.strip_prefix("branch") {
                        // example: refs/heads/main
                        let full_ref_name = line.trim_start();
                        let branch = full_ref_name.strip_prefix("refs/heads/");
                        worktrees.last_mut().unwrap().branch = branch
                    }
                }
            }
        }
        Ok(worktrees)
    }

    pub fn find_closest_parent<'t>(
        cwd: &CanonicalPath,
        trees: &'t [Self],
    ) -> Option<&'t Self> {
        trees
            .iter()
            .filter(|t| t.branch.is_some())
            .filter(|t| cwd <= &t.abs_path)
            .max_by(|a, b| a.abs_path.len().cmp(&b.abs_path.len()))
    }

    pub fn accept_and_resolve(
        &self,
        cwd: &CanonicalPath,
        trees: &[Self],
    ) -> Result<ExitCode, ()> {
        let parent_tree = match Self::find_closest_parent(&cwd, trees) {
            Some(v) => v,
            None => {
                io::stdout().write(self.abs_path_str.as_bytes()).unwrap();
                return Ok(ExitCode::ACCEPT);
            }
        };
        let relpath = cwd.strip_prefix(&parent_tree.abs_path).unwrap();
        let mut target = self.abs_path.join(relpath);
        while !target.exists() {
            target.pop();
        }
        let target = target.to_str().unwrap();
        io::stdout().write(target.as_bytes()).unwrap();
        Ok(ExitCode::ACCEPT)
    }
}
