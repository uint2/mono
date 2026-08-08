use crate::prelude::*;

macro_rules! key {
    () => {
        "checkout4" // set the global git config key used for this project here.
    };
    (regex) => {
        concat!("^", key!(), "\\.")
    };
}

#[derive(Debug)]
pub struct Config<'b, 'w> {
    data: HashMap<Branch<'b>, Worktree<'w>>,
}

mod git_config {
    use crate::prelude::*;

    pub fn get() -> String {
        const ARGS: [&str; 7] =
            ["config", "get", "--all", "-z", "--show-names", "--regexp", key!(regex)];
        let output = Command::new("git").args(ARGS).output().unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    // `raw` is guaranteed to be a list of key-value pairs, where each pair is
    // separated by the '\0' character, and the key and values have a '\n'
    // character between them.
    // pub fn parse(raw: &str) -> Result<HashMap<CanonicalPath, Vec<Branch>>, ()> {
    //     let raw = raw.trim().trim_end_matches('\0').trim();
    //     let mut hashmap = HashMap::new();
    //
    //     for line in raw.split('\0') {
    //         let (key, value) = line.split_once('\n').unwrap();
    //         let parsed = key
    //             .strip_prefix(concat!(key!(), "."))
    //             .and_then(|v| v.strip_suffix(".branches"));
    //         let Some(key) = parsed else {
    //             return Err(());
    //             // return err!(concat!(
    //             //     "Invalid git config key. Should be \"",
    //             //     key!(),
    //             //     ".<directory>.branches\"."
    //             // ));
    //         };
    //         // let branches = value.split(',').map(|v| Branch::new(v.to_string())).collect();
    //         // let worktree = CanonicalPath::new(Path::new(key)).unwrap();
    //         // hashmap.insert(worktree, branches);
    //     }
    //     Ok(hashmap)
    // }
}

impl<'b, 'w> Config<'b, 'w> {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn save(&self) {
        rayon::scope(|scope| {
            for (branch, worktree) in &self.data {
                let key = [key!(), branch.as_str(), "worktree"].join(".");
                let val = worktree.as_str();
                scope.spawn(move |_| {
                    git!("config", "set", key, val).output().unwrap();
                });
            }
        });
    }

    pub fn set(&mut self, branch: Branch<'b>, worktree: Worktree<'w>) {
        self.data.insert(branch, worktree);
    }

    pub fn get(&self, branch: &Branch<'b>) -> Option<&Worktree<'w>> {
        self.data.get(branch)
    }

    pub fn read() -> String {
        const ARGS: [&str; 7] =
            ["config", "get", "--all", "-z", "--show-names", "--regexp", key!(regex)];
        let output =
            Command::new("git").args(ARGS).output().expect("Unable to get git config");
        String::from_utf8(output.stdout).expect("Unable to decode git config as utf-8")
    }

    pub fn parse<'a: 'b + 'w>(raw: &'a str) -> Result<Self, ()> {
        let mut ht = HashMap::new();

        let raw = raw.trim().trim_end_matches('\0').trim();
        if raw.is_empty() {
            return Ok(Self { data: ht });
        }

        for line in raw.split('\0') {
            let (key, value) = line.split_once('\n').unwrap();
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
        Ok(Self { data: ht })
    }
}
