mod common;

#[path = "../src/consts.rs"]
mod consts;

use consts::{STICKY_CONFIG_KEY, STICKY_NO_JUMP};

use common::*;

use std::path::Path;
use std::{env, fs};

const CHECKOUT: &str = "checkout3";

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

/// Jump from the lift-lobby (git workspace area, but not in any git workspace)
#[test]
fn t1() {
    let (_t, root) = setup(function!());
    env::set_current_dir(&root).unwrap();
    let output = git!(CHECKOUT, "B1").get();
    assert_eq!(output.stdout, root.join("B1"), "Mismatch: {}", output.stdout);
    assert_eq!(output.status.code(), Some(64));
}

/// Jump using ref, from B1 -> B2. Expected to parse:
/// fatal: 'B2' is already used by worktree at '/tmp/gco/repo/B2'
#[test]
fn t2() {
    let (_t, root) = setup(function!());
    env::set_current_dir(root.join("B1")).unwrap();
    let output = git!(CHECKOUT, "B2").get();
    assert_eq!(output.stdout, root.join("B2"), "Mismatch: {}", output.stdout);
    assert_eq!(output.status.code(), Some(64));
}

/// Jump using ref, from B1 -> B3, but where the directory doesn't match the
/// branch name:
/// fatal: 'B3' is already used by worktree at '/tmp/gco/repo/D3'
#[test]
fn t3() {
    let (_t, root) = setup(function!());
    env::set_current_dir(root.join("B1")).unwrap();
    let output = git!(CHECKOUT, "B3").get();
    assert_eq!(output.stdout, root.join("D3"), "Mismatch: {}", output.stdout);
    assert_eq!(output.status.code(), Some(64));
}

/// Jump using directory, from B1 -> B3, but we use D3 as the target instead
/// of B3.
#[test]
fn t4() {
    let (_t, root) = setup(function!());
    env::set_current_dir(root.join("B1")).unwrap();
    let output = git!(CHECKOUT, "D3").get();
    assert_eq!(output.stdout, root.join("D3"), "Mismatch: {}", output.stdout);
    assert_eq!(output.status.code(), Some(64));
}

#[test]
fn t5() {
    let (_t, root) = setup(function!());
    let cwd = root.join("B1");
    env::set_current_dir(&cwd).unwrap();
    git!(CHECKOUT, "B4").snw();
    assert_eq!(git_branch(&cwd), "B4");
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
