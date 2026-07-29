use std::path::{Path, PathBuf};
use std::{env, fs};

pub struct Test {
    // Root directory for the test files.
    root_dir: PathBuf,
    // Unique id for anything.
    id: u32,
}

/// Get the path to the debug binary
fn bin_dir() -> String {
    let mut p = env::current_exe().unwrap();
    (p.pop(), p.pop(), p.to_string_lossy().trim().to_string()).2
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
    pub fn new(suffix: &'static str) -> Self {
        let new_path = format!("{}:{}", bin_dir(), env_var("PATH"));
        unsafe { env::set_var("PATH", new_path) };

        let temp_dir = env::temp_dir().join(suffix);
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        Self { root_dir: temp_dir, id: 0 }
    }

    pub fn as_path(&self) -> &Path {
        self.root_dir.as_path()
    }

    // Gets a unique id.
    pub fn id(&mut self) -> u32 {
        self.id += 1;
        self.id
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root_dir);
    }
}
