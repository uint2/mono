use crate::prelude::*;

use std::collections::hash_map::Iter;

macro_rules! key {
    () => {
        "checkout4" // set the global git config key used for this project here.
    };
    (regex) => {
        concat!("^", key!(), "\\.")
    };
}

#[derive(Debug, Clone)]
pub struct GitConfig<'a> {
    data: HashMap<Branch<'a>, Worktree<'a>>,
    /// To help with the unsetting.
    original_keys: Vec<&'a str>,
    /// To help with saving the git config reliably to the right repository
    /// each time.
    cwd: &'a Path,
}

impl<'a> GitConfig<'a> {
    pub fn save(&self) {
        static MUTEX: Mutex<()> = Mutex::new(());
        // Unset all original keys.
        rayon::scope(|scope| {
            for key in &self.original_keys {
                scope.spawn(move |_| {
                    let _lock = MUTEX.lock().unwrap();
                    git!("config", "unset", key)
                        .current_dir(self.cwd)
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                });
            }
        });
        // Fill in the new data.
        rayon::scope(|scope| {
            for (branch, worktree) in &self.data {
                let key = [key!(), branch.as_str(), "worktree"].join(".");
                scope.spawn(move |_| {
                    let _lock = MUTEX.lock().unwrap();
                    git!("config", "set", key.as_str(), worktree.as_str())
                        .current_dir(self.cwd)
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                });
            }
        });
    }

    pub fn set(&mut self, branch: Branch<'a>, worktree: Worktree<'a>) {
        self.data.insert(branch, worktree);
    }

    pub fn get(&self, branch: &Branch<'a>) -> Option<&Worktree<'a>> {
        self.data.get(branch)
    }

    pub fn remove(&mut self, branch: &Branch<'a>) {
        self.data.remove(branch);
    }

    pub fn retain<F: FnMut(&Branch, &mut Worktree) -> bool>(&mut self, f: F) {
        self.data.retain(f);
    }

    pub fn iter<'r>(&'r self) -> Iter<'r, Branch<'a>, Worktree<'a>> {
        self.data.iter()
    }

    pub fn read() -> String {
        const ARGS: [&str; 7] =
            ["config", "get", "--all", "-z", "--show-names", "--regexp", key!(regex)];
        let output =
            Command::new("git").args(ARGS).output().expect("Unable to get git config");
        String::from_utf8(output.stdout).expect("Unable to decode git config as utf-8")
    }

    pub fn parse(raw: &'a str, cwd: &'a Path) -> Result<Self, ()> {
        let mut ht = HashMap::new();
        let mut original_keys = vec![];

        let raw = raw.trim().trim_end_matches('\0').trim();
        if raw.is_empty() {
            return Ok(Self { data: ht, original_keys: Vec::new(), cwd });
        }

        for line in raw.split('\0') {
            let (key, value) = line.split_once('\n').unwrap();
            original_keys.push(key);
            let parsed = key
                .strip_prefix(concat!(key!(), "."))
                .and_then(|v| v.strip_suffix(".worktree"));
            let Some(key) = parsed else {
                return Err(eprintln!(concat!(
                    "Invalid git config key. Should be \"",
                    key!(),
                    ".<directory>.worktree\"."
                )));
            };
            let branch = Branch::new(key);
            let worktree = Worktree::new(value);
            ht.insert(branch, worktree);
        }
        Ok(Self { data: ht, original_keys, cwd })
    }
}

impl Drop for GitConfig<'_> {
    fn drop(&mut self) {
        self.save();
    }
}
