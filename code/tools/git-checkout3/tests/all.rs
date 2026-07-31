mod common;

#[path = "../src/consts.rs"]
mod consts;

use consts::{STICKY_CONFIG_KEY, STICKY_NO_JUMP};

use common::*;

use std::env;
use std::path::Path;

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

fn git_branch(dir: &Path) -> String {
    git!("-C", dir, "branch", "--show-current").get_stdout()
}

fn git_branches(dir: &Path) -> Vec<String> {
    let output = git!("-C", dir, "branch", "--format=%(refname:short)").get_stdout();
    output.lines().map(|v| v.trim().to_string()).collect()
}

#[test]
fn worktree_list_output() {
    let (_t, root) = setup(function!());
    let output = git!("worktree", "list", "--porcelain").get();
    let s = output.stdout.as_str();
    let l: Vec<_> = s.lines().collect();
    // assert_eq!(l[0], "worktree /tmp/gco-test/repo");
    assert!(l[1].starts_with("HEAD"));
    //     assert_eq!(
    //         output.stdout,
    //         "\
    // worktree /tmp/gco-test/repo
    // HEAD f6bf9d3a00711fb4bc2df681c32c4b9c88899146
    // branch refs/heads/main
    //
    // worktree /tmp/gco-test/repo/B1
    // HEAD c4d8564470c845c57beecfed4afda7845215bbab
    // branch refs/heads/B1
    //
    // worktree /tmp/gco-test/repo/B2
    // HEAD 3151dfef7e523f0091c88130398892a87959922a
    // branch refs/heads/B2
    //
    // worktree /tmp/gco-test/repo/D3
    // HEAD 20374554ab3b648dafa72737ffd879ae6eef7b65
    // branch refs/heads/B3"
    //     );
}

#[test]
fn setup_test_branch_1() {
    let (_t, root) = setup(function!());
    assert_eq!(git_branch(&root.join("B1")), "B1");
}

#[test]
fn setup_test_branch_2() {
    let (_t, root) = setup(function!());
    assert_eq!(git_branch(&root.join("B2")), "B2");
}

#[test]
fn setup_test_branch_3() {
    let (_t, root) = setup(function!());
    assert_eq!(git_branch(&root.join("D3")), "B3");
}

#[test]
fn setup_all_branches() {
    let (_t, root) = setup(function!());
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
    assert_eq!(cd(&output.stdout), cd(root.join("B1")), "Mismatch: {}", output.stdout);
    assert_eq!(output.status.code(), Some(64));
}

/// Jump using ref, from B1 -> B2. Expected to parse:
/// fatal: 'B2' is already used by worktree at '/tmp/gco/repo/B2'
#[test]
fn t2() {
    let (_t, root) = setup(function!());
    env::set_current_dir(root.join("B1")).unwrap();
    let output = git!(CHECKOUT, "B2").get();
    assert_eq!(cd(&output.stdout), cd(root.join("B2")), "Mismatch: {}", output.stdout);
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
    assert_eq!(cd(&output.stdout), cd(root.join("D3")), "Mismatch: {}", output.stdout);
    assert_eq!(output.status.code(), Some(64));
}

/// Jump using directory, from B1 -> B3, but we use D3 as the target instead
/// of B3.
#[test]
fn t4() {
    let (_t, root) = setup(function!());
    env::set_current_dir(root.join("B1")).unwrap();
    let output = git!(CHECKOUT, "D3").get();
    assert_eq!(cd(&output.stdout), cd(root.join("D3")), "Mismatch: {}", output.stdout);
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
    git!("config", STICKY_CONFIG_KEY, "some,sticky,branches,B4,to,consider").snw();
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
    git!("config", "checkout.sticky", "some,sticky,branches,B1,to,consider").snw();
    let output = git!(CHECKOUT, "B4").output().unwrap();
    let stdout = core::str::from_utf8(&output.stdout).unwrap();
    assert_eq!(stdout, STICKY_NO_JUMP);
    assert_eq!(git_branch(&cwd), "B1");
}

/// Try to maintain relative path.
#[test]
fn t8() {
    let (_t, root) = setup(function!());
    let cwd = root.join("B1");
    env::set_current_dir(&cwd).unwrap();
    git!("config", "checkout.sticky", "some,sticky,branches,B1,to,consider").snw();
    let output = git!(CHECKOUT, "B4").output().unwrap();
    let stdout = core::str::from_utf8(&output.stdout).unwrap();
    assert_eq!(stdout, STICKY_NO_JUMP);
    assert_eq!(git_branch(&cwd), "B1");
}

/// `git-checkout3` should return the same exit code as `git checkout` in an
/// empty repository.
#[test]
fn empty_directory() {
    let t = Test::new("gco-test");
    let _ = std::fs::remove_dir_all(t.as_path()).unwrap();
    std::fs::create_dir(t.as_path()).unwrap();
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
