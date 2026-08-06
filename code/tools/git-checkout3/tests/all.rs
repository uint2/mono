#[macro_use]
mod common;

#[path = "../src/consts.rs"]
mod consts;

use consts::{STICKY_CONFIG_KEY, STICKY_NO_JUMP};

use common::*;

use std::fs;
use std::path::Path;

const CHECKOUT: &str = "checkout3";
const MAIN: &str = "main";

macro_rules! function {
    () => {{
        fn f() {}
        type_name_of(f).strip_suffix("::f").unwrap()
    }};
}

fn type_name_of<T>(_: T) -> &'static str {
    core::any::type_name::<T>()
}

fn git_branch<P: AsRef<Path>>(dir: P) -> String {
    at(dir, || git!("branch", "--show-current").get_stdout())
}

/// Jump from the lift lobby.
#[test]
fn lift_lobby() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "--bare", ".git").snw();
        git!("worktree", "add", MAIN, "--orphan").snw();
    });

    let output = t.sh("", || git!(CHECKOUT, MAIN).get());
    assert_eq!(output.stdout, t.join(MAIN), "Mismatch: {output:?}");
    assert_eq!(output.status.code(), Some(64));

    let output = at(t.join(".git"), || git!(CHECKOUT, MAIN).get());
    assert_eq!(output.stdout, t.join(MAIN), "Mismatch: {output:?}");
    assert_eq!(output.status.code(), Some(64));
}

/// Can handle bare repos.
#[test]
fn bare_repos() {
    let t = Test::new(function!());
    cd(&t);
    t.sh("", || {
        git!("init", "-b", MAIN, "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", MAIN).snw();
        some_commit(MAIN);
        git!("worktree", "add", "dev").snw();
    });

    let output = at(t.join(MAIN), || git!(CHECKOUT, "dev").get());
    assert_eq!(output.stdout, t.join("dev"), "Mismatch: {output:?}");
    assert_eq!(output.status.code(), Some(64));
}

/// Jump from to worktree using branch name.
#[test]
fn jump_with_branch() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", MAIN, "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", MAIN).snw();
        some_commit(MAIN);
        git!("worktree", "add", "dev").snw();
        git!("worktree", "add", "-b", "benjamin", "diana").snw();
    });

    let output = at(t.join(MAIN), || git!(CHECKOUT, "benjamin").get());
    assert_eq!(output.stdout, t.join("diana"), "Mismatch: {output:?}");
    assert_eq!(output.status.code(), Some(64));
}

/// Jump from to worktree using directory name.
#[test]
fn jump_with_directory() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", MAIN, "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", MAIN).snw();
        some_commit(MAIN);
        git!("worktree", "add", "dev").snw();
        git!("worktree", "add", "-b", "benjamin", "diana").snw();
        git!("worktree", "add", "-b", "benjamin", "diana").snw();
    });

    let output = at(t.join(MAIN), || git!(CHECKOUT, "diana").get());
    assert_eq!(output.stdout, t.join("diana"), "Mismatch: {}", output.stdout);
    assert_eq!(output.status.code(), Some(64));
}

/// Checkout a branch.
#[test]
fn checkout_branch() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", MAIN, "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", MAIN).snw();
        some_commit(MAIN);
        git!("worktree", "add", "-b", "benjamin", "diana").snw();
        git!("-C", "diana", "checkout", "-b", "briana").snw();
    });

    let output = at(t.join(MAIN), || git!(CHECKOUT, "benjamin").get());

    assert_eq!(output.stdout, "", "Mismatch: {output:?}");
    assert!(output.status.success());
    assert_eq!(git_branch(t.join(MAIN)), "benjamin");
}

/// Checkout a branch that matches the current directory.
/// On a directory that is called "main", but is on branch "dev". Then when we
/// checkout "main" again, the git branch should now be "main".
#[test]
fn checkout_branch_matches_directory() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", MAIN, "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", MAIN).snw();
        some_commit(MAIN);
        git!("-C", MAIN, "checkout", "-b", "dev").snw();
    });

    let output = at(t.join(MAIN), || git!(CHECKOUT, MAIN).get());

    assert_eq!(output.stdout, "", "Mismatch: {output:?}");
    assert!(output.status.success());
    assert_eq!(git_branch(t.join(MAIN)), "main");
}

/// Checkout a sticky branch. This should result in full bypass behaviour.
#[test]
fn t6() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", MAIN, "--bare", ".git").snw();
        git!("config", STICKY_CONFIG_KEY, "hello,world,dev,hello,world").snw();
        git!("worktree", "add", "--orphan", MAIN).snw();
        some_commit(MAIN);
        git!("-C", MAIN, "checkout", "-b", "dev").snw();
        git!("-C", MAIN, "checkout", "-b", "feature").snw();
    });
    assert_eq!(git_branch(t.join(MAIN)), "feature");

    at(t.join(MAIN), || git!(CHECKOUT, "dev").get());
    assert_eq!(git_branch(t.join(MAIN)), "dev");
}

#[test]
fn git_config_sticky() {
    let t = Test::new(function!());
    const CONFIG_VALUE: &str = "hello,world";
    t.sh("", || {
        git!("init", "-b", MAIN, "--bare", ".git").snw();
        git!("config", STICKY_CONFIG_KEY, CONFIG_VALUE).snw();
    });
    let output = t.sh("", || git!("config", "--get", STICKY_CONFIG_KEY).get());
    assert_eq!(output.stdout, CONFIG_VALUE);
    assert_eq!(output.stderr, "");
    assert!(output.status.success());
}

/// When trying to checkout another branch but currently on a sticky branch, do
/// not jump, and print the help message.
#[test]
fn t7() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", MAIN, "--bare", ".git").snw();
        git!("config", STICKY_CONFIG_KEY, "hello,world,dev,hello,world").snw();
        git!("worktree", "add", "--orphan", MAIN).snw();
        some_commit(MAIN);
        git!("worktree", "add", "dev").snw();
    });
    assert_eq!(git_branch(t.join("dev")), "dev");

    // Use this way of getting output to be able to compare even the trailing
    // newline.
    let output = at(t.join("dev"), || git!(CHECKOUT, MAIN).output().unwrap());
    let stdout = core::str::from_utf8(&output.stdout).unwrap();
    let stderr = core::str::from_utf8(&output.stderr).unwrap();

    assert_eq!(stdout, "");
    assert_eq!(stderr, STICKY_NO_JUMP);
    assert!(output.status.success());
}

/// When the current relative path in the repo is availble, jump to that.
#[test]
#[ignore]
fn t8() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", MAIN, "--bare", ".git").snw();
        git!("config", STICKY_CONFIG_KEY, "hello,world,dev,hello,world").snw();
        git!("worktree", "add", "--orphan", MAIN).snw();
        some_commit(MAIN);
        git!("worktree", "add", "dev").snw();
    });
    let (_t, root) = setup(function!());
    let cwd = root.join("B1/src/main");
    let output = at(&cwd, || git!(CHECKOUT, "B2").get());
    assert!(output.stdout.ends_with("B2/src/main"));
}

/// If the relative path from the worktree root is not available, retreat back
/// until it exists.
#[test]
#[ignore]
fn t9() {
    const SUBDIRECTORY: &str = "B1/src/main/java/foo/bar/baz";

    let (_t, root) = setup(function!());
    fs::create_dir_all(root.join(SUBDIRECTORY)).unwrap();

    let output = at(root.join(SUBDIRECTORY), || git!(CHECKOUT, "B2").get());
    assert!(output.stdout.ends_with("B2/src/main/java"), "Got: {}", output.stdout);
}

/// `git-checkout3` should return the same exit code as `git checkout` in an
/// empty repository.
#[test]
fn empty_directory() {
    let t = Test::new(function!());
    fs::remove_dir_all(t.dir()).unwrap();
    fs::create_dir(t.dir()).unwrap();

    let (lhs, rhs) = t.sh("", || {
        let lhs = git!(CHECKOUT, "zeno").get();
        let rhs = git!("checkout", "zeno").get();
        (lhs, rhs)
    });
    assert_eq!(lhs.status, rhs.status);
}

/// `git-checkout3` should return the same exit code as `git checkout` when a
/// branch doesn't exist.
#[test]
fn branch_not_exists() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", MAIN, "--bare", ".git").snw();
        git!("config", STICKY_CONFIG_KEY, "hello,world,dev,hello,world").snw();
        git!("worktree", "add", "--orphan", MAIN).snw();
        some_commit(MAIN);
    });
    let (lhs, rhs) = at(t.join(MAIN), || {
        let lhs = git!(CHECKOUT, "zeno").get();
        let rhs = git!("checkout", "zeno").get();
        (lhs, rhs)
    });
    assert_eq!(lhs.status, rhs.status);
}
