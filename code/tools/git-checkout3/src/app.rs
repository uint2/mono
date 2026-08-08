use crate::prelude::*;

pub enum Commit<'a> {
    /// Detached HEAD state.
    Head,
    Branch(&'a str),
}

pub struct App<'app> {
    cwd: CanonicalPath,
    worktree_states: Vec<WorktreeState<'app>>,
    git_head: Commit<'app>,
}

impl<'app> App<'app> {
    /*
    Keep a cache of which worktree has checked out which branch before.
         */
    pub fn checkout<'r>(&'r self, goal: &'r str) -> Option<&'r Path> {
        let x = self.cwd.as_path();
        Some(Path::new(goal))
    }
}

/// Single worktree. `git checkout` should behave completely normally.
mod single_worktree {
    use super::*;

    #[cfg(test)]
    const SOME_HEAD: &str = "bfc06870f25da37383255dd363ff2457d188ae6a";

    #[cfg(test)]
    fn init_single_worktree_app() -> App<'static> {
        App {
            cwd: CanonicalPath::mock("/home/app"),
            worktree_states: vec![WorktreeState::mock(
                "/home/app",
                Some(SOME_HEAD),
                Some("refs/heads/main"),
            )],
            git_head: Commit::Head,
        }
    }

    #[test]
    fn t01() {
        let app = init_single_worktree_app();
        assert_eq!(app.checkout("dev"), Some(Path::new("dev")))
    }

    #[test]
    fn t02() {
        let app = init_single_worktree_app();
        assert_eq!(
            app.checkout("src/main/java/Main.java"),
            Some(Path::new("src/main/java/Main.java"))
        )
    }
}
