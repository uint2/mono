use crate::prelude::*;

pub struct App<'app> {
    cwd: &'app Path,
    bundles: Vec<Bundle<'app>>,
    /// None means that we're currently in detached HEAD state.
    current_branch: Option<Branch<'app>>,
}

// impl<'app> App<'app> {
//     /*
//     Keep a cache of which worktree has checked out which branch before.
//          */
//     pub fn checkout<'r>(&'r self, goal: &'r str) -> Option<&'r Path> {
//         let x = self.cwd.as_path();
//         Some(Path::new(goal))
//     }
// }

/// Single worktree. `git checkout` should behave completely normally.
mod single_worktree {
    use super::*;

    #[cfg(test)]
    const SOME_HEAD: &str = "cd90ec7cdcdc55b617dfae5317b2c24b76b4148a";

    #[cfg(test)]
    fn init_single_worktree_app() -> App<'static> {
        App {
            cwd: Path::new("/home/khang/repos/neovim"),
            bundles: vec![Bundle::new(Worktree::new("main")).detached(true)],
            current_branch: None,
        }
    }

    // #[test]
    // fn t01() {
    //     let app = init_single_worktree_app();
    //     assert_eq!(app.checkout("dev"), Some(Path::new("dev")))
    // }
    //
    // #[test]
    // fn t02() {
    //     let app = init_single_worktree_app();
    //     assert_eq!(
    //         app.checkout("src/main/java/Main.java"),
    //         Some(Path::new("src/main/java/Main.java"))
    //     )
    // }
}
