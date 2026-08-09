use crate::prelude::*;

/// git-checkout4 app context. An owned buffer all all data to be fetched.
pub struct AppCtx {
    /// Current working directory.
    pub cwd: PathBuf,
    /// Raw output of `git rev-parse --abbrev-ref --symbolic-full-name HEAD`.
    /// Shows the current branch, or "HEAD" if HEAD is detached.
    pub r_git_branch: String,
    /// Raw output of `git branch --format=%(refname:short)`.
    /// Shows the all the local branches, separated by newlines.
    pub r_git_branches: String,
    /// Raw output of `git worktree list --porcelain`.
    /// https://git-scm.com/docs/git-worktree
    pub r_git_worktree_list: String,

    pub r_git_config: String,

    pub config: AppConfig,
}

/// Gets the git branch, and if we're currently in detached HEAD state, it will
/// print HEAD.
fn get_git_branch() -> Result<String, ()> {
    let output = git!("rev-parse", "--abbrev-ref", "--symbolic-full-name", "HEAD")
        .output()
        .map_err(|_| eprintln!("Failed to execute shell command to get git branch."))?;
    let mut output = String::from_utf8(output.stdout)
        .map_err(|_| eprintln!("Failed to parsed git branch"))?;
    output.truncate(output.as_str().trim_end().len());
    Ok(output)
}

fn get_git_branches() -> Result<String, ()> {
    let output = git!("branch", "--format=%(refname:short)")
        .output()
        .map_err(|_| eprintln!("Failed to execute shell command to get git branches."))?;
    let mut output = String::from_utf8(output.stdout)
        .map_err(|_| eprintln!("Failed to parsed git branches"))?;
    output.truncate(output.as_str().trim_end().len());
    Ok(output)
}

fn get_git_worktrees() -> Result<String, ()> {
    let output = git!("worktree", "list", "--porcelain").output().map_err(|_| {
        eprintln!("Failed to execute shell command to get git worktrees.")
    })?;
    String::from_utf8(output.stdout)
        .map_err(|_| eprintln!("Failed to parsed git worktrees"))
}

impl AppCtx {
    pub fn init(config: AppConfig) -> Result<Self, ()> {
        if config.enable_logging {
            log::init(Some(config.log_level));
        }
        let mut r_git_branch = Err(());
        let mut r_git_branches = Err(());
        let mut r_git_worktree_list = Err(());
        let mut cwd = Err(());
        let mut r_git_config = String::new();

        rayon::scope(|scope| {
            scope.spawn(|_| r_git_branch = get_git_branch());
            scope.spawn(|_| r_git_branches = get_git_branches());
            scope.spawn(|_| r_git_worktree_list = get_git_worktrees());
            scope.spawn(|_| {
                cwd = std::env::current_dir()
                    .map_err(|_| eprintln!("Unable to get current dir"))
            });
            scope.spawn(|_| r_git_config = GitConfig::read());
        });
        let cwd = cwd?;
        let r_git_branch = r_git_branch?;
        let r_git_branches = r_git_branches?;
        let r_git_worktree_list = r_git_worktree_list?;
        Ok(Self {
            cwd,
            r_git_branches,
            r_git_branch,
            r_git_worktree_list,
            r_git_config,
            config,
        })
    }

    pub fn branches<'a>(&'a self) -> Vec<Branch<'a>> {
        self.r_git_branches
            .trim()
            .lines()
            .filter(|&v| {
                // Filter out things like "(HEAD detached at <SHA>)".
                let v = v.strip_prefix('(').unwrap_or(v);
                let v = v.strip_suffix(')').unwrap_or(v);
                !v.starts_with("HEAD")
            })
            .map(Branch::new)
            .collect()
    }

    pub fn git_config<'a>(&'a self) -> GitConfig<'a> {
        GitConfig::parse(self.r_git_config.as_str(), self.cwd.as_path()).unwrap()
    }

    pub fn bundles<'a>(&'a self) -> Vec<Bundle<'a>> {
        Bundle::parse_all(self.r_git_worktree_list.as_str())
    }

    pub fn cwd(&self) -> &Path {
        self.cwd.as_path()
    }
}
