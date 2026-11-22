use crate::prelude::*;

/// The global git config key prefix.
macro_rules! key {
    () => {
        "checkout4" // set the global git config key used for this project here.
    };
    (regex) => {
        concat!("^", key!(), "\\.")
    };
}

/// Declares a mapping of a branch to a worktree. So that using git-checkout4,
/// this branch will only ever be checked out at this worktree's directory.
#[derive(Debug, Clone, Copy)]
struct Mapping<'a> {
    /// The mapped branch.
    branch: Branch<'a>,
    /// The worktree that the branch in this [Mapping] is mapped to.
    worktree: Worktree<'a>,
}

#[derive(Debug, Clone)]
pub struct GitConfig<'a> {
    data: Vec<Mapping<'a>>,
    /// To help with the unsetting.
    original_keys: Vec<&'a str>,
    /// To help with saving the git config reliably to the right repository
    /// each time.
    cwd: &'a Path,
}

impl<'a> GitConfig<'a> {
    /// Not thread-safe. There should only be one concurrent call to this
    /// function at any given time.
    pub fn save(&self) {
        // This is an internal mutex that guarantees that only one git command
        // is ran at any given time.
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
            for Mapping { branch, worktree } in &self.data {
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
        if let Some(current) = self.data.iter_mut().find(|v| v.branch == branch) {
            current.worktree = worktree
        } else {
            self.data.push(Mapping { branch, worktree });
        }
    }

    /// Gets the worktree that this branch is mapped to.
    pub fn get(&self, branch: &Branch<'a>) -> Option<&Worktree<'a>> {
        self.data.iter().find(|v| v.branch.eq(branch)).map(|v| &v.worktree)
    }

    pub fn remove(&mut self, branch: &Branch<'a>) {
        self.data.retain(|v| v.branch.ne(branch));
    }

    pub fn retain<F: FnMut(&Branch, &Worktree) -> bool>(&mut self, mut f: F) {
        self.data.retain(|m| f(&m.branch, &m.worktree));
    }

    /// Obtain a subset of the (local) git config that is relevant to this app.
    pub fn read() -> Result<String, ()> {
        let output =
            git!("config", "get", "--all", "-z", "--show-names", "--regexp", key!(regex))
                .output()
                .map_err(|_| eprintln!("Unable to get git config"))?;
        String::from_utf8(output.stdout)
            .map_err(|_| eprintln!("Unable to decode git config as utf-8"))
    }

    /// Parses the output of [Self::read].
    pub fn parse(raw: &'a str, cwd: &'a Path) -> Result<Self, ()> {
        let mut vec = Vec::new();
        let mut original_keys = vec![];

        let raw = raw.trim().trim_end_matches('\0').trim();
        if raw.is_empty() {
            return Ok(Self { data: vec, original_keys: Vec::new(), cwd });
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
            vec.push(Mapping { branch, worktree });
        }
        Ok(Self { data: vec, original_keys, cwd })
    }
}
