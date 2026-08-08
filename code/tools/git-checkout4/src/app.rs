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

pub struct AppConfig {
    pub enable_logging: bool,
    pub log_level: log::LevelFilter,
    pub interactive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome<'a> {
    // Jump to a directory.
    Jump(&'a Path),
    /// Checkout a branch.
    Checkout(Branch<'a>),
    /// Jump first, then checkout.
    JumpAndCheckout {
        worktree: Worktree<'a>,
        branch: Branch<'a>,
        relpath: &'a Path,
    },
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

    config: AppConfig,
}

impl App {
    pub fn init(config: AppConfig) -> Result<Self, ()> {
        if config.enable_logging {
            log::init(Some(config.log_level));
        }
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
            scope.spawn(|_| r_git_config = GitConfig::read());
        });
        let cwd = cwd?;
        let r_git_branch = r_git_branch?;
        let r_git_branches = r_git_branches?;
        let r_git_worktree_list = r_git_worktree_list?;
        Ok(Self {
            cwd,
            r_git_branches,
            r_git_branch,
            r_git_worktree_list,
            r_git_config,
            config,
        })
    }

    pub fn branches<'a>(&'a self) -> Vec<Branch<'a>> {
        self.r_git_branches.trim().lines().map(Branch::new).collect()
    }

    pub fn git_config<'a>(&'a self) -> GitConfig<'a, 'a> {
        GitConfig::parse(self.r_git_config.as_str()).unwrap()
    }

    pub fn bundles<'a>(&'a self) -> Vec<Bundle<'a>> {
        Bundle::parse_all(self.r_git_worktree_list.as_str())
    }

    pub fn cwd(&self) -> &Path {
        self.cwd.as_path()
    }

    pub fn find_bundle_by_branch<'a>(&'a self, branch: Branch) -> Option<Bundle<'a>> {
        self.bundles().into_iter().find(|v| v.branch == Some(branch))
    }

    pub fn get_worktree_dir<'a>(&'a self, branch: Branch) -> Option<&'a Path> {
        self.bundles()
            .into_iter()
            .find(|v| v.branch == Some(branch))
            .map(|v| v.worktree.as_path())
    }

    pub fn get_worktree<'a>(&'a self, branch: Branch) -> Option<Worktree<'a>> {
        self.bundles().into_iter().find(|v| v.branch == Some(branch)).map(|v| v.worktree)
    }

    /// Auto-register branches that match their worktree.
    fn auto_register<'a>(
        git_branches: &[Branch<'a>],
        bundles: &[Bundle<'a>],
        git_config: &mut GitConfig<'a, 'a>,
    ) {
        for &branch in git_branches {
            if let Some(bundle) = bundles.iter().find(|v| v.branch == Some(branch)) {
                if bundle.worktree.as_path().ends_with(branch.as_str()) {
                    git_config.set(branch, bundle.worktree);
                }
            }
        }
    }

    fn prompt_user_for_worktree<'a>(
        input_buf: &mut String,
        branch: Branch<'a>,
        bundles: &[Bundle<'a>],
        git_config: &mut GitConfig<'a, 'a>,
    ) {
        let mut stdout = io::stdout().lock();
        writeln!(stdout, "Branch {branch} is not mapped to any worktree.").unwrap();
        for (idx, bundle) in bundles.iter().enumerate() {
            writeln!(stdout, "[{idx}] {}", bundle.worktree.as_str()).unwrap();
        }
        write!(stdout, "Pick one > ").unwrap();
        _ = stdout.flush();
        drop(stdout);

        input_buf.clear();
        io::stdin().read_line(input_buf).unwrap();
        let choice = input_buf.trim().parse::<usize>().unwrap();
        let choice = &bundles[choice];
        git_config.set(branch, choice.worktree);
    }

    fn resolve_relpath<'a>(
        cwd: &'a Path,
        current_bundle: &Bundle,
        mapped_worktree: &Worktree,
    ) -> &'a Path {
        let mut subdir_in_repo = cwd.strip_prefix(&current_bundle.worktree).unwrap();
        let base = mapped_worktree.as_path();
        let mut final_dest = base.join(subdir_in_repo);
        loop {
            if final_dest == base {
                return Path::new("");
            } else if final_dest.is_dir() {
                return subdir_in_repo;
            }
            final_dest.pop(); // Move upwards together.
            subdir_in_repo = subdir_in_repo.parent().unwrap();
        }
    }

    pub fn execute<'a>(&'a self, goal: &'a str) -> Outcome<'a> {
        // #[cfg(test)]
        // env::set_current_dir(&self.cwd).unwrap();

        let git_branches = self.branches();
        let mut git_config = self.git_config();
        let bundles = self.bundles();

        // Retain only the valid worktrees.
        git_config.retain(|_, worktree| bundles.iter().any(|b| b.worktree == *worktree));

        Self::auto_register(&git_branches, &bundles, &mut git_config);

        let mut input_buf = String::new();
        for branch in &git_branches {
            if git_config.get(branch).is_none() && self.config.interactive {
                Self::prompt_user_for_worktree(
                    &mut input_buf,
                    *branch,
                    &bundles,
                    &mut git_config,
                );
            }
            if self.config.interactive {
                assert!(git_config.get(branch).is_some())
            }
        }

        // if Path::new(goal).exists() {
        //     log::info!("Goal \"{goal}\" exists as a file.");
        //     return Outcome::Bypass(goal);
        // }

        let current_bundle = bundles
            .iter()
            .filter(|b| self.cwd().starts_with(b.worktree.as_path()))
            .max_by(|a, b| a.worktree.len().cmp(&b.worktree.len()))
            .expect("This operation should be run in a worktree");

        // Make sure that the goal is a branch that exists. Otherwise, default
        // to `git checkout` behavior.
        //
        // This execution branchs should handle the cases where the user is
        // trying to checkout a particular file in the worktree.
        let Some(goal) = git_branches.iter().find(|v| v.as_str() == goal) else {
            log::info!("Goal \"{goal}\" is not a git branch, might be a file.");
            return Outcome::Bypass(goal);
        };

        // Get the worktree that the `goal` branch belongs to.
        let Some(mapped_worktree) = git_config.get(goal) else {
            panic!("Goal \"{goal}\" is not configred in git config.")
        };

        if mapped_worktree.as_str() == current_bundle.worktree.as_str() {
            // We permit the user to checkout the `goal` branch, since it
            // belongs to this worktree.
            log::info!("Goal \"{goal}\" belongs to worktree. Allow checkout.");
            return Outcome::Bypass(goal.as_str());
        }

        // We do not permit the user to checkout the `goal` branch on this
        // current worktree, but instead direct the user to cd to that
        // owning worktree first, before jumping to the `goal` branch.

        // Here, we shall help the shell assistant a bit by first checking out
        // the `goal` branch in the mapped worktree, and then figuring out the
        // best directory to cd to.
        _ = git!("-C", mapped_worktree.as_str(), "checkout", goal.as_str()).output();

        let relpath = Self::resolve_relpath(&self.cwd, current_bundle, mapped_worktree);

        log::info!("Jump worktree then jump branch -> {relpath:?}");
        return Outcome::JumpAndCheckout {
            worktree: *mapped_worktree,
            branch: *goal,
            relpath,
        };
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
