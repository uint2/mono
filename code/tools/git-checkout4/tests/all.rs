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
        app.map_branch(branch!("main"), app.bundles()[0].worktree);

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

    /// Checkout a branch from a detached head state.
    #[test]
    fn checkout_from_detached() {
        let t = Test::new(function!());
        t.sh2("", &["git", "init", "-b", "main"]);
        t.sh("", || some_commit("."));

        let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
        App::new(&ctx).auto_register(); // Register the "main" branch.

        let sha = t.sh("", || git!("rev-parse", "--verify", "HEAD").get_stdout());
        t.sh2("", &["git", "checkout", sha.as_str().trim()]);

        // Verify that the HEAD is detached.
        let output = t.sh("", || {
            git!("rev-parse", "--abbrev-ref", "--symbolic-full-name", "HEAD").get_stdout()
        });
        assert_eq!(output, "HEAD", "HEAD is, in fact, not detached.");

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
    pub(super) fn setup_main_dev(t: &Test) {
        t.sh2("", &["git", "init", "-b", "main", "--bare", ".git"]);
        t.sh2("", &["git", "worktree", "add", "--orphan", "main"]);
        t.sh("main", || some_commit("."));
        t.sh2("", &["git", "worktree", "add", "dev"]);
        t.sh("dev", || some_commit("."));

        // Validate current branch of worktrees.
        assert_eq!(git_branch(&t.join("main")), "main");
        assert_eq!(git_branch(&t.join("dev")), "dev");

        let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        app.auto_register();

        // Check for correct registration.
        assert_eq!(
            app.mapped_worktree(branch!("main")).map(Worktree::last_component),
            Some("main")
        );
        assert_eq!(
            app.mapped_worktree(branch!("dev")).map(Worktree::last_component),
            Some("dev")
        );

        log::info!("Completed setup: setup_main_dev");
    }

    /// Run checkout on "main", when currently on the "dev" bundle. Critically,
    /// "main" is not mapped to the current worktree, and so we need to first
    /// jump to the worktree that "main" is mapped to, and then run
    /// `git checkout main`.
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
                worktree: app.bundle(branch!("main")).worktree,
                branch: branch!("main"),
                relpath: Path::new("")
            }
        );
    }

    /// Run checkout on "dev", when currently on the "dev" bundle.
    #[test]
    fn checkout_current_branch() {
        let t = Test::new(function!());
        setup_main_dev(&t);

        let ctx = t.sh("dev", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = t.sh("dev", || app.execute("dev"));
        assert_eq!(outcome, Outcome::Bypass);
    }

    /// Run checkout on "zero", something that is not currently a valid branch
    /// name.
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
                worktree: app.bundle(branch!("dev")).worktree,
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
                worktree: app.bundle(branch!("main")).worktree,
                branch: branch!("main"),
                relpath: Path::new("src/main/java")
            }
        );
    }

    /// Checkout a branch that matches the current directory.
    /// On a directory that is called "main", but is on branch "feature". Then
    /// when we checkout "main" again, the git branch should now be "main".
    #[test]
    fn checkout_branch_matches_directory() {
        let t = Test::new(function!());
        setup_main_dev(&t);

        t.sh("main", || git!("checkout", "-b", "feature").snw());
        assert_eq!(git_branch(t.join("main")), "feature");

        let ctx = t.sh("main", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);

        let outcome = t.sh("main", || app.execute("main"));
        assert_eq!(outcome, Outcome::Bypass);
    }

    /// Checkout a branch from a detached head state.
    #[test]
    fn checkout_from_detached() {
        let t = Test::new(function!());
        setup_main_dev(&t);

        let sha = t.sh("main", || git!("rev-parse", "--verify", "HEAD").get_stdout());
        t.sh2("main", &["git", "checkout", sha.as_str().trim()]);

        // Verify that the HEAD is detached.
        let output = t.sh("main", || {
            git!("rev-parse", "--abbrev-ref", "--symbolic-full-name", "HEAD").get_stdout()
        });
        assert_eq!(output, "HEAD", "HEAD is, in fact, not detached.");

        let ctx = t.sh("main", || AppCtx::init(CONFIG)).unwrap();
        let outcome = t.sh("main", || App::new(&ctx).execute("main"));
        assert_eq!(outcome, Outcome::Bypass);
    }
}

/// These are operations done in a directory where git is aware that it's
/// active, but nothing is checked out. An example can be achived by running
/// `git init --bare .git`.
///
/// Just for fun, we shall refer to this directory as the "lift lobby".
mod lobby {
    use super::*;

    /// Jump from the lift lobby, with everything already registered.
    #[test]
    fn lift_lobby_registered() {
        let t = Test::new(function!());
        linked::setup_main_dev(&t);

        let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = t.sh("", || app.execute("main"));
        assert_eq!(
            outcome,
            Outcome::JumpAndCheckout {
                worktree: app.bundle(branch!("main")).worktree,
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
        let worktree = app.bundle(branch!("main")).worktree;
        assert_eq!(outcome, Outcome::Jump { worktree, relpath: Path::new("") });
    }
}

/// Jumps when the target's worktree and branch name do not match.
mod smart_jumps {
    use super::*;

    /// We create a linked worktree situation, where there is a "main" worktree
    /// checked out at the "main" branch (standard stuff), but then there will
    /// be a "dino" worktree checked out at the "book" branch (D for directory
    /// and B for branch).
    pub(super) fn setup_dino_book(t: &Test) {
        t.sh("", || {
            git!("init", "-b", "main", "--bare", ".git").snw();
            git!("worktree", "add", "--orphan", "main").snw();
            some_commit("main");
            git!("worktree", "add", "-b", "book", "dino").snw();
            some_commit("dino");
        });

        // Register the "book" branch.
        let ctx = t.sh("main", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let w_dino = app.bundle(branch!("book")).worktree;
        t.sh("main", || app.map_branch(branch!("book"), w_dino));

        // Validate current branch of worktrees.
        assert_eq!(git_branch(&t.join("main")), "main");
        assert_eq!(git_branch(&t.join("dino")), "book");

        let ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        app.auto_register();

        // Check for correct registration.
        assert_eq!(
            app.mapped_worktree(branch!("main")).map(Worktree::last_component),
            Some("main")
        );
        assert_eq!(
            app.mapped_worktree(branch!("book")).map(Worktree::last_component),
            Some("dino")
        );
    }

    /// Jump from to worktree using branch name.
    #[test]
    fn jump_with_branch() {
        let t = Test::new(function!());
        setup_dino_book(&t);

        let ctx = t.sh("main", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = t.sh("main", || app.execute("book"));
        assert_eq!(
            outcome,
            Outcome::JumpAndCheckout {
                worktree: app.bundle(branch!("book")).worktree,
                branch: branch!("book"),
                relpath: Path::new("")
            }
        );
    }

    /// Jump from to worktree using directory name.
    #[test]
    fn jump_with_directory() {
        let t = Test::new(function!());
        setup_dino_book(&t);

        let ctx = t.sh("main", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = t.sh("main", || app.execute("dino"));
        assert_eq!(
            outcome,
            Outcome::Jump {
                worktree: app.bundle(branch!("book")).worktree,
                relpath: Path::new("")
            }
        );
    }
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
