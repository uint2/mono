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
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io};

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
        enum State {
            /// Looking for "worktree".
            LFWorktree,
            /// Looking for "HEAD". Might see "bare".
            LFHead,
            /// Looking for "branch", followed by an absolute path.
            /// Might see "detached".
            LFDirectory,
        }
        let mut state = State::LFWorktree;
        for line in text.lines() {
            match state {
                State::LFWorktree => {
                    let Some(line) = line.strip_prefix("worktree") else {
                        eprintln!(
                            "The first line of each worktree must start with \"worktree\"."
                        );
                        return Err(());
                    };
                    let abs_path = line.trim_start();
                    worktrees.push(GitWorktree { abs_path, head: Err(()), branch: None });
                    state = State::LFHead;
                }
                State::LFHead => {
                    if line.trim() == "bare" {
                        state = State::LFDirectory;
                        continue;
                    }
                    let Some(line) = line.strip_prefix("HEAD") else {
                        eprintln!(
                            "The second line of each worktree must start with \"HEAD\"."
                        );
                        return Err(());
                    };
                    worktrees.last_mut().unwrap().head = Ok(line.trim_start());
                    state = State::LFDirectory;
                }
                State::LFDirectory if line.is_empty() => state = State::LFWorktree,
                State::LFDirectory => {
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

    fn find_closest_parent<'t>(abs_cwd: &Path, trees: &'t [Self]) -> Option<&'t Self> {
        trees
            .iter()
            .filter(|t| t.branch.is_some())
            .filter(|t| abs_cwd.starts_with(t.abs_path))
            .max_by(|a, b| a.abs_path.len().cmp(&b.abs_path.len()))
    }

    pub fn accept_and_resolve(&self, cwd: &Path, trees: &[Self]) -> Result<ExitCode, ()> {
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

#[inline]
fn getcwd() -> Result<PathBuf, ()> {
    env::current_dir().map_err(|_| eprintln!("Unable to get current working directory."))
}

fn is_branch_sticky(branch: &str) -> bool {
    let Ok(sticky) = git!("config", "--get", STICKY_CONFIG_KEY).output() else {
        eprintln!("Failed to execute shell command to get git config.");
        return false;
    };
    let Ok(sticky) = str::from_utf8(&sticky.stdout) else {
        eprintln!("Unable to decode sticky config as utf-8.");
        return false;
    };
    let branch = branch.trim();
    sticky.split(',').any(|v| v.trim() == branch)
}

fn try_main(goal: &str) -> Result<ExitCode, ()> {
    let Ok(current_branch) = git!("branch", "--show-current").output() else {
        return err!("Failed to execute shell command to get git branch.");
    };
    let Ok(current_branch) = str::from_utf8(&current_branch.stdout) else {
        return err!("Unable to decode `git branch` as utf-8.");
    };

    if is_branch_sticky(current_branch) {
        io::stderr().write(STICKY_NO_JUMP.as_bytes()).unwrap();
        return Ok(ExitCode::SUCCESS);
    }

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
            // panic!("{:?}", worktree);
            let cwd = getcwd()?;
            let worktree_dir = Path::new(worktree.abs_path).canonicalize().ok();
            if worktree_dir == cwd.canonicalize().ok() {
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
            return worktree.accept_and_resolve(&getcwd()?, &worktrees);
        }
    }

    shell::run(git!("checkout", goal));
}

fn main() -> std::process::ExitCode {
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
    match try_main(goal.trim()) {
        Ok(v) => v.exit(),
        Err(()) => return std::process::ExitCode::FAILURE,
    }
}
