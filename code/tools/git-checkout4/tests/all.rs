#[macro_use]
mod common;

use common::*;
use git_checkout4::{App, AppConfig, Branch, Outcome, Worktree};

use std::fs;
use std::path::Path;
use std::process::Stdio;

use regex::Regex;

const CHECKOUT: &str = "checkout4";

macro_rules! function {
    () => {{
        fn f() {}
        type_name_of(f).strip_suffix("::f").unwrap()
    }};
}

macro_rules! re {
    ($regex:expr) => {
        Regex::new($regex).unwrap()
    };
}

fn type_name_of<T>(_: T) -> &'static str {
    core::any::type_name::<T>()
}

fn git_branch<P: AsRef<Path>>(dir: P) -> String {
    at(dir, || git!("branch", "--show-current").get_stdout())
}

const CONFIG: AppConfig = AppConfig {
    enable_logging: true,
    log_level: log::LevelFilter::Trace,
    interactive: false,
};

macro_rules! assert_regex {
    ($text:expr, $regex:expr $(,)?) => {{
        let text: &str = &$text;
        let r = Regex::new($regex).unwrap();
        match r.find(text) {
            Some(m) if m.len() == ($text).len() => {}
            _ => panic!(
                "Regex mismatch:\nregex: \x1b[36m[\x1b[m{}\x1b[36m]\x1b[m\ntext:  \x1b[36m[\x1b[m{}\x1b[36m]\x1b[m",
                $regex, $text
            ),
        }
    }};
}

#[test]
fn no_worktree() {
    let t = Test::new(function!());
    t.sh2("", &["git", "init", "-b", "main"]);
    t.sh2("", &["git", "commit", "--allow-empty", "-m", "Initial commit"]);

    let output = t.sh("", || git!("worktree", "list", "--porcelain").get_stdout());

    assert_regex!(
        output.as_str().trim(),
        "\
worktree [A-Za-z0-9/:_-]+
HEAD [a-f0-9]{40}
branch refs/heads/main"
    );
}

#[test]
fn basic_worktree() {
    let t = Test::new(function!());
    t.sh2("", &["git", "init", "--bare", ".git"]);
    t.sh2("", &["git", "worktree", "add", "main", "--orphan"]);
    t.sh2("main", &["git", "commit", "--allow-empty", "-m", "Initial commit"]);

    let output = t.sh("", || git!("worktree", "list", "--porcelain").get_stdout());

    assert_regex!(
        output.as_str(),
        "\
worktree [A-Za-z0-9/:_-]+
bare

worktree [A-Za-z0-9/:_-]+/main
HEAD [a-f0-9]{40}
branch refs/heads/main"
    );
}

#[test]
fn checkout_an_owned_branch() {
    let t = Test::new(function!());
    t.sh2("", &["git", "init", "-b", "main"]);
    t.sh2("", &["git", "commit", "--allow-empty", "-m", "Initial commit"]);
    t.sh2("", &["git", "checkout", "-b", "dev"]);

    let dir = t.dir().to_str().unwrap();
    t.sh2("", &["git", "config", "set", "checkout4.dev.worktree", dir]);
    t.sh2("", &["git", "config", "set", "checkout4.main.worktree", dir]);

    let app = t.sh("", || App::init(CONFIG)).unwrap();

    assert_eq!(git_branch(&t), "dev");
    let outcome = t.sh("", || app.execute("main"));
    assert_eq!(outcome, Outcome::Bypass("main"));
}

#[test]
fn checkout_an_unowned_branch() {
    let t = Test::new(function!());
    t.sh2("", &["git", "init", "-b", "main", "--bare", ".git"]);
    t.sh2("", &["git", "worktree", "add", "--orphan", "main"]);
    t.sh2("main", &["git", "commit", "--allow-empty", "-m", "Initial commit"]);
    t.sh2("", &["git", "worktree", "add", "dev"]);

    let app = t.sh("", || App::init(CONFIG)).unwrap();

    assert_eq!(git_branch(&t.join("dev")), "dev");

    let outcome = t.sh("", || app.execute("main"));
    let b_main = Branch::new("main");
    assert_eq!(
        outcome,
        Outcome::JumpAndCheckout {
            worktree: app.get_worktree(b_main).unwrap(),
            branch: b_main,
            relpath: Path::new("")
        }
    );
}

/// When the current relative path in the repo is availble, jump to that.
#[test]
fn successful_dir_match_jump() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", "main", "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", "main").snw();
        some_commit("main/src/main/java/com/example");
        git!("worktree", "add", "dev").snw();
    });

    let app = t.sh("dev/src/main/java", || App::init(CONFIG)).unwrap();
    let outcome = t.sh("dev/src/main/java", || app.execute("main"));
    let b_main = Branch::new("main");
    assert_eq!(
        outcome,
        Outcome::JumpAndCheckout {
            worktree: app.get_worktree(b_main).unwrap(),
            branch: b_main,
            relpath: Path::new("src/main/java")
        }
    );
}

/// If the relative path from the worktree root is not available, retreat back
/// until it exists.
#[test]
fn nearest_dir_match_jump() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", "main", "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", "main").snw();
        some_commit("main/src/main/java");
        git!("worktree", "add", "dev").snw();
        some_commit("dev/src/main/java/com/example");
    });

    let app = t.sh("dev/src/main/java/com/example", || App::init(CONFIG)).unwrap();
    let outcome = t.sh("dev/src/main/java/com/example", || app.execute("main"));
    let b_main = Branch::new("main");
    assert_eq!(
        outcome,
        Outcome::JumpAndCheckout {
            worktree: app.get_worktree(b_main).unwrap(),
            branch: b_main,
            relpath: Path::new("src/main/java")
        }
    );
}

/// Jump from the lift lobby.
#[test]
fn lift_lobby() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "--bare", ".git").snw();
        git!("worktree", "add", "main", "--orphan").snw();
    });

    let app = t.sh("", || App::init(CONFIG)).unwrap();
    let outcome = t.sh("", || app.execute("main"));
    let worktree = app.get_worktree(Branch::new("main")).unwrap();
    assert_eq!(outcome, Outcome::Jump { worktree, relpath: Path::new("") });
}

/// Can handle bare repos.
#[test]
fn bare_repos() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", "main", "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", "main").snw();
        some_commit("main");
        git!("worktree", "add", "dev").snw();
    });

    let app = t.sh("", || App::init(CONFIG)).unwrap();
    let outcome = t.sh("", || app.execute("dev"));
    let b_dev = Branch::new("dev");
    assert_eq!(
        outcome,
        Outcome::JumpAndCheckout {
            worktree: app.get_worktree(b_dev).unwrap(),
            branch: b_dev,
            relpath: Path::new("")
        }
    );
}

/// Jump from to worktree using branch name.
#[test]
fn jump_with_branch() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", "main", "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", "main").snw();
        some_commit("main");
        git!("worktree", "add", "dev").snw();
        git!("worktree", "add", "-b", "benjamin", "diana").snw();
    });

    // Register the "benjamin" branch.
    let app = t.sh("main", || App::init(CONFIG)).unwrap();
    let b_benjamin = Branch::new("benjamin");
    let w_diana = app.get_worktree(b_benjamin).unwrap();
    t.sh("main", || app.git_config().set(b_benjamin, w_diana));

    // Re-read the updated config from filesystem.
    let app = t.sh("main", || App::init(CONFIG)).unwrap();

    let outcome = t.sh("main", || app.execute("benjamin"));
    assert_eq!(
        outcome,
        Outcome::JumpAndCheckout {
            worktree: w_diana,
            branch: b_benjamin,
            relpath: Path::new("")
        }
    );
}

/// Jump from to worktree using directory name.
#[test]
fn jump_with_directory() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", "main", "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", "main").snw();
        some_commit("main");
        git!("worktree", "add", "dev").snw();
        git!("worktree", "add", "-b", "benjamin", "diana").snw();
    });

    // Register the "benjamin" branch.
    let app = t.sh("main", || App::init(CONFIG)).unwrap();
    let b_benjamin = Branch::new("benjamin");
    let w_diana = app.get_worktree(b_benjamin).unwrap();
    t.sh("main", || app.git_config().set(b_benjamin, w_diana));

    // Re-read the updated config from filesystem.
    let app = t.sh("main", || App::init(CONFIG)).unwrap();

    let outcome = t.sh("main", || app.execute("diana"));
    assert_eq!(outcome, Outcome::Jump { worktree: w_diana, relpath: Path::new("") });
}

/// Checkout a branch that matches the current directory.
/// On a directory that is called "main", but is on branch "dev". Then when we
/// checkout "main" again, the git branch should now be "main".
#[test]
fn checkout_branch_matches_directory() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", "main", "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", "main").snw();
        some_commit("main");
    });

    // Register the "main" branch.
    let app = t.sh("main", || App::init(CONFIG)).unwrap();
    t.sh("main", || app.execute(""));

    // Re-read the updated config from filesystem.
    let app = t.sh("main", || App::init(CONFIG)).unwrap();

    // Set branch to "dev".
    t.sh("main", || git!("checkout", "-b", "dev").snw());
    assert_eq!(t.branch_at("main"), "dev");

    let outcome = t.sh("main", || app.execute("main"));
    assert_eq!(outcome, Outcome::Bypass("main"));
}

/// Checkout a branch from a detached head state.
#[test]
fn checkout_from_detached() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", "main").snw();
        some_commit(".");
    });

    // Register the "main" branch.
    let app = t.sh("", || App::init(CONFIG)).unwrap();
    t.sh("", || app.execute(""));

    let sha = t.sh("", || git!("rev-parse", "HEAD").get_stdout());
    t.sh2("", &["git", "checkout", sha.as_str().trim()]);

    let app = t.sh("", || App::init(CONFIG)).unwrap();
    let outcome = t.sh("", || app.execute("main"));
    assert_eq!(outcome, Outcome::Bypass("main"));
}

// /// `git-checkout3` should return the same exit code as `git checkout` in an
// /// empty repository.
// #[test]
// #[ignore]
// fn empty_directory() {
//     let t = Test::new(function!());
//
//     let (lhs, rhs) = t.sh("", || {
//         let lhs = git!(CHECKOUT, "zeno").get();
//         let rhs = git!("checkout", "zeno").get();
//         (lhs, rhs)
//     });
//     assert_eq!(lhs.status, rhs.status);
// }
//
// /// `git-checkout3` should return the same exit code as `git checkout` when a
// /// branch doesn't exist.
// #[test]
// #[ignore]
// fn branch_not_exists() {
//     let t = Test::new(function!());
//     t.sh("", || {
//         git!("init", "-b", "main", "--bare", ".git").snw();
//         // git!("config", STICKY_CONFIG_KEY, "hello,world,dev,hello,world").snw();
//         git!("worktree", "add", "--orphan", "main").snw();
//         some_commit("main");
//     });
//     let (lhs, rhs) = at(t.join("main"), || {
//         let lhs = git!(CHECKOUT, "zeno").get();
//         let rhs = git!("checkout", "zeno").get();
//         (lhs, rhs)
//     });
//     assert_eq!(lhs.status, rhs.status);
// }
