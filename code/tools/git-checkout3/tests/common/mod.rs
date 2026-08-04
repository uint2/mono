macro_rules! git {
    ($($arg:expr),* $(,)?) => { std::process::Command::new("git")$(.arg($arg))* };
}

macro_rules! sh {
    ($first:expr) => { std::process::Command::new($first) };
    ($first:expr, $($arg:expr),* $(,)?) => { std::process::Command::new($first)$(.arg($arg))* };
}

macro_rules! cd {
    ($dir:expr) => {
        std::env::set_current_dir(&dir).unwrap()
    };
}

mod shell;
mod types;

pub use {
    shell::{CommandExt, OutputExt, commit_file},
    types::Test,
};

use std::path::{Path, PathBuf};
use std::{env, fs};

/// Set up the test directory to a git repo of the state below, and returns the
/// path to the `repo/` dir here:
/// └── repo
///    ├── .git                  bare repo
///    ├── B1                    worktree
///    │  ├── src
///    │  │   └── main
///    │  │       └── java
///    │  │           └── Main.java
///    │  └── README.md
///    ├── B2
///    │  ├── src
///    │  │   └── main
///    │  │       └── java
///    │  │           └── Main.java
///    │  └── README.md
///    └── D3
///       ├── src
///       │   └── main
///       │       └── java
///       │           └── Main.java
///       └── README.md
pub fn setup(name: &'static str) -> (Test, PathBuf) {
    let mut t = Test::new(name);

    // The place where we initialize the git history. Fill it out with events.
    let d_base = t.as_path().join("base");
    // The place where we'll make into a bare repo with the history from `base`.
    let d_repo = t.as_path().join("repo");

    fs::create_dir_all(&d_base).unwrap();
    fs::create_dir_all(&d_repo).unwrap();

    env::set_current_dir(&d_base).unwrap();
    git!("init", "--initial-branch=main").snw();
    git!("config", "--local", "user.email", "git@git.git").snw();
    git!("config", "--local", "user.name", "git").snw();

    eprintln!("=== Initialized a git repo ===");

    commit_file(&mut t, "README.md");

    let mut commits = vec![];

    let dir = Path::new("src/main/java");
    fs::create_dir_all(dir).unwrap();
    let main_java = dir.join("Main.java");

    for _ in 0..6 {
        commit_file(&mut t, &main_java);
        commit_file(&mut t, "README.md");
        let c = git!("rev-parse", "--verify", "HEAD").get_stdout();
        commits.push(c);
    }

    {
        fn ok(sha: &str) -> bool {
            sha.is_ascii() && sha.len() == 40
        }
        for (i, commit) in commits.iter().enumerate() {
            let i = i + 1;
            assert!(ok(&commit), "Commit #{i} is a strange one: {commit}");
        }
    }

    let branches = ["B1", "B2", "B3", "B4"];

    for (branch, commit) in branches.into_iter().zip(&commits) {
        git!("checkout", "-b", branch).snw();
        git!("reset", "--hard", commit).snw();
        assert_eq!(git!("branch", "--show-current").get_stdout(), branch);
    }

    git!("checkout", "main").snw();
    assert_eq!(git!("branch", "--show-current").get_stdout(), "main");

    // Convert to bare repo.
    git!("-C", d_base.join(".git"), "config", "--bool", "core.bare", "true");
    fs::rename(d_base.join(".git"), &d_repo).unwrap();
    fs::remove_dir_all(d_base).unwrap(); // Intentionally drop `d_base`
    env::set_current_dir(&d_repo).unwrap();

    // Create the worktrees.
    git!("worktree", "add", "B1").snw();
    git!("worktree", "add", "B2").snw();
    git!("worktree", "add", "D3").snw();
    git!("-C", d_repo.join("D3"), "checkout", "B3").snw();
    git!("branch", "-D", "D3").snw();

    (t, d_repo)
}

/// Creates a random commit by making some file at `dir`.
pub fn some_commit<P: AsRef<Path>>(dir: P) {
    let dir = dir.as_ref();
    let fingerprint = std::time::UNIX_EPOCH.elapsed().unwrap();
    let fingerprint = format!("{:?}.txt", fingerprint.as_micros());
    let file = dir.join(&fingerprint);
    fs::write(&file, "hello").unwrap();
    git!("-C", dir, "add", fingerprint).snw();
    git!("-C", dir, "commit", "-m", "boopus gloopus").snw();
}

pub fn cd<P: AsRef<Path>>(dir: P) {
    env::set_current_dir(dir).unwrap()
}
