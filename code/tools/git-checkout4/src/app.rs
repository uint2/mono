use crate::prelude::*;

/// Gets the git branch, and if we're currently in detached HEAD state, it will
/// print HEAD.
fn get_git_branch() -> Result<String, ()> {
    let output = git!("rev-parse", "--abbrev-ref", "--symbolic-full-name", "HEAD")
        .output()
        .map_err(|_| eprintln!("Failed to execute shell command to get git branch."))?;
    let mut output = String::from_utf8(output.stdout)
        .map_err(|_| eprintln!("Failed to parsed git branch"))?;
    output.truncate(output.as_str().trim_end().len());
    Ok(output)
}

fn get_git_branches() -> Result<String, ()> {
    let output = git!("branch", "--format=%(refname:short)")
        .output()
        .map_err(|_| eprintln!("Failed to execute shell command to get git branches."))?;
    let mut output = String::from_utf8(output.stdout)
        .map_err(|_| eprintln!("Failed to parsed git branches"))?;
    output.truncate(output.as_str().trim_end().len());
    Ok(output)
}

fn get_git_worktrees() -> Result<String, ()> {
    let output = git!("worktree", "list", "--porcelain").output().map_err(|_| {
        eprintln!("Failed to execute shell command to get git worktrees.")
    })?;
    String::from_utf8(output.stdout)
        .map_err(|_| eprintln!("Failed to parsed git worktrees"))
}

fn current_bundle<'r, 'a>(
    cwd: &Path,
    bundles: &'r [Bundle<'a>],
) -> Option<&'r Bundle<'a>> {
    bundles
        .iter()
        .filter(|b| cwd.starts_with(b.worktree.as_path()))
        .max_by(|a, b| a.worktree.len().cmp(&b.worktree.len()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome<'a> {
    // Jump to a directory.
    Jump(&'a Path),
    /// Checkout a branch.
    Checkout(Branch<'a>),
    /// Jump first, then checkout.
    JumpAndCheckout(&'a Path, Branch<'a>),
    /// Complete bypass.
    Bypass(&'a str),
}

pub struct App {
    /// Current working directory.
    cwd: PathBuf,
    /// Raw output of `git rev-parse --abbrev-ref --symbolic-full-name HEAD`.
    /// Shows the current branch, or "HEAD" if HEAD is detached.
    r_git_branch: String,
    /// Raw output of `git branch --format=%(refname:short)`.
    /// Shows the all the local branches, separated by newlines.
    r_git_branches: String,
    /// Raw output of `git worktree list --porcelain`.
    /// https://git-scm.com/docs/git-worktree
    r_git_worktree_list: String,

    r_git_config: String,
}

impl App {
    pub fn init() -> Result<Self, ()> {
        let mut r_git_branch = Err(());
        let mut r_git_branches = Err(());
        let mut r_git_worktree_list = Err(());
        let mut cwd = Err(());
        let mut r_git_config = String::new();

        rayon::scope(|scope| {
            scope.spawn(|_| r_git_branch = get_git_branch());
            scope.spawn(|_| r_git_branches = get_git_branches());
            scope.spawn(|_| r_git_worktree_list = get_git_worktrees());
            scope.spawn(|_| {
                cwd = std::env::current_dir()
                    .map_err(|_| eprintln!("Unable to get current dir"))
            });
            scope.spawn(|_| r_git_config = Config::read());
        });
        let cwd = cwd?;
        let r_git_branch = r_git_branch?;
        let r_git_branches = r_git_branches?;
        let r_git_worktree_list = r_git_worktree_list?;
        Ok(Self { cwd, r_git_branches, r_git_branch, r_git_worktree_list, r_git_config })
    }

    pub fn branches<'a>(&'a self) -> Vec<Branch<'a>> {
        self.r_git_branches.trim().lines().map(Branch::new).collect()
    }

    pub fn config<'a>(&'a self) -> Config<'a, 'a> {
        Config::parse(self.r_git_config.as_str()).unwrap()
    }

    pub fn bundles<'a>(&'a self) -> Vec<Bundle<'a>> {
        Bundle::parse_all(self.r_git_worktree_list.as_str())
    }

    pub fn cwd(&self) -> &Path {
        self.cwd.as_path()
    }

    pub fn execute<'a>(&'a self, goal: &'a str) -> Outcome<'a> {
        let git_branches = self.branches();
        let mut config = self.config();

        // log::info!("cwd = {cwd:?}");
        // log::info!("branch: {git_branch_output:?}");
        // log::info!("branches: {git_branches:?}");
        // log::info!("worktrees:\n---\n{git_worktree_output}\n---");

        log::info!("config: {config:?}");

        let bundles = self.bundles();
        for (i, wt) in bundles.iter().enumerate() {
            log::info!("[{i}] {wt:?}")
        }

        let mut read_buf = String::new();
        for branch in &git_branches {
            if let None = config.get(branch) {
                println!("Branch {branch} is not mapped to any worktree.");
                for (idx, bundle) in bundles.iter().enumerate() {
                    println!("[{idx}] {}", bundle.worktree.as_str())
                }
                print!("Pick one > ");
                _ = io::stdout().flush();
                read_buf.clear();
                io::stdin().read_line(&mut read_buf).unwrap();
                let choice = read_buf.trim().parse::<usize>().unwrap();
                let choice = &bundles[choice];
                config.set(*branch, choice.worktree);
                config.save();
                log::info!("Saved: {branch}");
            }
            assert!(config.get(branch).is_some())
        }

        if Path::new(goal).exists() {
            return Outcome::Bypass(goal);
        }

        let current_bundle = bundles
            .iter()
            .filter(|b| self.cwd().starts_with(b.worktree.as_path()))
            .max_by(|a, b| a.worktree.len().cmp(&b.worktree.len()))
            .expect("This operation should be run in a worktree");

        // Make sure that the goal is a branch that exists. Otherwise, default to
        // `git checkout` behavior.
        let Some(goal) = git_branches.iter().find(|v| v.as_str() == goal) else {
            return Outcome::Bypass(goal);
        };

        // Unwrap is safe as long as we've already made sure that every branch
        // belongs to a worktree, as done above.
        let goal_worktree = config.get(goal).unwrap();

        if goal_worktree.as_str() == current_bundle.worktree.as_str() {
            return Outcome::Bypass(goal.as_str());
        } else {
            println!("{goal}|||{goal_worktree}");
            return Outcome::JumpAndCheckout(goal_worktree.as_path(), *goal);
        }
    }
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

    // #[cfg(test)]
    // fn init_single_worktree_app() -> App<'static> {
    //     App {
    //         cwd: Path::new("/home/khang/repos/neovim"),
    //         bundles: vec![Bundle::new(Worktree::new("main")).detached(true)],
    //         current_branch: None,
    //     }
    // }

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
