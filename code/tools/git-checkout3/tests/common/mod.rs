macro_rules! git {
    ($($arg:expr),* $(,)?) => { std::process::Command::new("git")$(.arg($arg))* };
}

macro_rules! sh {
    ($first:expr) => { std::process::Command::new($first) };
    ($first:expr, $($arg:expr),* $(,)?) => { std::process::Command::new($first)$(.arg($arg))* };
}

mod shell;
mod types;

pub use {
    shell::{CommandExt, OutputExt},
    types::{Test, at},
};

use std::fs;
use std::path::Path;

/// Creates a random commit by making some file at `dir`.
pub fn some_commit<P: AsRef<Path>>(dir: P) {
    let dir = dir.as_ref();
    let fingerprint = std::time::UNIX_EPOCH.elapsed().unwrap();
    let fingerprint = format!("{:?}.txt", fingerprint.as_micros());
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join(&fingerprint);
    fs::write(&file, "hello").unwrap();
    git!("-C", dir, "add", fingerprint).snw();
    git!("-C", dir, "commit", "-m", "boopus gloopus").snw();
}
