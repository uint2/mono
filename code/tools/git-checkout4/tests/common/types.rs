use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use std::{env, fs};

use cucumber::World;

static CWD_MUTEX: Mutex<()> = Mutex::new(());

#[derive(Debug)]
pub struct ShellState {
    pub proc: tokio::process::Child,
    pub cwd: PathBuf,
}

#[derive(Debug, Default, World)]
pub struct Test {
    name: String,

    /// Root directory for the test files.
    root_dir: PathBuf,

    /// Whether or not to remove the test directory after we're done.
    cleanup: bool,

    shell_state: Option<ShellState>,
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
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        let name = name.as_ref();

        static TEST_NAMES: LazyLock<Mutex<HashSet<String>>> =
            LazyLock::new(|| Mutex::new(HashSet::new()));

        let mut lock = Mutex::lock(&TEST_NAMES).unwrap();
        assert!(lock.insert(name.to_string()), "Duplicate test name: {name}");
        drop(lock);

        let bin_dir = Path::new(env!("CARGO_BIN_EXE_git-checkout4")).parent().unwrap();
        let new_path = format!("{}:{}", bin_dir.display(), env_var("PATH"));
        unsafe { env::set_var("PATH", new_path) };

        let temp_dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        Self {
            root_dir: temp_dir,
            cleanup: true,
            name: name.to_string(),
            shell_state: None,
        }
    }

    pub fn set_shell_state(&mut self, proc: tokio::process::Child, cwd: PathBuf) {
        self.shell_state = Some(ShellState { proc, cwd })
    }

    pub fn get_shell_state_mut(&mut self) -> Option<&mut ShellState> {
        self.shell_state.as_mut()
    }

    pub fn get_shell_state(&self) -> Option<&ShellState> {
        self.shell_state.as_ref()
    }

    /// Runs an arbitrary function `f` with the current directory set to a path
    /// (`relpath`) that is relative to the test's root directory.
    ///
    /// Returns the output of the `f` function.
    pub fn sh<T, P: AsRef<Path>, F: FnOnce() -> T>(&self, relpath: P, f: F) -> T {
        let _lock = CWD_MUTEX.lock();
        let cwd = self.join(relpath);
        env::set_current_dir(&cwd).unwrap();
        let result = f();
        drop(_lock);
        result
    }

    /// Runs `Path::join` on the root directory of the test.
    pub fn join<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        self.root_dir.join(p)
    }

    pub async fn cleanup(&mut self) {
        if self.cleanup {
            tracing::info!("Teardown test: {}", self.name);
            _ = fs::remove_dir_all(&self.root_dir);
        } else {
            tracing::info!("Skip teardown test: {}", self.name);
        }
        if let Some(shell_state) = self.shell_state.as_mut() {
            _ = shell_state.proc.wait().await;
        }
    }
}

impl AsRef<Path> for Test {
    fn as_ref(&self) -> &Path {
        self.root_dir.as_path()
    }
}
