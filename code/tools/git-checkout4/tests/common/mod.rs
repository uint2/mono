pub use git_checkout4::{AppConfig, AppCtx};

pub use {
    shell::{CommandExt, OutputExt},
    types::Test,
};

pub(crate) use std::fs;
pub(crate) use std::path::{Path, PathBuf};

/// Creates a `std::process::Command` instance with "git" as `argv[0]`, and
/// fills in the CLI arguments.
macro_rules! git {
    ($($arg:expr),* $(,)?) => {{
        let mut x = std::process::Command::new("git");
        $(x.arg($arg);)*
        x
    }};
}

/// Creates a `std::process::Command` instance with arguments.
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
    interactive: true,
};
