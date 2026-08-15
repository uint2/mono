use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome<'a> {
    // Jump to a directory.
    Jump {
        worktree: Worktree<'a>,
        relpath: &'a Path,
    },
    /// Jump first, then checkout.
    JumpAndCheckout {
        worktree: Worktree<'a>,
        branch: Branch<'a>,
        relpath: &'a Path,
    },
    /// Complete bypass.
    Bypass,
}

pub struct App<'a> {
    cwd: &'a Path,
    /// The absolute path to the root of the git workspace.
    toplevel: Option<&'a Path>,
    is_in_submodule: bool,
    git_branches: Vec<Branch<'a>>,
    git_config: GitConfig<'a>,
    bundles: Vec<Bundle<'a>>,
    config: AppConfig,
}

impl<'a> App<'a> {
    pub fn new(ctx: &'a AppCtx) -> Self {
        Self {
            cwd: ctx.cwd(),
            toplevel: ctx.toplevel(),
            is_in_submodule: ctx.is_in_submodule(),
            git_branches: ctx.branches(),
            git_config: ctx.git_config(),
            bundles: ctx.bundles(),
            config: ctx.config,
        }
    }

    /// Auto-register branches that match their worktree. So note that if a
    /// branch "dev" is checked out at a worktree whose last path component is
    /// "feature", then it won't be auto-registered.
    pub fn try_auto_register_branch(&mut self, branch: Branch<'a>) {
        log::trace!("auto-registering branch: \"{branch}\"");

        let Some(bundle) = self.bundles.iter().find(|b| b.branch == Some(branch)) else {
            return;
        };

        let branch_name_matches_worktree_dir_name =
            bundle.worktree.as_path().ends_with(branch.as_str());
        // In the docs at [https://git-scm.com/docs/git-worktree], they
        // differentiate between "main worktree" and "linked worktree".
        let is_main_worktree = bundle.worktree.as_path().join(".git").is_dir();

        if branch_name_matches_worktree_dir_name || is_main_worktree {
            log::trace!("auto-registering branch: \"{branch}\" -> {}", bundle.worktree);
            self.git_config.set(branch, bundle.worktree);
        }
    }

    /// Auto-register branches that match their worktree. So note that if a
    /// branch "dev" is checked out at a worktree whose last path component is
    /// "feature", then it won't be auto-registered.
    pub fn auto_register(&mut self) {
        for i in 0..self.git_branches.len() {
            let branch = self.git_branches[i];
            self.try_auto_register_branch(branch);
        }
        self.git_config.save();
    }

    fn prompt_user_for_worktree(&mut self, input_buf: &mut String, branch: Branch<'a>) {
        let mut f = io::stderr().lock();
        writeln!(f, "Branch \x1b[36m{branch}\x1b[m is not mapped to any worktree.")
            .unwrap();
        for (idx, bundle) in self.bundles.iter().enumerate() {
            let idx = idx + 1;
            let (left, right) = bundle.worktree.pretty_split();
            writeln!(f, "[\x1b[32m{idx}\x1b[m] {left}\x1b[32m{right}\x1b[m").unwrap();
        }
        write!(f, "Pick one > ").unwrap();
        _ = f.flush();
        drop(f);

        input_buf.clear();
        io::stdin().read_line(input_buf).unwrap();
        let choice = input_buf
            .trim()
            .parse::<usize>()
            .ok()
            .and_then(|v| v.checked_sub(1))
            .unwrap();
        let choice = &self.bundles[choice];
        self.git_config.set(branch, choice.worktree);
    }

    fn contained_in(&self, worktree: &Worktree) -> bool {
        log::info!(
            "toplevel: {:?}, cwd: {:?}, worktree: {:?}",
            self.toplevel,
            self.cwd,
            worktree
        );
        self.toplevel.map_or(false, |root| root == worktree.as_path())
            || self.cwd.starts_with(worktree)
    }

    /// Gets the current bundle/worktree that we invoked the program from.
    ///
    /// This may return None, for instance when we're in a submodule.
    pub fn get_current_bundle(&self) -> &Bundle<'a> {
        self.bundles
            .iter()
            // CWD must be a descendant directory of the worktree root.
            .filter(|b| self.contained_in(&b.worktree))
            // Favour the longest match because there is a chance that worktrees
            // are nested. For instance, when there is a bare repo.
            .max_by(|a, b| a.worktree.len().cmp(&b.worktree.len()))
            // There should be at least one match if ran in a git worktree.
            .expect("This operation should be run in a worktree that's not a submodule")
    }

    fn find_branch(&self, goal: &str) -> Option<&Branch<'a>> {
        self.git_branches.iter().find(|v| v.as_str() == goal)
    }

    fn deregister_invalid_worktrees(&mut self) {
        self.git_config.retain(|_, worktree| {
            let valid = self.bundles.iter().any(|b| b.worktree == *worktree);
            if !valid {
                log::error!("Deregister worktree: \x1b[33m{worktree}\x1b[m")
            }
            valid
        })
    }

    pub fn execute(&mut self, goal: &str) -> Outcome<'a> {
        if self.is_in_submodule {
            log::info!("Bypass because in submodule");
            return Outcome::Bypass;
        }
        self.deregister_invalid_worktrees();
        self.auto_register();

        let mut input_buf = String::new();
        for i in 0..self.git_branches.len() {
            let branch = self.git_branches[i];
            if self.git_config.get(&branch).is_none() && self.config.interactive {
                self.prompt_user_for_worktree(&mut input_buf, branch);
            }
            if self.config.interactive {
                assert!(self.git_config.get(&branch).is_some())
            }
        }

        let current_bundle = self.get_current_bundle();

        log::info!("Current worktree: {current_bundle:?}");

        // Make sure that the goal is a branch that exists. Otherwise, default
        // to `git checkout` behavior.
        //
        // This execution branchs should handle the cases where the user is
        // trying to checkout a particular file in the worktree.
        let Some(goal) = self.find_branch(goal) else {
            // At this point, `goal` is not a branch.

            // First, try to see if there is a worktree whose directory name
            // matches `goal`'s value. If so, jump there.
            let dir_match =
                self.bundles.iter().find(|b| b.worktree.as_path().ends_with(goal));
            if let Some(bundle) = dir_match {
                // Discussion: do we want this behavior? This has its
                // conveniences but it does come at a trade-off of possible
                // collisions of worktree directory names with git's files.
                //
                // Reasons for: we can have a "pr" worktree were we keep all the
                // branches related to PRs, and then just run `gco pr` to keep
                // going back there. That's pretty neat.
                log::info!(
                    "Goal \"{goal}\" is not a git branch, but found a worktree with a matching name."
                );
                return self.jump(&bundle.worktree, current_bundle);
            }
            log::info!("Goal \"{goal}\" is not a git branch, might be a file. Bypass.");
            return Outcome::Bypass;
        };

        // Get the worktree that the `goal` branch belongs to.
        let Some(mapped_worktree) = self.git_config.get(goal) else {
            panic!("Goal \"{goal}\" is not configred in git config.")
        };

        if mapped_worktree.as_str() == current_bundle.worktree.as_str() {
            // We permit the user to checkout the `goal` branch, since it
            // belongs to this worktree.
            log::info!("Goal \"{goal}\" belongs to worktree. Allow checkout.");
            return Outcome::Bypass;
        }

        // We do not permit the user to checkout the `goal` branch on this
        // current worktree, but instead direct the user to cd to that
        // owning worktree first, before jumping to the `goal` branch.

        // Here, we shall help the shell assistant a bit by first checking out
        // the `goal` branch in the mapped worktree, and then figuring out the
        // best directory to cd to.
        self.jump_and_checkout(mapped_worktree, current_bundle, goal)
    }

    fn resolve_relpath(
        &self,
        current_bundle: &Bundle,
        mapped_worktree: &Worktree,
    ) -> &'a Path {
        let mut subdir_in_repo = self.cwd.strip_prefix(&current_bundle.worktree).unwrap();
        let base = mapped_worktree.as_path();
        let mut final_dest = base.join(subdir_in_repo);
        loop {
            if final_dest == base {
                return Path::new("");
            } else if final_dest.is_dir() {
                return subdir_in_repo;
            }
            final_dest.pop(); // Move upwards together.
            subdir_in_repo = subdir_in_repo.parent().unwrap();
        }
    }

    fn jump_and_checkout(
        &self,
        mapped_worktree: &Worktree<'a>,
        current_bundle: &Bundle,
        goal: &Branch<'a>,
    ) -> Outcome<'a> {
        // Here, we shall help the shell assistant a bit by first checking out
        // the `goal` branch in the mapped worktree, and then figuring out the
        // best directory to cd to.
        // _ = git!("-C", mapped_worktree.as_str(), "checkout", goal.as_str()).output();

        let relpath = self.resolve_relpath(current_bundle, mapped_worktree);

        log::info!("Jump worktree then jump branch -> {relpath:?}");
        return Outcome::JumpAndCheckout {
            worktree: *mapped_worktree,
            branch: *goal,
            relpath,
        };
    }

    fn jump(
        &self,
        mapped_worktree: &Worktree<'a>,
        current_bundle: &Bundle,
    ) -> Outcome<'a> {
        let relpath = self.resolve_relpath(current_bundle, mapped_worktree);

        log::info!("Jump worktree then cd -> {relpath:?}");
        return Outcome::Jump { worktree: *mapped_worktree, relpath };
    }

    pub fn get_worktree(&self, branch: Branch) -> Option<Worktree<'a>> {
        self.bundles.iter().find(|v| v.branch == Some(branch)).map(|v| v.worktree)
    }

    pub fn map_branch(&mut self, branch: Branch<'a>, worktree: Worktree<'a>) {
        self.git_config.set(branch, worktree);
        self.git_config.save();
    }

    pub fn save_git_config(&self) {
        self.git_config.save();
    }

    pub fn git_config(&self) -> &GitConfig<'a> {
        &self.git_config
    }

    pub fn toplevel(&self) -> Option<&'a Path> {
        self.toplevel
    }

    pub fn get_worktrees(&self) -> Vec<Worktree<'a>> {
        self.bundles.iter().map(|v| v.worktree).collect()
    }

    pub fn bundles(&self) -> &[Bundle<'a>] {
        &self.bundles
    }
}

impl<'a, 'b> core::ops::Index<Branch<'b>> for App<'a> {
    type Output = Worktree<'a>;
    fn index(&self, branch: Branch<'b>) -> &Self::Output {
        let Some(bundle) = self.bundles.iter().find(|v| v.branch == Some(branch)) else {
            panic!("App has no bundle that has branch \"{branch}\"");
        };
        &bundle.worktree
    }
}
