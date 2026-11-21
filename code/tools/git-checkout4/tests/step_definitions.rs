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

fn git_with_mock_user() -> Command {
    git!("-c", "user.name=zed", "-c", "user.email=z@z.z")
}

#[given(expr = "a repo is init'd at [{}] with branch [{}]")]
fn cc1f834(t: &mut Test, cwd: PathBuf, branch: String) {
    _ = fs::create_dir_all(t.join(&cwd));
    t.sh(cwd, || git!("init", "-b", branch).snw());
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

#[given(expr = "we commit at [{}] with {string}")]
fn a37cf69(t: &mut Test, cwd: PathBuf, message: String) {
    t.sh(cwd, || git_with_mock_user().args(["commit", "-m"]).arg(message).snw());
}

#[given(expr = "we create a .git file at [{}] with gitdir = [{}]")]
fn a37cf6a(t: &mut Test, dir: PathBuf, gitdir: PathBuf) {
    let parent_dir = t.join(dir);
    _ = fs::create_dir_all(&parent_dir);
    let git_file = parent_dir.join(".git");
    fs::write(git_file, format!("gitdir: {}", t.join(gitdir).display())).unwrap();
}

#[given(expr = "a bare repo is init'd at [{}]")]
fn a3324ef(t: &mut Test, cwd: PathBuf) {
    _ = fs::create_dir_all(t.join(&cwd));
    t.sh(cwd, || git!("init", "--bare", ".git").snw());
}

#[then(expr = "the git dir of [{}] is [{}]")]
fn a3324e0(t: &mut Test, cwd: PathBuf, gitdir: PathBuf) {
    let actual_path = t.sh(cwd, || git!("rev-parse", "--git-dir").get_stdout());
    assert_eq!(realpath(actual_path), realpath(t.join(gitdir)));
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
    let actual_branch = t.sh(cwd, || {
        git!("rev-parse", "--abbrev-ref", "--symbolic-full-name", "HEAD").get_stdout()
    });
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

fn git_branches(t: &Test, cwd: &Path) -> Vec<String> {
    t.sh(cwd, || {
        git!("for-each-ref", "--format=%(refname:lstrip=2)", "refs/heads/").get_stdout()
    })
    .split('\n')
    .map(|v| v.trim())
    .filter(|v| !v.is_empty())
    .map(String::from)
    .collect::<Vec<_>>()
}

#[then(expr = "at [{}] the [{}] branch does not exist")]
fn b263a3c(t: &mut Test, cwd: PathBuf, branch: String) {
    assert!(!git_branches(t, &cwd).contains(&branch));
}

#[then(expr = "at [{}] the [{}] branch exists")]
fn b263a3d(t: &mut Test, cwd: PathBuf, branch: String) {
    assert!(git_branches(t, &cwd).contains(&branch));
}

#[given(expr = "delete at [{}] the branch [{}]")]
fn b263a3b(t: &mut Test, cwd: PathBuf, branch: String) {
    t.sh(cwd, || git!("branch", "-D", branch).snw());
}

fn cucumber_tests(rt: tokio::runtime::Runtime, print_logs: bool) {
    const LEVEL_FILTER: LevelFilter = LevelFilter::INFO;

    let t = Test::cucumber().fail_fast();

    // Initialize logging.
    let t = match print_logs {
        true => t.configure_and_init_tracing(
            format::DefaultFields::new(),
            Format::default().without_time().with_target(false),
            |layer| tracing_subscriber::registry().with(LEVEL_FILTER.and_then(layer)),
        ),
        false => t,
    };

    // The [before] hook.
    let t = t.before(|feature, _rule, scenario, world| {
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
    });

    // The [after] hook.
    let t = t.after(|_feature, _rule, _scenario, _ev, world| {
        Box::pin(async {
            if let Some(t) = world {
                t.cleanup().await;
            }
        })
    });

    // Run using tokio.
    rt.block_on(t.run("tests/features"));
}

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .worker_threads(8)
        .build()
        .unwrap();
    cucumber_tests(rt, false);
}
