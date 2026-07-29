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

use std::process::ExitCode;

fn try_main() -> Result<ExitCode, ()> {
    let Ok(worktrees) = git!("worktrees", "list", "--porcelain").output() else {
        eprintln!("Failed to get git worktrees");
        return Err(());
    };

    Ok(ExitCode::from(64))
}

fn main() -> ExitCode {
    match try_main() {
        Ok(v) => v,
        Err(()) => ExitCode::FAILURE,
    }
}
