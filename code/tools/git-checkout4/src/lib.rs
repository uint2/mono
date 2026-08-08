macro_rules! git {
    ($($arg:expr),*) => {{
        let mut cmd = std::process::Command::new("git");
        cmd$(.arg($arg))* ;
        cmd
    }};
}

mod app;
mod cli;
mod config;
mod data;
mod git;
mod prelude;
mod shell;

use prelude::*;

pub use {
    app::{App, Outcome},
    data::Branch,
};

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

fn main() -> std::process::ExitCode {
    // log::init(Some(log::LevelFilter::Trace));

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
    pool.install(|| {
        let app = App::init().unwrap();
        app.execute(goal.trim());
    });
    std::process::ExitCode::SUCCESS
    // Ok(v) => v.exit(),
    // Err(()) => return std::process::ExitCode::FAILURE,
    // }
}
