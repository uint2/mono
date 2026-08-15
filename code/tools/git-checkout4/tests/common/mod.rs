pub use git_checkout4::{App, AppConfig, AppCtx, Branch, Outcome, Worktree};

#[allow(unused)]
pub use {
    shell::{CommandExt, OutputExt},
    types::{Test, at},
};

use std::fs;
pub use std::path::Path;

macro_rules! git {
    ($($arg:expr),* $(,)?) => { std::process::Command::new("git")$(.arg($arg))* };
}

macro_rules! branch {
    ($value:expr) => {
        Branch::new($value)
    };
}

#[allow(unused)]
macro_rules! sh {
    ($first:expr) => { std::process::Command::new($first) };
    ($first:expr, $($arg:expr),* $(,)?) => { std::process::Command::new($first)$(.arg($arg))* };
}

macro_rules! assert_regex {
    ($text:expr, $regex:expr $(,)?) => {{
        let text: &str = &$text;
        let r = regex::Regex::new($regex).unwrap();
        match r.find(text) {
            Some(m) if m.len() == ($text).len() => {}
            _ => panic!(
                "Regex mismatch:\nregex: \x1b[36m[\x1b[m{}\x1b[36m]\x1b[m\ntext:  \x1b[36m[\x1b[m{}\x1b[36m]\x1b[m",
                $regex, $text
            ),
        }
    }};
}

mod shell;
mod types;

pub const CONFIG: AppConfig = AppConfig {
    enable_logging: false,
    log_level: log::LevelFilter::Trace,
    interactive: false,
};

/// Creates a random commit by making some file at `dir`.
pub fn some_commit<P: AsRef<Path>>(dir: P) {
    let dir = dir.as_ref();
    let fingerprint = std::time::UNIX_EPOCH.elapsed().unwrap();
    let fingerprint = format!("{:?}.txt", fingerprint.as_micros());
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join(&fingerprint);
    fs::write(&file, "hello").unwrap();
    git!("-C", dir, "config", "user.name", "git").snw();
    git!("-C", dir, "config", "user.email", "git@git.git").snw();
    git!("-C", dir, "add", fingerprint).snw();
    git!("-C", dir, "commit", "-m", "boopus gloopus").snw();
}

pub fn git_branch<P: AsRef<Path>>(dir: P) -> String {
    at(dir, || git!("branch", "--show-current").get_stdout())
}

macro_rules! function {
    () => {{
        fn f() {}
        type_name_of(f).strip_suffix("::f").unwrap()
    }};
}

pub fn type_name_of<T>(_: T) -> &'static str {
    core::any::type_name::<T>()
}
