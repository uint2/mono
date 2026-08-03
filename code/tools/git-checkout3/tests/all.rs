#[macro_use]
mod common;

#[path = "../src/consts.rs"]
mod consts;

use consts::{STICKY_CONFIG_KEY, STICKY_NO_JUMP};

use common::*;

use std::path::Path;
use std::{env, fs};

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
    git!("-C", dir.as_ref(), "branch", "--show-current").get_stdout()
}

fn git_branches(dir: &Path) -> Vec<String> {
    let output = git!("-C", dir, "branch", "--format=%(refname:short)").get_stdout();
    output.lines().map(|v| v.trim().to_string()).collect()
}

fn cd<P: AsRef<Path>>(dir: P) {
    env::set_current_dir(dir).unwrap()
}

#[test]
fn setup_branches() {
    let (_t, root) = setup(function!());
    assert_eq!(git_branch(root.join("B1")), "B1");
    assert_eq!(git_branch(root.join("B2")), "B2");
    assert_eq!(git_branch(root.join("D3")), "B3");

    let mut branches = git_branches(&root);
    branches.sort();
    assert_eq!(branches, ["B1", "B2", "B3", "B4", "main"]);
}

/// Jump from the lift lobby.
#[test]
fn lift_lobby() {
    let t = Test::new(function!());
    cd(&t);
    git!("init", "-b", MAIN, "--bare", ".git").snw();
    git!("worktree", "add", MAIN, "--orphan").snw();

    let output = git!(CHECKOUT, MAIN).get();
    assert_eq!(output.stdout, t.join(MAIN), "Mismatch: {output:?}");
    assert_eq!(output.status.code(), Some(64));

    cd(".git");
    let output = git!(CHECKOUT, MAIN).get();
    assert_eq!(output.stdout, t.join(MAIN), "Mismatch: {output:?}");
    assert_eq!(output.status.code(), Some(64));
}

/// Can handle bare repos.
#[test]
fn bare_repos() {
    let t = Test::new(function!());
    cd(&t);
    git!("init", "-b", MAIN, "--bare", ".git").snw();
    git!("worktree", "add", "--orphan", MAIN).snw();
    fs::write(t.join(MAIN).join("README"), "boopus").unwrap();
    git!("-C", MAIN, "add", "--all").snw();
    git!("-C", MAIN, "commit", "-m", "gloopus").snw();

    git!("worktree", "add", "dev").snw();
    cd(MAIN);
    let output = git!(CHECKOUT, "dev").get();
    assert_eq!(output.stdout, t.join("dev"), "Mismatch: {output:?}");
    assert_eq!(output.status.code(), Some(64));
}

/// Jump from to worktree using branch name.
#[test]
fn jump_with_branch() {
    let t = Test::new(function!());
    cd(&t);
    git!("init", "-b", MAIN, "--bare", ".git").snw();
    git!("worktree", "add", "--orphan", MAIN).snw();
    fs::write(t.join(MAIN).join("README"), "boopus").unwrap();
    git!("-C", MAIN, "add", "--all").snw();
    git!("-C", MAIN, "commit", "-m", "gloopus").snw();

    git!("worktree", "add", "-b", "benjamin", "diana").snw();

    cd(MAIN);

    let output = git!(CHECKOUT, "benjamin").get();
    assert_eq!(output.stdout, t.join("diana"), "Mismatch: {output:?}");
    assert_eq!(output.status.code(), Some(64));
}

/// Jump from to worktree using directory name.
#[test]
fn jump_with_directory() {
    let t = Test::new(function!());
    cd(&t);
    git!("init", "-b", MAIN, "--bare", ".git").snw();
    git!("worktree", "add", "--orphan", MAIN).snw();
    fs::write(t.join(MAIN).join("README"), "boopus").unwrap();
    git!("-C", MAIN, "add", "--all").snw();
    git!("-C", MAIN, "commit", "-m", "gloopus").snw();

    git!("worktree", "add", "-b", "benjamin", "diana").snw();

    cd(MAIN);

    let output = git!(CHECKOUT, "diana").get();
    assert_eq!(output.stdout, t.join("diana"), "Mismatch: {}", output.stdout);
    assert_eq!(output.status.code(), Some(64));
}

/// Checkout a branch.
#[test]
fn checkout_branch() {
    let t = Test::new(function!());
    cd(&t);
    git!("init", "-b", MAIN, "--bare", ".git").snw();
    git!("worktree", "add", "--orphan", MAIN).snw();
    fs::write(t.join(MAIN).join("README"), "boopus").unwrap();
    git!("-C", MAIN, "add", "--all").snw();
    git!("-C", MAIN, "commit", "-m", "gloopus").snw();

    git!("worktree", "add", "-b", "benjamin", "diana").snw();
    git!("-C", "diana", "checkout", "-b", "briana").snw();

    cd(MAIN);

    let output = git!(CHECKOUT, "benjamin").get();

    assert_eq!(output.stdout, "", "Mismatch: {output:?}");
    assert_eq!(git_branch("."), "benjamin");
}

/// Checkout a branch that matches the current directory.
#[test]
fn checkout_branch_matches_directory() {
    let t = Test::new(function!());
    cd(&t);
    git!("init", "-b", MAIN, "--bare", ".git").snw();
    git!("worktree", "add", "--orphan", MAIN).snw();
    fs::write(t.join(MAIN).join("README"), "boopus").unwrap();
    git!("-C", MAIN, "add", "--all").snw();
    git!("-C", MAIN, "commit", "-m", "gloopus").snw();

    git!("-C", MAIN, "checkout", "-b", "dev").snw();

    cd(MAIN);

    let output = git!(CHECKOUT, "main").get();

    assert_eq!(output.stdout, "", "Mismatch: {output:?}");
    assert_eq!(git_branch("."), "main");
}

#[test]
fn t6() {
    let (_t, root) = setup(function!());
    let cwd = root.join("B1");
    env::set_current_dir(&cwd).unwrap();
    git!("config", STICKY_CONFIG_KEY, "sticky,branches,B4,hello,world").snw();
    git!(CHECKOUT, "B4").snw();
    assert_eq!(git_branch(&cwd), "B4");
}

/// When trying to checkout another branch but currently on a sticky branch, do
/// not jump, and print the help message.
#[test]
fn t7() {
    let (_t, root) = setup(function!());
    let cwd = root.join("B1");
    env::set_current_dir(&cwd).unwrap();
    git!("config", "checkout.sticky", "sticky,branches,B1,hello,world").snw();
    // Use this way of getting output to be able to compare even the trailing
    // newline.
    let output = git!(CHECKOUT, "B4").output().unwrap();
    let stdout = core::str::from_utf8(&output.stdout).unwrap();
    assert_eq!(stdout, STICKY_NO_JUMP);
    assert_eq!(git_branch(&cwd), "B1");
}

/// When the current relative path in the repo is availble, jump to that.
#[test]
fn t8() {
    let (_t, root) = setup(function!());
    let cwd = root.join("B1/src/main");
    env::set_current_dir(&cwd).unwrap();
    let output = git!(CHECKOUT, "B2").get();
    assert!(output.stdout.ends_with("B2/src/main"));
}

/// If the relative path from the worktree root is not available, retreat back
/// until it exists.
#[test]
fn t9() {
    const SUBDIRECTORY: &str = "B1/src/main/java/foo/bar/baz";

    let (_t, root) = setup(function!());
    fs::create_dir_all(root.join(SUBDIRECTORY)).unwrap();
    env::set_current_dir(root.join(SUBDIRECTORY)).unwrap();
    let output = git!(CHECKOUT, "B2").get();
    assert!(output.stdout.ends_with("B2/src/main/java"), "Got: {}", output.stdout);
}

/// `git-checkout3` should return the same exit code as `git checkout` in an
/// empty repository.
#[test]
fn empty_directory() {
    let t = Test::new(function!());
    fs::remove_dir_all(t.as_path()).unwrap();
    fs::create_dir(t.as_path()).unwrap();
    env::set_current_dir(t.as_path()).unwrap();
    let lhs = git!(CHECKOUT, "zeno").get();
    let rhs = git!("checkout", "zeno").get();
    assert_eq!(lhs.status, rhs.status);
}

/// `git-checkout3` should return the same exit code as `git checkout` when a
/// branch doesn't exist.
#[test]
fn branch_not_exists() {
    let (_t, root) = setup(function!());
    env::set_current_dir(root.join("B1")).unwrap();
    let lhs = git!(CHECKOUT, "zeno").get();
    let rhs = git!("checkout", "zeno").get();
    assert_eq!(lhs.status, rhs.status);
}
