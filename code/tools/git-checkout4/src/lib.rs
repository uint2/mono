macro_rules! git {
    ($($arg:expr),*) => {{
        let mut cmd = std::process::Command::new("git");
        cmd$(.arg($arg))* ;
        cmd
    }};
}

mod app;
mod data;
mod git;
mod git_config;
mod prelude;
mod shell;

use prelude::*;

pub use {
    app::{App, AppConfig, Outcome},
    data::{Branch, Worktree},
};

const RUNTIME_CONFIG: AppConfig = AppConfig {
    enable_logging: false,
    log_level: log::LevelFilter::Trace,
    interactive: true,
};

pub fn main() -> std::process::ExitCode {
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
    enum Action {
        Bypass,
        ExitCode(u8),
    }
    let action = pool.install(|| {
        let app = App::init(RUNTIME_CONFIG).unwrap();
        match app.execute(goal.trim()) {
            Outcome::Jump { worktree, relpath } => {
                let path = worktree.as_path().join(relpath);
                println!("{}", path.display());
                Action::ExitCode(61)
            }
            Outcome::JumpAndCheckout { worktree, branch, relpath } => {
                let path = worktree.as_path().join(relpath);
                println!("{}:{}", path.display(), branch.as_str());
                Action::ExitCode(62)
            }
            Outcome::Bypass => Action::Bypass,
        }
    });
    match action {
        Action::Bypass => shell::run(git!("checkout", goal)),
        Action::ExitCode(code) => std::process::ExitCode::from(code),
    }
}
