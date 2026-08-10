macro_rules! git {
    ($($arg:expr),*) => {{
        let mut cmd = std::process::Command::new("git");
        cmd$(.arg($arg))* ;
        cmd
    }};
}

mod app;
mod context;
mod data;
mod git_config;
mod prelude;
mod shell;

use prelude::*;

pub use {
    app::{App, Outcome},
    context::AppCtx,
    data::{Branch, Worktree},
    prelude::AppConfig,
};

const RUNTIME_CONFIG: AppConfig = AppConfig {
    enable_logging: false,
    log_level: log::LevelFilter::Trace,
    interactive: true,
};

fn full_bypass(args: &[std::ffi::OsString]) -> ! {
    let mut cmd = Command::new("git");
    cmd.arg("checkout");
    cmd.args(args);
    shell::run(cmd)
}

pub fn main() -> ExitCode {
    // To keep things simple, we only run the complicated logic when there is
    // exactly 1 CLI argument (that is not the binary itself).
    let args: Vec<_> = env::args_os().skip(1).collect();
    let 1 = args.len() else { full_bypass(&args) };

    let Some(goal) = args[0].to_str().map(str::trim) else {
        eprintln!("Failed to decode target.");
        return ExitCode::FAILURE;
    };

    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    let ctx = pool.install(|| AppCtx::init(RUNTIME_CONFIG)).unwrap();
    match App::new(&ctx).execute(goal) {
        Outcome::Jump { worktree, relpath } => {
            let path = worktree.as_path().join(relpath);
            eprintln!("[\x1b[36mgco\x1b[m] jump to worktree");
            let mut stdout = io::stdout().lock();
            stdout.write(path.as_os_str().as_encoded_bytes()).unwrap();
            stdout.write(b"\n").unwrap();
            std::process::exit(61)
        }
        Outcome::JumpAndCheckout { worktree, branch, relpath } => {
            let path = worktree.as_path().join(relpath);
            eprintln!(
                "[\x1b[36mgco\x1b[m] jump to worktree, and checkout \x1b[33m{branch}\x1b[m."
            );
            let mut stdout = io::stdout().lock();
            stdout.write(path.as_os_str().as_encoded_bytes()).unwrap();
            stdout.write(b":").unwrap();
            stdout.write(branch.as_str().as_bytes()).unwrap();
            stdout.write(b"\n").unwrap();
            std::process::exit(62)
        }
        Outcome::Bypass => {
            eprintln!("[\x1b[36mgco\x1b[m] bypass.");
            shell::run(git!("checkout", goal))
        }
    }
}
