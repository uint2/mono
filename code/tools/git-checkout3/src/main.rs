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
use std::process::{Command, Output, Stdio};
use std::time::Duration;
use std::{env, io};

use futures::FutureExt;

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

/// Get the sticky branches in a comma-separated string.
fn get_sticky_branches() -> Result<Output, ()> {
    let output = git!("config", "--get", STICKY_CONFIG_KEY)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .output();
    output.map_err(|_| eprintln!("Failed to execute shell command to get git config."))
}

fn canonical_eq(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

fn try_main(goal: &str) -> Result<ExitCode, ()> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();

    let mut sticky_output: Result<Output, ()> = Err(());
    let mut git_branch_output: Result<Output, ()> = Err(());
    let mut git_worktree_output: Result<Output, ()> = Err(());

    rayon::scope(|scope| {
        scope.spawn(|_| {
            sticky_output =
                git!("config", "--get", STICKY_CONFIG_KEY).output().map_err(|_| {
                    eprintln!("Failed to execute shell command to get git config.")
                });
        });
        scope.spawn(|_| {
            git_branch_output = git!("branch", "--show-current").output().map_err(|_| {
                eprintln!("Failed to execute shell command to get git branch.")
            });
        });
        scope.spawn(|_| {
            git_worktree_output =
                git!("worktree", "list", "--porcelain").output().map_err(|_| {
                    eprintln!("Failed to execute shell command to get git worktrees.")
                });
        });
    });
    let sticky_output = sticky_output?;
    let git_branch_output = git_branch_output?;
    let git_worktree_output = git_worktree_output?;

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

    // if is_branch_sticky {
    //     io::stderr().write(STICKY_NO_JUMP.as_bytes()).unwrap();
    //     return Ok(ExitCode::SUCCESS);
    // }

    let worktrees = GitWorktree::parse(worktrees)?;

    // 1. Prioritize the directory match. Look through the worktrees and match
    // the directory of each worktree checked out against `goal`.
    for worktree in &worktrees {
        if worktree.directory() == goal {
            let cwd = getcwd()?;
            if canonical_eq(Path::new(worktree.abs_path), cwd.as_path()) {
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
            return worktree.accept_and_resolve(&getcwd()?, &worktrees);
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
