use crate::prelude::*;

macro_rules! git_config_key {
    () => {
        "checkout3" // set the global git config key used for this project here.
    };
    (regex) => {
        concat!("^", git_config_key!(), "\\.")
    };
}

#[derive(Debug)]
pub struct Config {
    data: HashMap<CanonicalPath, Vec<Branch>>,
}

mod git_config {
    use crate::prelude::*;

    /// Get all the values from the local git config that start with
    /// "checkout3".
    pub fn get() -> String {
        const ARGS: [&str; 7] = [
            "config",
            "get",
            "--all",
            "-z",
            "--show-names",
            "--regexp",
            git_config_key!(regex),
        ];
        let output = Command::new("git").args(ARGS).output().unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    /// `raw` is guaranteed to be a list of key-value pairs, where each pair is
    /// separated by the '\0' character, and the key and values have a '\n'
    /// character between them.
    pub fn parse(raw: &str) -> Result<HashMap<CanonicalPath, Vec<Branch>>, ()> {
        let raw = raw.trim().trim_end_matches('\0').trim();
        let mut hashmap = HashMap::new();

        for line in raw.split('\0') {
            let (key, value) = line.split_once('\n').unwrap();
            let parsed = key
                .strip_prefix(concat!(git_config_key!(), "."))
                .and_then(|v| v.strip_suffix(".branches"));
            let Some(key) = parsed else {
                return err!(concat!(
                    "Invalid git config key. Should be \"",
                    git_config_key!(),
                    ".<directory>.branches\"."
                ));
            };
            let branches = value.split(',').map(|v| Branch::new(v.to_string())).collect();
            let worktree = CanonicalPath::new(Path::new(key)).unwrap();
            hashmap.insert(worktree, branches);
        }
        Ok(hashmap)
    }
}

impl Config {
    pub fn save(&self) {
        rayon::scope(|scope| {
            for (key, value) in &self.data {
                let values =
                    value.iter().map(|v| v.as_str()).collect::<Vec<_>>().join(",");
                scope.spawn(move |_| {
                    git!("config", "set", format!("checkout.\"{key}\".branches"), values)
                        .output()
                        .unwrap();
                });
            }
        });
    }

    pub fn load() -> Result<Self, ()> {
        let raw = git_config::get();
        Ok(Self { data: git_config::parse(&raw)? })
    }
}
