#[macro_use]
mod common;

use common::*;
use git_checkout4::{App, Outcome};

use std::fs;
use std::path::Path;
use std::process::Stdio;

use regex::Regex;

const CHECKOUT: &str = "checkout4";
const MAIN: &str = "main";

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

    let app = t.sh("", App::init).unwrap();

    assert_eq!(git_branch(&t), "dev");
    let outcome = t.sh("", || app.execute("main"));
    assert_eq!(outcome, Outcome::Bypass("main"));
}

#[test]
#[ignore]
fn checkout_an_unowned_branch() {
    let t = Test::new(function!());
    t.sh2("", &["git", "init", "-b", "main"]);
    t.sh2("", &["git", "commit", "--allow-empty", "-m", "Initial commit"]);
    t.sh2("", &["git", "checkout", "-b", "dev"]);

    let dir = t.dir().to_str().unwrap();
    t.sh2("", &["git", "config", "set", "checkout4.dev.worktree", dir]);
    t.sh2("", &["git", "config", "set", "checkout4.main.worktree", "somewhere"]);

    assert_eq!(git_branch(&t), "dev");
    let output = t.sh("", || git!(CHECKOUT, "main").get());
    assert_eq!(output.stdout, "main|||somewhere");
    assert_eq!(output.stderr, "");
    assert_eq!(git_branch(&t), "dev");
}

/// When the current relative path in the repo is availble, jump to that.
#[test]
#[ignore]
fn successful_dir_match_jump() {
    let t = Test::new(function!());
    t.sh("", || {
        git!("init", "-b", "main", "--bare", ".git").snw();
        git!("worktree", "add", "--orphan", "main").snw();
        some_commit("main/src/main/java/com/example");
        git!("worktree", "add", "dev").snw();
    });

    let dev_dir = t.join("dev");
    let main_dir = t.join("main");
    let dev_dir = dev_dir.to_str().unwrap();
    let main_dir = main_dir.to_str().unwrap();
    t.sh2("", &["git", "config", "set", "checkout4.dev.worktree", dev_dir]);
    t.sh2("", &["git", "config", "set", "checkout4.main.worktree", main_dir]);

    let output = t.sh("dev/src/main/java", || git!(CHECKOUT, "main").get());
    assert_regex!(output.stdout, r"^main\|\|\|.*");
    let suggested_dir = output.stdout.split_once("|||").unwrap().1;
    assert_eq!(suggested_dir, main_dir);
}

// /// If the relative path from the worktree root is not available, retreat back
// /// until it exists.
// #[test]
// #[ignore]
// fn nearest_dir_match_jump() {
//     let t = Test::new(function!());
//     t.sh("", || {
//         git!("init", "-b", "main", "--bare", ".git").snw();
//         git!("worktree", "add", "--orphan", "main").snw();
//         some_commit("main/src/main/java");
//         git!("worktree", "add", "dev").snw();
//         some_commit("dev/src/main/java/com/example");
//     });
//
//     let output = t.sh("dev/src/main/java/com/example", || git!(CHECKOUT, "main").get());
//     assert_eq!(output.stderr, "", "Mismatch: {output:?}");
//     assert_eq!(output.status.code(), Some(64));
//     let suggest = Path::new(output.stdout.as_str()).strip_prefix(t.dir()).unwrap();
//     assert_eq!(suggest, "main/src/main/java");
// }
//

// /// Jump from the lift lobby.
// #[test]
// #[ignore]
// fn lift_lobby() {
//     let t = Test::new(function!());
//     t.sh("", || {
//         git!("init", "--bare", ".git").snw();
//         git!("worktree", "add", MAIN, "--orphan").snw();
//     });
//
//     let output = t.sh("", || git!(CHECKOUT, MAIN).get());
//     assert_eq!(output.stdout, t.join(MAIN), "Mismatch: {output:?}");
//     assert_eq!(output.status.code(), Some(64));
//
//     let output = at(t.join(".git"), || git!(CHECKOUT, MAIN).get());
//     assert_eq!(output.stdout, t.join(MAIN), "Mismatch: {output:?}");
//     assert_eq!(output.status.code(), Some(64));
// }
//
// /// Can handle bare repos.
// #[test]
// #[ignore]
// fn bare_repos() {
//     let t = Test::new(function!());
//     t.sh("", || {
//         git!("init", "-b", MAIN, "--bare", ".git").snw();
//         git!("worktree", "add", "--orphan", MAIN).snw();
//         some_commit(MAIN);
//         git!("worktree", "add", "dev").snw();
//     });
//
//     let output = at(t.join(MAIN), || git!(CHECKOUT, "dev").get());
//     assert_eq!(output.stdout, t.join("dev"), "Mismatch: {output:?}");
//     assert_eq!(output.status.code(), Some(64));
// }
//
// /// Jump from to worktree using branch name.
// #[test]
// #[ignore]
// fn jump_with_branch() {
//     let t = Test::new(function!());
//     t.sh("", || {
//         git!("init", "-b", MAIN, "--bare", ".git").snw();
//         git!("worktree", "add", "--orphan", MAIN).snw();
//         some_commit(MAIN);
//         git!("worktree", "add", "dev").snw();
//         git!("worktree", "add", "-b", "benjamin", "diana").snw();
//     });
//
//     let output = at(t.join(MAIN), || git!(CHECKOUT, "benjamin").get());
//     assert_eq!(output.stdout, t.join("diana"), "Mismatch: {output:?}");
//     assert_eq!(output.status.code(), Some(64));
// }
//
// /// Jump from to worktree using directory name.
// #[test]
// #[ignore]
// fn jump_with_directory() {
//     let t = Test::new(function!());
//     t.sh("", || {
//         git!("init", "-b", MAIN, "--bare", ".git").snw();
//         git!("worktree", "add", "--orphan", MAIN).snw();
//         some_commit(MAIN);
//         git!("worktree", "add", "dev").snw();
//         git!("worktree", "add", "-b", "benjamin", "diana").snw();
//         git!("worktree", "add", "-b", "benjamin", "diana").snw();
//     });
//
//     let output = at(t.join(MAIN), || git!(CHECKOUT, "diana").get());
//     assert_eq!(output.stdout, t.join("diana"), "Mismatch: {}", output.stdout);
//     assert_eq!(output.status.code(), Some(64));
// }
//
// /// Checkout a branch.
// #[test]
// #[ignore]
// fn checkout_branch() {
//     let t = Test::new(function!());
//     t.sh("", || {
//         git!("init", "-b", MAIN, "--bare", ".git").snw();
//         git!("worktree", "add", "--orphan", MAIN).snw();
//         some_commit(MAIN);
//         git!("worktree", "add", "-b", "benjamin", "diana").snw();
//         git!("-C", "diana", "checkout", "-b", "briana").snw();
//     });
//
//     let output = at(t.join(MAIN), || git!(CHECKOUT, "benjamin").get());
//
//     assert_eq!(output.stdout, "", "Mismatch: {output:?}");
//     assert!(output.status.success());
//     assert_eq!(git_branch(t.join(MAIN)), "benjamin");
// }
//
// /// Checkout a branch that matches the current directory.
// /// On a directory that is called "main", but is on branch "dev". Then when we
// /// checkout "main" again, the git branch should now be "main".
// #[test]
// #[ignore]
// fn checkout_branch_matches_directory() {
//     let t = Test::new(function!());
//     t.sh("", || {
//         git!("init", "-b", MAIN, "--bare", ".git").snw();
//         git!("worktree", "add", "--orphan", MAIN).snw();
//         some_commit(MAIN);
//         git!("-C", MAIN, "checkout", "-b", "dev").snw();
//     });
//
//     let output = at(t.join(MAIN), || git!(CHECKOUT, MAIN).get());
//
//     assert_eq!(output.stdout, "", "Mismatch: {output:?}");
//     assert!(output.status.success());
//     assert_eq!(git_branch(t.join(MAIN)), "main");
// }
//
// /// Checkout a sticky branch. This should result in full bypass behaviour.
// #[test]
// #[ignore]
// fn checkout_sticky() {
//     let t = Test::new(function!());
//     t.sh("", || {
//         println!("---");
//         sh!("pwd").snw();
//         git!("init", "-b", MAIN, "--bare", ".git").snw();
//         println!("---");
//         sh!("pwd").snw();
//         // git!("config", STICKY_CONFIG_KEY, "hello,world,dev,hello,world").snw();
//         println!("---");
//         sh!("pwd").snw();
//         git!("worktree", "add", "--orphan", MAIN).snw();
//         println!("---");
//         some_commit(MAIN);
//         git!("-C", MAIN, "checkout", "-b", "dev").snw();
//         git!("-C", MAIN, "checkout", "-b", "feature").snw();
//     });
//     assert_eq!(git_branch(t.join(MAIN)), "feature");
//
//     at(t.join(MAIN), || git!(CHECKOUT, "dev").get());
//     assert_eq!(git_branch(t.join(MAIN)), "dev");
// }
//
// #[test]
// #[ignore]
// fn git_config_sticky() {
//     let t = Test::new(function!());
//     const CONFIG_VALUE: &str = "hello,world";
//     t.sh("", || {
//         git!("init", "-b", MAIN, "--bare", ".git").snw();
//         // git!("config", STICKY_CONFIG_KEY, CONFIG_VALUE).snw();
//     });
//     // let output = t.sh("", || git!("config", "--get", STICKY_CONFIG_KEY).get());
//     // assert_eq!(output.stdout, CONFIG_VALUE);
//     // assert_eq!(output.stderr, "");
//     // assert!(output.status.success());
// }
//
// /// When trying to checkout another branch but currently on a sticky branch, do
// /// not jump, and print the help message.
// #[test]
// #[ignore]
// fn t7() {
//     let t = Test::new(function!());
//     t.sh("", || {
//         git!("init", "-b", MAIN, "--bare", ".git").snw();
//         // git!("config", STICKY_CONFIG_KEY, "hello,world,dev,hello,world").snw();
//         git!("worktree", "add", "--orphan", MAIN).snw();
//         some_commit(MAIN);
//         git!("worktree", "add", "dev").snw();
//     });
//     assert_eq!(git_branch(t.join("dev")), "dev");
//
//     // Use this way of getting output to be able to compare even the trailing
//     // newline.
//     let output = at(t.join("dev"), || git!(CHECKOUT, MAIN).output().unwrap());
//     let stdout = core::str::from_utf8(&output.stdout).unwrap();
//     let stderr = core::str::from_utf8(&output.stderr).unwrap();
//
//     assert_eq!(stdout, "");
//     // assert_eq!(stderr, STICKY_NO_JUMP);
//     assert!(output.status.success());
// }
//
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
//         git!("init", "-b", MAIN, "--bare", ".git").snw();
//         // git!("config", STICKY_CONFIG_KEY, "hello,world,dev,hello,world").snw();
//         git!("worktree", "add", "--orphan", MAIN).snw();
//         some_commit(MAIN);
//     });
//     let (lhs, rhs) = at(t.join(MAIN), || {
//         let lhs = git!(CHECKOUT, "zeno").get();
//         let rhs = git!("checkout", "zeno").get();
//         (lhs, rhs)
//     });
//     assert_eq!(lhs.status, rhs.status);
// }
