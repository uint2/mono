use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use std::{env, fs};

use core::sync::atomic::{AtomicU32, Ordering};

const BIN: &str = env!("CARGO_BIN_EXE_git-checkout3");
const TMPDIR: &str = env!("CARGO_TARGET_TMPDIR");

static CWD_MUTEX: Mutex<()> = Mutex::new(());

pub fn at<T, P: AsRef<Path>, F: FnOnce() -> T>(workdir: P, f: F) -> T {
    let lock = CWD_MUTEX.lock();
    env::set_current_dir(workdir).unwrap();
    let result = f();
    drop(lock);
    result
}

pub struct Test {
    // Root directory for the test files.
    root_dir: PathBuf,
    // Unique id for anything.
    id: u32,
}

/// Gets an environment variable with a maximum of 100 retries.
fn env_var(name: &str) -> String {
    let mut max_retries: usize = 100;
    let mut path = env::var(name).ok();
    loop {
        if max_retries == 0 {
            panic!("Exceeded max retries while trying to get env: {name}");
        }
        max_retries -= 1;
        match path {
            Some(v) if !v.trim_matches(char::from(0)).is_empty() => return v,
            _ => path = env::var(name).ok(),
        }
    }
}

impl Test {
    pub fn new(name: &'static str) -> Self {
        static TEST_NAMES: LazyLock<Mutex<HashSet<&'static str>>> =
            LazyLock::new(|| Mutex::new(HashSet::new()));
        let mut lock = TEST_NAMES.lock().unwrap();
        assert!(lock.insert(name), "Duplicate test name: {name}");
        drop(lock);

        let bin_dir = {
            let mut p = PathBuf::from(BIN);
            p.pop();
            p
        };
        let new_path = format!("{}:{}", bin_dir.display(), env_var("PATH"));
        unsafe { env::set_var("PATH", new_path) };

        let temp_dir = Path::new(TMPDIR).join(name);
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        Self { root_dir: temp_dir, id: 0 }
    }

    pub fn sh<T, P: AsRef<Path>, F: FnOnce() -> T>(&self, relpath: P, f: F) -> T {
        let _lock = CWD_MUTEX.lock();
        env::set_current_dir(self.join(relpath)).unwrap();
        f()
    }

    pub fn as_path(&self) -> &Path {
        self.root_dir.as_path()
    }

    pub fn dir(&self) -> &Path {
        self.root_dir.as_path()
    }

    pub fn join<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        self.root_dir.join(p)
    }

    // Gets a unique id.
    pub fn id(&self) -> u32 {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        id
    }
}

impl AsRef<Path> for Test {
    fn as_ref(&self) -> &Path {
        self.root_dir.as_path()
    }
}
