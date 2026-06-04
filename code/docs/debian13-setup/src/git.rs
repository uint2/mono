use std::fs;
use std::path::Path;

pub struct GitRepo {
    remote_url: &'static str,
    tag: &'static str,
}

impl GitRepo {
    pub const fn new(remote_url: &'static str, tag: &'static str) -> Self {
        Self { remote_url, tag }
    }

    fn clone<P: AsRef<Path>>(&self, dir: P) {
        sh!("git", "clone", self.remote_url, dir.as_ref());
    }

    fn checkout_at<P: AsRef<Path>>(&self, dir: P) {
        cmd!("git", "checkout", self.tag).current_dir(dir).run().unwrap();
    }

    fn clean<P: AsRef<Path>>(&self, dir: P) {
        cmd!("git", "clean", "-fxd").current_dir(dir).run().unwrap();
    }

    pub fn ensure_exists_at(&self, dir: &Path) {
        if !is_inside_worktree(dir)
            || get_git_remote_origin(dir) != self.remote_url
        {
            let _ = fs::remove_dir_all(dir);
            self.clone(dir);
        }
        self.clean(dir);
        cmd!("git", "checkout", ".").current_dir(dir).run().unwrap();
        self.checkout_at(dir);
    }
}

/// Checkis if `dir` is in a git worktree.
pub fn is_inside_worktree<P>(dir: P) -> bool
where
    P: AsRef<Path>,
{
    let out = cmd!("git", "rev-parse", "--is-inside-work-tree")
        .current_dir(dir)
        .collect_stdout()
        .run();
    let Ok(out) = out else { return false };
    out.status.success() && out.stdout.map_or(false, |v| v.trim() == "true")
}

/// Gets the git remote for "origin" at directory `dir`. Undefined behaviour if
/// `dir` is not in a git worktree.
pub fn get_git_remote_origin<P>(dir: P) -> String
where
    P: AsRef<Path>,
{
    let out = cmd!("git", "remote", "get-url", "origin")
        .current_dir(dir)
        .collect_stdout()
        .run()
        .unwrap();
    out.stdout.unwrap().trim().to_string()
}
