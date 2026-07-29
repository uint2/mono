macro_rules! git {
    ($($arg:expr),*) => {{
        let mut cmd = std::process::Command::new("git");
        cmd$(.arg($arg))* ;
        cmd
    }};
}

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod tests;

use core::str;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{Command, ExitCode, ExitStatus};

#[derive(Debug)]
struct GitWorktree<'a> {
    path: &'a Path,
    head: &'a str,
    /// The full ref of the branch. Parsed from one of
    /// * "branch refs/heads/main",
    /// * "detached".
    ///
    /// The other cases are just not considered. We really only care when the
    /// branch ref actually exists.
    ref_name: Option<&'a str>,
}

macro_rules! err {
    ($($arg:tt)*) => {{
        Err(eprintln!($($arg)*))
    }};
}

impl<'a> GitWorktree<'a> {
    /// The short branch name, if it exists.
    pub fn canonical_branch(&self) -> Option<&'a str> {
        let Some(ref_name) = self.ref_name else { return None };
        ref_name.strip_prefix("refs/heads/")
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
                    worktrees.push(GitWorktree {
                        path: Path::new(line.trim_start()),
                        head: "",
                        ref_name: None,
                    });
                    state = 1;
                }
                1 => {
                    let Some(line) = line.strip_prefix("HEAD") else {
                        eprintln!(
                            "The second line of each worktree must start with \"HEAD\"."
                        );
                        return Err(());
                    };
                    worktrees.last_mut().unwrap().head = line.trim_start();
                    state = 2;
                }
                2 if line.is_empty() => state = 0,
                2 => {
                    if let Some(line) = line.strip_prefix("branch") {
                        worktrees.last_mut().unwrap().ref_name = Some(line.trim_start());
                    }
                }
                _ => return Err(eprintln!("Invalid git worktree parser state.")),
            }
        }
        Ok(worktrees)
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
        return Ok(status_to_code(worktrees.status));
    }
    let Ok(stdout) = str::from_utf8(&worktrees.stdout) else {
        return err!("Unable to decode `git worktree` stdout as utf-8.");
    };
    let worktrees = GitWorktree::parse(stdout)?;

    println!("{:?}", worktrees);

    Ok(ExitCode::from(64))
}

fn status_to_code(status: ExitStatus) -> ExitCode {
    match status.code() {
        Some(v) => ExitCode::from(v as u8),
        None => ExitCode::FAILURE,
    }
}
// return worktrees.status.code().map(|v| ExitCode::from(v as u8)).ok_or(());

fn main() -> ExitCode {
    // To keep things simple, we only run the complicated logic when there is
    // exactly 1 CLI argument (that is not the binary itself).
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let 1 = args.len() else {
        let mut cmd = Command::new("git");
        cmd.arg("checkout");
        cmd.args(args);
        return if cfg!(unix) {
            use std::os::unix::process::CommandExt;
            let err = cmd.exec();
            eprintln!("Failed execvp call: {err}");
            ExitCode::FAILURE
        } else {
            status_to_code(cmd.spawn().unwrap().wait().unwrap())
        };
    };
    let Some(goal) = args[0].to_str() else {
        eprintln!("Failed to decode target.");
        return ExitCode::FAILURE;
    };
    match try_main(goal) {
        Ok(v) => v,
        Err(()) => ExitCode::FAILURE,
    }
}
