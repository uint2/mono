//! git-checkout3
//!
//! For some context on how git resolves pathspecs:
//!   1. When the current branch is "dev" and there is a modified file called
//!      "dev" and we run `git checkout dev`, then git responds with
//!      "Already on 'dev'" and does not edit the "dev" file.
//!   2. When the current branch is "dev" and there is a modified file called
//!      "main" and we run `git checkout main`, then git responds with
//!      "Switched to branch 'main'" and does not edit the "main" file. Of
//!      course, if switching branches causes "main" to be overwritten, git will
//!      instead respond with
//!
//!        error: Your local changes to the following files would be overwritten by checkout:
//!          main
//!        Please commit your changes or stash them before you switch branches.
//!        Aborting
//!
//! Now, there are 5 entities that we are concerned with in git-checkout3. These
//! are:
//!
//!   1. The current branch.          c_branch
//!   2. The current directory.       c_dir
//!   3. The worktree's branch.       w_branch
//!   4. The worktree's directory.    w_dir
//!   5. The goal.                    goal
//!
//! The problem we're trying to solve is when we're working with git worktrees,
//! and we run into the scenario where the "dev" directory has the "main" branch
//! checked out, and vice versa, and this required a dance with a third branch
//! in order to swap the branches back.
//!
//! To prevent that, in `git-checkout3`, we shall only allow checking out to
//! branches that do not match any of the existing worktree's directory names.
//!
//! Moreover, we also implement prefix constraits, where in the git config we
//! can specify a comma-separated list of strings under the key
//! "checkout.const-prefix". For each of these prefixes, branches that start
//! with them can only be checked out at the worktree whose directory matches
//! the prefix. For example, we can have a worktree dedicated to Pull Requests,
//! and name that directory as "pr". Then, we add "pr" to the (comma-separated)
//! list of "checkout.const-prefix". Now, branches like "pr-140" and "pr-bugfix"
//! can be and only be checked out at the "pr" directory. That is, the "dev"
//! worktree directory will not be allowed to check out branches with the "pr"
//! prefix.
//!
//! Or, to simplify, let W be the set of all worktrees. Let B be the set of all
//! branches. At all times we would like there to establish a mapping f : B → W
//! such that all checkouts operate as the inverse of this function. Every
//! branch should only be allowed to be checked out by exactly one worktree.
//!
//! [idea] to implement this, git-checkout3 can track which branch was created
//! under, and when the worktree is deleted, delete all the branches that came
//! with that worktree. Store this information in local git config.

macro_rules! git {
    ($($arg:expr),*) => {{
        let mut cmd = std::process::Command::new("git");
        cmd$(.arg($arg))* ;
        cmd
    }};
}

macro_rules! err {
    ($($arg:tt)*) => {{
        Err(eprintln!($($arg)*))
    }};
}

mod app;
mod branch;
mod canonical_path;
mod config;
mod consts;
mod prelude;
mod shell;
mod worktree;

use prelude::*;

#[inline]
fn getcwd() -> Result<PathBuf, ()> {
    env::current_dir().map_err(|_| eprintln!("Unable to get current working directory."))
}

fn canonical_eq(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

fn get_git_config_const_prefix() -> Result<Output, ()> {
    git!("config", "--get", CONST_PREFIX_CONFIG_KEY)
        .output()
        .map_err(|_| eprintln!("Failed to execute shell command to get git config."))
}

/// Gets the git branch, and if we're currently in detached HEAD state, it will
/// print HEAD.
fn get_git_branch() -> Result<Output, ()> {
    git!("rev-parse", "--abbrev-ref", "--symbolic-full-name", "HEAD")
        .output()
        .map_err(|_| eprintln!("Failed to execute shell command to get git branch."))
}

fn get_git_worktrees() -> Result<Output, ()> {
    git!("worktree", "list", "--porcelain")
        .output()
        .map_err(|_| eprintln!("Failed to execute shell command to get git worktrees."))
}

fn try_main(goal: &str) -> Result<ExitCode, ()> {
    let mut sticky_output: Result<Output, ()> = Err(());
    let mut git_branch_output: Result<Output, ()> = Err(());
    let mut git_worktree_output: Result<Output, ()> = Err(());
    let mut cwd: Result<CanonicalPath, ()> = Err(());

    rayon::scope(|scope| {
        scope.spawn(|_| sticky_output = get_git_config_const_prefix());
        scope.spawn(|_| git_branch_output = get_git_branch());
        scope.spawn(|_| git_worktree_output = get_git_worktrees());
        scope.spawn(|_| cwd = CanonicalPath::current_dir());
    });
    let sticky_output = sticky_output?;
    let git_branch_output = git_branch_output?;
    let git_worktree_output = git_worktree_output?;
    let cwd = cwd?;

    let x = Config::load()?;
    x.save();
    println!("{x:?}");

    if true {
        return Err(());
    }

    {
        // Firstly, make sure that we're in a git-enabled directory.
        let Ok(stderr) = str::from_utf8(&git_worktree_output.stderr) else {
            return err!("Unable to decode `git worktree` stderr as utf-8.");
        };
        if stderr.starts_with("fatal: not a git repository") {
            // fatal: not a git repository (or any parent up to mount point)
            // fatal: not a git repository (or any of the parent directories)
            io::stderr().write(&git_worktree_output.stderr).unwrap();
            return Ok(ExitCode::of(git_worktree_output.status));
        }
    }

    // Parse the shell outputs.
    let Ok(sticky_branches) = str::from_utf8(&sticky_output.stdout) else {
        return err!("Unable to decode sticky config as utf-8.");
    };
    let sticky_branches = sticky_branches.trim().split(',').collect::<Vec<_>>();
    let Ok(current_branch) = str::from_utf8(&git_branch_output.stdout) else {
        return err!("Unable to decode `git branch` stdout as utf-8.");
    };
    let current_branch = current_branch.trim();

    let Ok(worktrees) = str::from_utf8(&git_worktree_output.stdout) else {
        return err!("Unable to decode `git worktree` stdout as utf-8.");
    };
    let worktrees = worktrees.trim();

    let is_branch_sticky = sticky_branches.iter().any(|&v| current_branch == v);

    let worktrees = WorktreeState::parse(worktrees)?;
    let parent_worktree = WorktreeState::find_closest_parent(&cwd, &worktrees);

    // 1. Prioritize the directory match. Look through the worktrees and match
    // the directory of each worktree checked out against `goal`.
    for worktree in &worktrees {
        if worktree.directory() == goal {
            if &worktree.abs_path == &cwd {
                // The goal is the same as the worktree's dir, i.e. cwd == goal,
                // and also we're checking out `goal`.
                shell::run(git!("checkout", goal));
            } else {
                return worktree.accept_and_resolve(&cwd, &worktrees);
            }
        }
    }

    // 2. Find a match in the branch. This is done _intentionally_ before
    // running git checkout on the goal directly.
    for worktree in &worktrees {
        let Some(branch) = worktree.branch else { continue };
        if branch == goal {
            return worktree.accept_and_resolve(&cwd, &worktrees);
        }
    }

    shell::run(git!("checkout", goal));
}

fn main() -> std::process::ExitCode {
    log::init(Some(log::LevelFilter::Trace));

    // To keep things simple, we only run the complicated logic when there is
    // exactly 1 CLI argument (that is not the binary itself).
    let args: Vec<_> = env::args_os().skip(1).collect();
    let 1 = args.len() else {
        let mut cmd = Command::new("git");
        cmd.arg("checkout");
        cmd.args(args);
        shell::run(cmd);
    };
    let Some(goal) = args[0].to_str() else {
        eprintln!("Failed to decode target.");
        return std::process::ExitCode::FAILURE;
    };
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    match pool.install(|| try_main(goal.trim())) {
        Ok(v) => v.exit(),
        Err(()) => return std::process::ExitCode::FAILURE,
    }
}
