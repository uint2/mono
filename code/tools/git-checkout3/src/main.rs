//! git-checkout3
//!
//! Prioritize directory names over branch names.

macro_rules! git {
    ($($arg:expr),*) => {{
        let mut cmd = std::process::Command::new("git");
        cmd$(.arg($arg))* ;
        cmd
    }};
}

mod consts;
mod shell;

use consts::{STICKY_CONFIG_KEY, STICKY_NO_JUMP};
use shell::ExitCode;

use core::str;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
struct GitWorktree<'a> {
    /// Absolute path to the worktree.
    abs_path: &'a str,
    head: Result<&'a str, ()>,
    /// The branch. Parsed from one of
    /// * "branch refs/heads/main",
    /// * "detached".
    ///
    /// The other cases are just not considered. We really only care when the
    /// branch ref actually exists.
    branch: Option<&'a str>,
}

macro_rules! err {
    ($($arg:tt)*) => {{
        Err(eprintln!($($arg)*))
    }};
}

impl<'a> GitWorktree<'a> {
    pub fn directory(&self) -> &'a str {
        match self.abs_path.rsplit_once(std::path::MAIN_SEPARATOR) {
            Some((_, dir)) => dir,
            None => self.abs_path,
        }
    }

    pub fn parse(text: &'a str) -> Result<Vec<Self>, ()> {
        let mut worktrees = vec![];
        let mut state: u8 = 0;
        for line in text.lines() {
            match state {
                0 => {
                    let Some(line) = line.strip_prefix("worktree") else {
                        eprintln!(
                            "The first line of each worktree must start with \"worktree\"."
                        );
                        return Err(());
                    };
                    let abs_path = line.trim_start();
                    worktrees.push(GitWorktree { abs_path, head: Err(()), branch: None });
                    state = 1;
                }
                1 => {
                    let Some(line) = line.strip_prefix("HEAD") else {
                        eprintln!(
                            "The second line of each worktree must start with \"HEAD\"."
                        );
                        return Err(());
                    };
                    worktrees.last_mut().unwrap().head = Ok(line.trim_start());
                    state = 2;
                }
                2 if line.is_empty() => state = 0,
                2 => {
                    if let Some(line) = line.strip_prefix("branch") {
                        // example: refs/heads/main
                        let full_ref_name = line.trim_start();
                        let branch = full_ref_name.strip_prefix("refs/heads/");
                        worktrees.last_mut().unwrap().branch = branch
                    }
                }
                _ => return Err(eprintln!("Invalid git worktree parser state.")),
            }
        }
        Ok(worktrees)
    }

    fn find_closest_parent<'t>(abs_cwd: &Path, trees: &'t [Self]) -> Option<&'t Self> {
        trees
            .iter()
            .filter(|t| t.branch.is_some())
            .filter(|t| abs_cwd.starts_with(t.abs_path))
            .max_by(|a, b| a.abs_path.len().cmp(&b.abs_path.len()))
    }

    pub fn accept_and_resolve(&self, trees: &[Self]) -> Result<ExitCode, ()> {
        let Ok(cwd) = std::env::current_dir() else {
            return err!("Unable to get current working directory.");
        };

        let parent_tree = match Self::find_closest_parent(&cwd, trees) {
            Some(v) => v,
            None => {
                io::stdout().write(self.abs_path.as_bytes()).unwrap();
                return Ok(ExitCode::ACCEPT);
            }
        };
        let relpath = cwd.strip_prefix(parent_tree.abs_path).unwrap();
        let mut target = Path::new(self.abs_path).join(relpath);
        while !target.exists() {
            target.pop();
        }
        let target = target.to_str().unwrap();
        io::stdout().write(target.as_bytes()).unwrap();
        Ok(ExitCode::ACCEPT)
    }
}

fn try_main(goal: &str) -> Result<ExitCode, ()> {
    let Ok(worktrees) = git!("worktree", "list", "--porcelain").output() else {
        return err!("Failed to execute shell command to get git worktrees.");
    };
    let Ok(stderr) = str::from_utf8(&worktrees.stderr) else {
        return err!("Unable to decode `git worktree` stderr as utf-8.");
    };
    if stderr.starts_with("fatal: not a git repository") {
        // fatal: not a git repository (or any parent up to mount point)
        // fatal: not a git repository (or any of the parent directories)
        io::stderr().write(&worktrees.stderr).unwrap();
        return Ok(ExitCode::of(worktrees.status));
    }
    let Ok(stdout) = str::from_utf8(&worktrees.stdout) else {
        return err!("Unable to decode `git worktree` stdout as utf-8.");
    };
    let worktrees = GitWorktree::parse(stdout)?;

    // 1. Prioritize the directory match.
    for worktree in &worktrees {
        if worktree.directory() == goal {
            return worktree.accept_and_resolve(&worktrees);
        }
    }

    // 2. Find a match in the branch. This is done _intentionally_ before
    // running git checkout on the goal directly.
    for worktree in &worktrees {
        let Some(branch) = worktree.branch else { continue };
        if branch == goal {
            return worktree.accept_and_resolve(&worktrees);
        }
    }

    let current_branch_is_sticky = {
        let Ok(sticky) = git!("config", "get", STICKY_CONFIG_KEY).output() else {
            return err!("Failed to execute shell command to get git config.");
        };
        let Ok(sticky) = str::from_utf8(&sticky.stdout) else {
            return err!("Unable to decode sticky config as utf-8.");
        };
        let Ok(current_branch) = git!("branch", "--show-current").output() else {
            return err!("Failed to execute shell command to get git branch.");
        };
        let Ok(current_branch) = str::from_utf8(&current_branch.stdout) else {
            return err!("Unable to decode git branch as utf-8.");
        };
        let current_branch = current_branch.trim();
        sticky.split(',').any(|v| v.trim() == current_branch)
    };

    if current_branch_is_sticky {
        io::stdout().write(STICKY_NO_JUMP.as_bytes()).unwrap();
        return Ok(ExitCode::SUCCESS);
    }

    shell::run(git!("checkout", goal));
}

fn main() -> std::process::ExitCode {
    // To keep things simple, we only run the complicated logic when there is
    // exactly 1 CLI argument (that is not the binary itself).
    let args: Vec<_> = std::env::args_os().skip(1).collect();
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
    match try_main(goal.trim()) {
        Ok(v) => v.exit(),
        Err(()) => return std::process::ExitCode::FAILURE,
    }
}
