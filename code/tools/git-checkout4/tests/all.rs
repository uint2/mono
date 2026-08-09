#[macro_use]
mod common;

use common::*;

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

    let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();

    assert_eq!(git_branch(&t), "dev");
    let outcome = t.sh("", || App::new(&ctx).execute("main"));
    assert_eq!(outcome, Outcome::Bypass);
}

#[test]
fn checkout_an_unowned_branch() {
    let t = Test::new(function!());
    t.sh2("", &["git", "init", "-b", "main", "--bare", ".git"]);
    t.sh2("", &["git", "worktree", "add", "--orphan", "main"]);
    t.sh2("main", &["git", "commit", "--allow-empty", "-m", "Initial commit"]);
    t.sh2("", &["git", "worktree", "add", "dev"]);

    let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);

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

    let ctx = t.sh("dev/src/main/java", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);
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

    let ctx = t.sh("dev/src/main/java/com/example", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);
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

    let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);
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

    let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);
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
    let ctx = t.sh("main", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);
    let b_benjamin = Branch::new("benjamin");
    let w_diana = app.get_worktree(b_benjamin).unwrap();
    t.sh("main", || app.map_branch(b_benjamin, w_diana));

    // Re-read the updated config from filesystem.
    let ctx = t.sh("main", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);

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
    let ctx = t.sh("main", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);
    let b_benjamin = Branch::new("benjamin");
    let w_diana = app.get_worktree(b_benjamin).unwrap();
    t.sh("main", || app.map_branch(b_benjamin, w_diana));

    // Re-read the updated config from filesystem.
    let ctx = t.sh("main", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);

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
    let ctx = t.sh("main", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);
    t.sh("main", || app.execute(""));

    // Re-read the updated config from filesystem.
    let ctx = t.sh("main", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);

    // Set branch to "dev".
    t.sh("main", || git!("checkout", "-b", "dev").snw());
    assert_eq!(t.branch_at("main"), "dev");

    let outcome = t.sh("main", || app.execute("main"));
    assert_eq!(outcome, Outcome::Bypass);
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
    let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);
    t.sh("", || {
        app.auto_register();
        app.save_git_config();
        println!("SAVED {:?}", app.git_config());
    });

    let sha = t.sh("", || git!("rev-parse", "HEAD").get_stdout());
    t.sh2("", &["git", "checkout", sha.as_str().trim()]);

    let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
    let outcome = t.sh("", || App::new(&ctx).execute("main"));
    assert_eq!(outcome, Outcome::Bypass);
}
