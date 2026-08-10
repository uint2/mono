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

/// Tests done with only the main worktree, and no linked worktrees.
mod primary {
    use super::*;

    #[test]
    fn checkout_an_owned_branch() {
        let t = Test::new(function!());
        t.sh2("", &["git", "init", "-b", "main"]);
        t.sh("", || some_commit("."));
        t.sh2("", &["git", "checkout", "-b", "dev"]);
        assert_eq!(git_branch(&t), "dev");

        // Register the "main" branch.
        let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        app.map_branch(branch!("main"), app.get_worktrees()[0]);

        // Re-read the updated config from filesystem.
        let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();

        let outcome = t.sh("", || App::new(&ctx).execute("main"));
        assert_eq!(outcome, Outcome::Bypass);
    }

    #[test]
    fn checkout_an_owned_branch_2() {
        let t = Test::new(function!());
        t.sh2("", &["git", "init", "-b", "main"]);
        t.sh("", || some_commit("."));

        let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
        App::new(&ctx).auto_register(); // Register the "main" branch.

        t.sh2("", &["git", "checkout", "-b", "dev"]);
        assert_eq!(git_branch(&t), "dev");

        // Re-read the updated config from filesystem.
        let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();

        let outcome = t.sh("", || App::new(&ctx).execute("main"));
        assert_eq!(outcome, Outcome::Bypass);
    }
}

mod linked {
    use super::*;

    /// Sets up a bare repo, and then links two worktrees: "main" and "dev".
    /// Also makes sure that the branches are registered to their respective
    /// worktrees.
    #[cfg(test)]
    fn setup_main_dev(t: &Test) {
        t.sh2("", &["git", "init", "-b", "main", "--bare", ".git"]);
        t.sh2("", &["git", "worktree", "add", "--orphan", "main"]);
        t.sh("main", || some_commit("."));
        t.sh2("", &["git", "worktree", "add", "dev"]);
        t.sh("dev", || some_commit("."));

        // Validate current branch of worktrees.
        assert_eq!(git_branch(&t.join("main")), "main");
        assert_eq!(git_branch(&t.join("dev")), "dev");

        let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
        let app = App::new(&ctx);
        // Check for correct registration.
        assert_eq!(app[branch!("main")].last_component(), "main");
        assert_eq!(app[branch!("dev")].last_component(), "dev");
    }

    #[test]
    fn checkout_an_unowned_branch() {
        let t = Test::new(function!());
        setup_main_dev(&t);

        let ctx = t.sh("dev", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = t.sh("dev", || app.execute("main"));
        assert_eq!(
            outcome,
            Outcome::JumpAndCheckout {
                worktree: app[branch!("main")],
                branch: branch!("main"),
                relpath: Path::new("")
            }
        );
    }

    #[test]
    fn checkout_current_branch() {
        let t = Test::new(function!());
        setup_main_dev(&t);

        let ctx = t.sh("dev", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = t.sh("dev", || app.execute("dev"));
        assert_eq!(outcome, Outcome::Bypass);
    }

    #[test]
    fn checkout_a_non_branch() {
        let t = Test::new(function!());
        setup_main_dev(&t);

        let ctx = t.sh("dev", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = t.sh("dev", || app.execute("zero"));
        assert_eq!(outcome, Outcome::Bypass);
    }

    /// When the current relative path in the repo is availble, jump to that.
    #[test]
    fn successful_dir_match_jump() {
        let t = Test::new(function!());
        setup_main_dev(&t);

        t.sh("", || some_commit("main/src/main/java"));
        t.sh("", || some_commit("dev/src/main/java/com/example"));

        let ctx = t.sh("main/src/main/java", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = t.sh("main/src/main/java", || app.execute("dev"));
        assert_eq!(
            outcome,
            Outcome::JumpAndCheckout {
                worktree: app[branch!("dev")],
                branch: branch!("dev"),
                relpath: Path::new("src/main/java")
            }
        );
    }

    /// If the relative path from the worktree root is not available, retreat back
    /// until it exists.
    #[test]
    fn nearest_dir_match_jump() {
        let t = Test::new(function!());
        setup_main_dev(&t);

        t.sh("", || some_commit("main/src/main/java"));
        t.sh("", || some_commit("dev/src/main/java/com/example"));

        let ctx = t.sh("dev/src/main/java/com/example", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = t.sh("dev/src/main/java/com/example", || app.execute("main"));
        assert_eq!(
            outcome,
            Outcome::JumpAndCheckout {
                worktree: app[branch!("main")],
                branch: branch!("main"),
                relpath: Path::new("src/main/java")
            }
        );
    }

    /// Jump from the lift lobby, with everything already registered.
    #[test]
    fn lift_lobby_registered() {
        let t = Test::new(function!());
        setup_main_dev(&t);

        let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = t.sh("", || app.execute("main"));
        assert_eq!(
            outcome,
            Outcome::JumpAndCheckout {
                worktree: app[branch!("main")],
                branch: branch!("main"),
                relpath: Path::new("")
            }
        );
    }

    /// Jump from the lift lobby, with nothing registered yet.
    #[test]
    fn lift_lobby_unregistered() {
        let t = Test::new(function!());
        t.sh("", || {
            git!("init", "--bare", ".git").snw();
            git!("worktree", "add", "main", "--orphan").snw();
        });

        let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = t.sh("", || app.execute("main"));
        let worktree = app[branch!("main")];
        assert_eq!(outcome, Outcome::Jump { worktree, relpath: Path::new("") });
    }
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
    let b_benjamin = branch!("benjamin");
    let w_diana = app[b_benjamin];
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
    let b_benjamin = branch!("benjamin");
    let w_diana = app[b_benjamin];
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

#[test]
fn do_not_handle_git_submodule() {
    let t = Test::new(function!());
    std::fs::create_dir(t.join("plenary")).unwrap();
    std::fs::create_dir(t.join("neovim")).unwrap();
    t.sh("plenary", || {
        git!("init", "-b", "main").snw();
        some_commit(".");
    });
    t.sh("neovim", || {
        git!("init", "-b", "main").snw();
        some_commit(".");
        git!(
            "-c",
            "protocol.file.allow=always",
            "submodule",
            "add",
            "../plenary/.git", // repository.
            "deps/plenary"     // directory.
        )
        .snw();
    });
    let ctx = t.sh("neovim/deps/plenary", || AppCtx::init(CONFIG)).unwrap();
    let mut app = App::new(&ctx);
    let outcome = t.sh("neovim/deps/plenary", || app.execute("main"));
    assert_eq!(outcome, Outcome::Bypass);
}
