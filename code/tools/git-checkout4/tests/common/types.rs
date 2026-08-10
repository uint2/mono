use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use std::{env, fs};

use super::CommandExt;

const BIN: &str = env!("CARGO_BIN_EXE_git-checkout4");
const TMPDIR: &str = env!("CARGO_TARGET_TMPDIR");

static CWD_MUTEX: Mutex<()> = Mutex::new(());

pub fn at<T, P: AsRef<Path>, F: FnOnce() -> T>(workdir: P, f: F) -> T {
    let _lock = CWD_MUTEX.lock();
    env::set_current_dir(workdir).unwrap();
    f()
}

pub struct Test {
    // Root directory for the test files.
    root_dir: PathBuf,
}

/// Gets an environment variable with a maximum of 100 retries.
fn env_var(name: &str) -> String {
    const MAX_RETRIES: u8 = 100;
    for _ in 0..MAX_RETRIES {
        let Ok(value) = env::var(name) else { continue };
        if !value.trim_matches(char::from(0)).is_empty() {
            return value;
        }
    }
    panic!("Exceeded max retries while trying to get env: {name}");
}

impl Test {
    /// Creates an empty directory in which to start a test.
    pub fn new(name: &'static str) -> Self {
        static TEST_NAMES: LazyLock<Mutex<HashSet<&'static str>>> =
            LazyLock::new(|| Mutex::new(HashSet::new()));

        let mut lock = Mutex::lock(&TEST_NAMES).unwrap();
        assert!(lock.insert(name), "Duplicate test name: {name}");
        drop(lock);

        let bin_dir = Path::new(BIN).parent().unwrap();
        let new_path = format!("{}:{}", bin_dir.display(), env_var("PATH"));
        unsafe { env::set_var("PATH", new_path) };

        let temp_dir = Path::new(TMPDIR).join(name);
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        Self { root_dir: temp_dir }
    }

    pub fn sh<T, P: AsRef<Path>, F: FnOnce() -> T>(&self, relpath: P, f: F) -> T {
        let _lock = CWD_MUTEX.lock();
        let cwd = self.join(relpath);
        env::set_current_dir(&cwd).unwrap();
        let result = f();
        drop(_lock);
        result
    }

    pub fn sh2<P: AsRef<Path>>(&self, relpath: P, args: &[&str]) {
        std::process::Command::new(args[0])
            .args(&args[1..])
            .current_dir(self.join(relpath))
            .snw();
    }

    /// Get the git branch.
    pub fn branch_at<P: AsRef<Path>>(&self, relpath: P) -> String {
        std::process::Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "--symbolic-full-name", "HEAD"])
            .current_dir(self.join(relpath))
            .get_stdout()
    }

    pub fn join<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        self.root_dir.join(p)
    }
}

impl AsRef<Path> for Test {
    fn as_ref(&self) -> &Path {
        self.root_dir.as_path()
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        _ = fs::remove_dir_all(&self.root_dir);
    }
}
