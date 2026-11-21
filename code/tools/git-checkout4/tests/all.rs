#[macro_use]
mod common;

use common::*;

use cucumber::gherkin::Step;
use cucumber::{World, given, then, when};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::{self, Format};
use tracing_subscriber::prelude::*;

use core::mem;
use std::hash::{Hash, Hasher};
use std::process::{Command, Stdio};
use std::time::Duration;

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

/// Special cases, as seen in real life use.
mod specials {

    #[test]
    fn work_01() {
        let t = Test::new(function!());

        let t = t.no_cleanup();

        // Mock an external project (google's xz) at "./internet/xz".
        t.sh("", || fs::create_dir_all("internet/xz").unwrap());
        t.sh2("internet/xz", &["git", "init", "-b", "main"]);
        t.sh("internet/xz", || some_commit("lib"));

        // Start the environment repository at "./delta".
        t.sh("", || fs::create_dir("delta").unwrap());
        t.sh2("delta", &["git", "init", "-b", "main"]);

        // Add the xz project as a submodule.
        t.sh("delta", || {
            git!(
                "-c",
                "protocol.file.allow=always", // to allow using local repos as submodules
                "submodule",
                "add",
                t.join("internet/xz/.git"),
                "deps/xz"
            )
            .snw()
        });
        t.sh2("delta", &["git", "commit", "-m", "Add first submodule"]);

        t.sh("delta", || fs::create_dir_all("workspace/xz").unwrap());
        let git_dir =
            t.sh("delta/deps/xz", || git!("rev-parse", "--git-dir").get_stdout());
        assert!(Path::new(&git_dir).starts_with(t.join("delta")));

        // Manually create the ".git" file in the workspace root.
        t.sh("delta/workspace/xz", || {
            fs::write(".git", format!("gitdir: {}", git_dir)).unwrap()
        });
        assert_eq!(
            t.sh("delta/workspace/xz", || git!("rev-parse", "--git-dir").get_stdout()),
            git_dir,
        );

        t.sh2("delta/workspace/xz", &["git", "worktree", "add", "dev"]);

        let ctx = t.sh("delta/workspace/xz", || AppCtx::init(CONFIG)).unwrap();
        let mut app = App::new(&ctx);
        let outcome = app.execute("dev");
        assert_eq!(
            outcome,
            Outcome::JumpAndCheckout {
                worktree: app.bundle(branch!("dev")).worktree,
                relpath: Path::new("."),
                branch: branch!("dev")
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

fn gather_feature_files() -> Vec<PathBuf> {
    let mut features = vec![];
    let mut it = walkdir::WalkDir::new("tests").into_iter();
    while let Some(dir_entry) = it.next() {
        let Ok(dir_entry) = dir_entry else { continue };
        if !dir_entry.file_type().is_file() {
            continue;
        }
        let filepath = dir_entry.path();
        let Some(extension) = filepath.extension() else { continue };
        if extension == "feature" {
            features.push(dir_entry.into_path());
        }
    }
    features
}

/// Generates a random hex string of a particular length.
fn fingerprint(len: usize) -> String {
    fn mini() -> u64 {
        let fingerprint = std::time::UNIX_EPOCH.elapsed().unwrap();
        let mut h = std::hash::DefaultHasher::new();
        fingerprint.hash(&mut h);
        h.finish()
    }
    let mut fpr = String::new();
    while fpr.len() < len {
        fpr.push_str(&format!("{:x}", mini()))
    }
    fpr.truncate(len);
    fpr
}

#[given(expr = "a repo is init'd at [{}] with branch [{}]")]
fn cc1f834(t: &mut Test, cwd: PathBuf, branch: String) {
    _ = fs::create_dir_all(t.join(&cwd));
    t.sh(cwd, || git!("init", "-b", branch).snw());
}

fn git_with_mock_user() -> Command {
    git!("-c", "user.name=zed", "-c", "user.email=z@z.z")
}

#[given(expr = "there is an empty commit at [{}]")]
fn a37af66(t: &mut Test, cwd: PathBuf) {
    const M: &str = "Just an empty commit";
    t.sh(cwd, || git_with_mock_user().args(["commit", "--allow-empty", "-m", M]).snw());
}

#[given(expr = "there is a non-trivial commit at [{}]")]
fn a37cf66(t: &mut Test, cwd: PathBuf) {
    let cwd = cwd.as_path();

    // Write to the target file.
    let target_file = t.join(cwd).join(fingerprint(8));
    fs::write(&target_file, fingerprint(8)).unwrap();

    // Stage this file.
    t.sh(cwd, || git!("add", target_file).snw());

    // Commit this change.
    let message = format!("New feature @ {}", fingerprint(8));
    t.sh(cwd, || git_with_mock_user().args(["commit", "-m"]).arg(message).snw());
}

#[given(expr = "there is a non-trivial commit on file [{}] at [{}]")]
fn a37cf67(t: &mut Test, file: PathBuf, cwd: PathBuf) {
    let cwd = cwd.as_path();

    // Write to the target file.
    let target_file = t.join(cwd).join(file);
    if let Some(parent) = target_file.parent() {
        _ = fs::create_dir_all(parent);
    }
    fs::write(&target_file, fingerprint(8)).unwrap();

    // Stage this file.
    t.sh(cwd, || git!("add", target_file).snw());

    // Commit this change.
    let message = format!("New feature @ {}", fingerprint(8));
    t.sh(cwd, || git_with_mock_user().args(["commit", "-m"]).arg(message).snw());
}

#[given(expr = "a bare repo is init'd at [{}]")]
fn a3324ef(t: &mut Test, cwd: PathBuf) {
    _ = fs::create_dir_all(t.join(&cwd));
    t.sh(cwd, || git!("init", "--bare", ".git").snw());
}

#[given(expr = "there is an orphan worktree at [{}]")]
fn eb394e4(t: &mut Test, cwd: PathBuf) {
    t.sh("", || git!("worktree", "add", cwd, "--orphan").snw());
}

#[given(expr = "checkout at [{}] a new branch [{}]")]
fn a3296bc(t: &mut Test, cwd: PathBuf, branch: String) {
    t.sh(cwd, || git!("checkout", "-B", branch).snw());
}

#[given(expr = "at [{}] we checkout [{}]")]
fn cdbafc0(t: &mut Test, cwd: PathBuf, thing: String) {
    t.sh(cwd, || git!("checkout", thing).snw());
}

#[given(expr = "at [{}] we checkout the current commit")]
fn cdbafc1(t: &mut Test, cwd: PathBuf) {
    let head = t.sh(&cwd, || git!("rev-parse", "--verify", "HEAD").get_stdout());
    t.sh(cwd, || git!("checkout", head).snw());
}

#[then("the output of listing worktrees matches regex")]
fn dd29f36(t: &mut Test, step: &Step) {
    assert!(step.docstring().is_some());
    let regex = step.docstring().unwrap();
    let stdout = t.sh("", || git!("worktree", "list", "--porcelain").get_stdout());
    assert_regex!(stdout, regex.trim());
    tracing::info!("{}", t.sh("", || git!("log").get_stdout()));
}

#[then(expr = "the current branch at [{}] is [{}]")]
fn d0a0305(t: &mut Test, cwd: PathBuf, branch: String) {
    let actual_branch = git_branch(t.join(cwd));
    assert_eq!(actual_branch, branch);
}

#[when(expr = "we run [{}] at [{}]")]
fn e774636(t: &mut Test, args: String, cwd: PathBuf) {
    let args = args.split(' ').collect::<Vec<_>>();
    let proc = t.sh(&cwd, || {
        let mut git = tokio::process::Command::new(&args[0]);
        git.args(&args[1..]);
        git.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .unwrap()
    });
    t.set_shell_state(proc, cwd);
}

mod so_far {
    use super::*;
    use tokio::io::AsyncReadExt;
    use tokio::process::{ChildStderr, ChildStdout};
    use tokio::time::sleep;

    const CHUNK_SIZE: usize = 1024;
    const INTERVAL: Duration = Duration::from_millis(200);

    fn convert(x: &[u8]) -> String {
        core::str::from_utf8(x).unwrap().trim().to_string()
    }

    pub(super) async fn stderr(reader: &mut ChildStderr) -> String {
        let mut output = Vec::new();
        let mut chunk = [0u8; CHUNK_SIZE];
        loop {
            let winner = tokio::select! {
                res = reader.read(&mut chunk) => res,
                _ = sleep(INTERVAL) => Ok(0),
            };
            match winner {
                Ok(0) | Err(_) => return convert(&output),
                Ok(n) => output.extend_from_slice(&chunk[..n]),
            }
        }
    }

    pub(super) async fn stdout(reader: &mut ChildStdout) -> String {
        let mut output = Vec::new();
        let mut chunk = [0u8; CHUNK_SIZE];
        loop {
            let winner = tokio::select! {
                res = reader.read(&mut chunk) => res,
                _ = sleep(INTERVAL) => Ok(0),
            };
            match winner {
                Ok(0) | Err(_) => return convert(&output),
                Ok(n) => output.extend_from_slice(&chunk[..n]),
            }
        }
    }
}

/// Trims the ending path separators from a path and exports that as a String.
fn trim_end_sep<P: AsRef<Path>>(path: P) -> String {
    let path_str = path.as_ref().to_str().unwrap();
    path_str.trim_end_matches(std::path::MAIN_SEPARATOR_STR).to_string()
}

fn expected_output_from_docstring(t: &mut Test, step: &Step) -> String {
    assert!(step.docstring().is_some());
    let text = step.docstring().unwrap().trim();

    let shell_state = t.get_shell_state().unwrap();
    let head = t.sh(&shell_state.cwd, || {
        git!("rev-parse", "--short", "--verify", "HEAD").get_stdout()
    });

    text.replace("${{ test.dir }}", trim_end_sep(&t).as_str())
        .replace("${{ git.HEAD.short }}", &head[0..7])
}

#[then("stdout is")]
async fn e774731(t: &mut Test, step: &Step) {
    let shell_state = t.get_shell_state_mut().unwrap();
    let stdout = shell_state.proc.stdout.as_mut().unwrap();
    let stdout = so_far::stdout(stdout).await;
    let stdout = strip_ansi_escapes::strip_str(&stdout);
    assert_eq!(stdout, expected_output_from_docstring(t, step));
}

#[then("stderr is")]
async fn e774732(t: &mut Test, step: &Step) {
    let shell_state = t.get_shell_state_mut().unwrap();
    let stderr = shell_state.proc.stderr.as_mut().unwrap();
    let stderr = so_far::stderr(stderr).await;
    let stderr = strip_ansi_escapes::strip_str(&stderr);
    assert_eq!(stderr, expected_output_from_docstring(t, step));
}

#[when(expr = "we feed keys {string}")]
async fn dcbc1c2(t: &mut Test, feedkeys: String) {
    use tokio::io::AsyncWriteExt;

    let shell_state = t.get_shell_state_mut().unwrap();
    let stdin = shell_state.proc.stdin.as_mut().unwrap();
    stdin.write(feedkeys.as_bytes()).await.unwrap();
    stdin.write(b"\n").await.unwrap();
    stdin.flush().await.unwrap();
}

#[then(expr = "running [git-checkout4 {}] at [{}] results in a bypass")]
fn e774736(t: &mut Test, goal: String, cwd: PathBuf) {
    let proc = t.sh(&cwd, || {
        let mut git = tokio::process::Command::new("git");
        git.args(["checkout4", goal.as_str()]);
        git.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .unwrap()
    });
    t.set_shell_state(proc, cwd.clone());
    let ctx = t.sh(cwd, || AppCtx::init(CONFIG)).unwrap();
    let actual_outcome = App::new(&ctx).execute(goal.as_str());
    assert_eq!(actual_outcome, Outcome::Bypass);
}

#[then(expr = "running [git-checkout4 {}] at [{}] yields")]
fn bbca0ff(t: &mut Test, step: &Step, goal: String, cwd: PathBuf) {
    let ctx = t.sh(cwd, || AppCtx::init(CONFIG)).unwrap();
    let actual_outcome = App::new(&ctx).execute(goal.as_str());

    assert!(step.docstring().is_some());
    let _expected_outcome = step.docstring().unwrap();
    assert_eq!(actual_outcome, Outcome::Bypass);
}

#[given("init app")]
fn bbca0fd(t: &mut Test) {
    let _ctx = t.sh("", || AppCtx::init(CONFIG)).unwrap();
}

#[then("experiment")]
fn experiment(_t: &mut Test) {
    // let x = t.sh("", || git!("config", "get"))
}

#[then(expr = "the git config value at [{}] for {string} is {string}")]
fn a483d6d(t: &mut Test, cwd: PathBuf, config_key: String, expected_value: String) {
    let x = t.sh(&cwd, || git!("rev-parse", "--git-dir").get_stdout());
    tracing::info!("\n{}", fs::read_to_string(Path::new(&x).join("config")).unwrap());
    let actual_value = t.sh(cwd, || git!("config", "get", config_key).get_stdout());
    assert_eq!(actual_value, expected_value);
}

fn realpath<P: AsRef<Path>>(x: P) -> PathBuf {
    let x = x.as_ref();
    let Ok(x) = x.canonicalize() else {
        panic!("Unable to canonicalize path: {x:?}");
    };
    x
}

#[then(expr = "at [{}] the mapped worktree for branch [{}] is [{}]")]
fn fd2b803(t: &mut Test, cwd: PathBuf, branch: String, relpath: PathBuf) {
    let actual_value = t.sh(cwd, || {
        git!("config", "get", format!("checkout4.{branch}.worktree")).get_stdout()
    });
    assert_eq!(realpath(actual_value), realpath(t.join(relpath)));
}

#[then(expr = "at [{}] there is no mapped worktree for branch [{}]")]
fn fd2b804(t: &mut Test, cwd: PathBuf, branch: String) {
    let actual_value = t.sh(cwd, || {
        git!("config", "get", format!("checkout4.{branch}.worktree")).get_stdout()
    });
    assert!(actual_value.trim().is_empty());
}

#[given(expr = "a submodule is added at [{}] with repo [{}] and directory [{}]")]
fn d7528d5(t: &mut Test, cwd: PathBuf, repository: PathBuf, relpath: PathBuf) {
    let local_repo = t.join(repository);
    t.sh(cwd, || {
        git!(
            "-c",
            "protocol.file.allow=always",
            "submodule",
            "add",
            "--",
            local_repo,
            relpath
        )
        .snw()
    });
}

#[given(expr = "a worktree is added at [{}] called [{}]")]
fn d7528d6(t: &mut Test, cwd: PathBuf, thing: String) {
    t.sh(cwd, || git!("worktree", "add", "--", thing).snw());
}

#[then(expr = "the file [{}] exists at [{}]")]
fn b263a3a(t: &mut Test, filename: String, relpath: PathBuf) {
    let filepath = t.join(relpath).join(filename);
    assert!(filepath.is_file(), "File not found: {filepath:?}");
}

fn cucumber_tests(print_logs: bool) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .worker_threads(8)
        .build()
        .unwrap();

    const LEVEL_FILTER: LevelFilter = LevelFilter::INFO;

    let t = Test::cucumber().fail_fast();
    let t = match print_logs {
        true => t.configure_and_init_tracing(
            format::DefaultFields::new(),
            Format::default().without_time().with_target(false),
            |layer| tracing_subscriber::registry().with(LEVEL_FILTER.and_then(layer)),
        ),
        false => t,
    };
    let t = t
        .before(|feature, _rule, scenario, world| {
            let fpr = fingerprint(8);
            tracing::info!(
                "----------------------------------------
 -----
 Feature:  {}
 Scenario: {}
 Test ID:  {fpr}
 -----",
                feature.name,
                scenario.name
            );
            _ = mem::replace(world, Test::new(fpr));
            Box::pin(async {})
        })
        .after(|_feature, _rule, _scenario, _ev, world| {
            world.map(|v| v.cleanup());
            Box::pin(async {})
        });
    rt.block_on(t.run("tests/features"));
}

fn main() {
    cucumber_tests(false);
}
