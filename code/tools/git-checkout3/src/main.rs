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
use std::process::ExitCode;

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
    branch: Option<&'a str>,
}

impl<'a> GitWorktree<'a> {
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
                        branch: None,
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
                        worktrees.last_mut().unwrap().branch = Some(line.trim_start());
                    }
                }
                _ => return Err(eprintln!("Invalid git worktree parser state.")),
            }
        }
        Ok(worktrees)
    }
}

fn try_main() -> Result<ExitCode, ()> {
    let Ok(worktrees) = git!("worktree", "list", "--porcelain").output() else {
        eprintln!("Failed to execute shell command to get git worktrees.");
        return Err(());
    };
    let Ok(stderr) = str::from_utf8(&worktrees.stderr) else {
        eprintln!("Unable to decode `git worktree` stderr as utf-8.");
        return Err(());
    };
    if stderr.starts_with("fatal: not a git repository") {
        // fatal: not a git repository (or any parent up to mount point)
        // fatal: not a git repository (or any of the parent directories)
        io::stderr().write(&worktrees.stderr).unwrap();
        return worktrees.status.code().map(|v| ExitCode::from(v as u8)).ok_or(());
    }
    let Ok(stdout) = str::from_utf8(&worktrees.stdout) else {
        eprintln!("Unable to decode `git worktree` stdout as utf-8.");
        return Err(());
    };
    let worktrees = GitWorktree::parse(stdout)?;

    println!("{:?}", worktrees);

    Ok(ExitCode::from(64))
}

fn main() -> ExitCode {
    match try_main() {
        Ok(v) => v,
        Err(()) => ExitCode::FAILURE,
    }
}
