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

    /// Tries to find a worktree that we can auto-register the branch to. First
    /// it finds the current worktree that the branch is checked out at.
    fn auto_find_worktree_for_branch(&self, branch: Branch<'a>) -> Option<Worktree<'a>> {
        let (bundle_index, bundle) =
            self.bundles.iter().enumerate().find(|(_, b)| b.branch == Some(branch))?;

        let branch_name_matches_worktree_dir_name =
            bundle.worktree.as_path().ends_with(branch.as_str());

        // In the docs at [https://git-scm.com/docs/git-worktree], they
        // differentiate between "main worktree" and "linked worktree".
        //
        // In the git-worktree `list` command docs, it also says: "The main
        // worktree is listed first, followed by each of the linked worktrees."
        let is_main_worktree = bundle_index == 0;

        (branch_name_matches_worktree_dir_name || is_main_worktree)
            .then_some(bundle.worktree)
    }

    /// Auto-register branches that match their worktree. So note that if a
    /// branch "dev" is checked out at a worktree whose last path component is
    /// "feature", then it won't be auto-registered.
    fn try_auto_register_branch(&mut self, branch: Branch<'a>) {
        match self.auto_find_worktree_for_branch(branch) {
            Some(worktree) => {
                log::trace!("[auto-register {branch}] -> {}", worktree);
                self.git_config.set(branch, worktree);
            }
            None => {
                log::trace!("[auto-register {branch}]: no bundle found");
            }
        }
    }

    /// Auto-register branches that match their worktree. So note that if a
    /// branch "dev" is checked out at a worktree whose last path component is
    /// "feature", then it won't be auto-registered.
    pub fn auto_register(&mut self) {
        for i in 0..self.git_branches.len() {
            let branch = self.git_branches[i];
            if let None = self.git_config.get(&branch) {
                self.try_auto_register_branch(branch);
            }
        }
        self.git_config.save();
    }

    /// Gets (and prints) a list of candidate bundles that a branch could
    /// possibly be mapped to.
    fn get_and_print_candidate_bundles(
        &self,
        branch: Branch<'a>,
    ) -> Option<Vec<&Bundle<'a>>> {
        let candidate_bundles: Vec<_> = self.bundles.iter().filter(|v| !v.bare).collect();
        if candidate_bundles.is_empty() {
            log::info!("There are no candidate bundles to suggest for branch {branch}");
            return None;
        }

        let mut f = io::stderr().lock();
        macro_rules! w  { ($($arg:tt)*) => { write!  (f, $($arg)*).unwrap() } }
        macro_rules! wl { ($($arg:tt)*) => { writeln!(f, $($arg)*).unwrap() } }
        wl!("Branch \x1b[36m{branch}\x1b[m is not mapped to any worktree.");
        let mut idx = 0;
        for bundle in &candidate_bundles {
            idx += 1;
            let (ancestors, direct_parent) = bundle.worktree.pretty_split();
            // The assigned number, where only the number is green.
            w!("[\x1b[32m{idx}\x1b[m] ");
            // The path to the worktree, where the direct parent is
            // highlighted in green.
            wl!("{ancestors}\x1b[32m{direct_parent}\x1b[m");
        }
        f.write(b"Pick one > ").unwrap();
        _ = f.flush();
        Some(candidate_bundles)
    }

    fn prompt_user_for_worktree(
        &mut self,
        input_buf: &mut String,
        branch: Branch<'a>,
    ) -> io::Result<()> {
        let Some(candidate_bundles) = self.get_and_print_candidate_bundles(branch) else {
            return Ok(());
        };

        // Read the user's input.
        input_buf.clear();
        io::stdin().read_line(input_buf).unwrap();
        // Parse the user's input as an integer.
        let Ok(choice_int) = input_buf.trim().parse::<usize>() else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid choice for mapping worktree - cannot be parsed as integer",
            ));
        };
        if !(1 <= choice_int && choice_int <= candidate_bundles.len()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid choice for mapping worktree - index out of range",
            ));
        }
        let map_choice = candidate_bundles[choice_int - 1];
        self.git_config.set(branch, map_choice.worktree);
        self.git_config.save();
        Ok(())
    }

    fn contained_in(&self, worktree: &Worktree) -> bool {
        log::info!(
            "toplevel: {:?}, cwd: {:?}, worktree: {:?}",
            self.toplevel,
            self.cwd,
            worktree
        );
        let Some(root) = self.toplevel else { return false };
        root == worktree.as_path() || self.cwd.starts_with(worktree)
    }

    /// Gets the current bundle/worktree that we invoked the program from.
    ///
    /// This may return None, for instance when we're in a submodule. Or for
    /// instance when we're in the git directory, and all the worktrees are
    /// subdirectories.
    pub fn get_current_bundle(&self) -> Option<&Bundle<'a>> {
        self.bundles
            .iter()
            // CWD must be a descendant directory of the worktree root.
            .filter(|b| self.contained_in(&b.worktree))
            // Favour the longest match because there is a chance that worktrees
            // are nested. For instance, when there is a bare repo.
            //
            // This unwrap is safe because both `a` and `b`.
            .min_by(|a, b| a.worktree.partial_cmp(&b.worktree).unwrap())
    }

    fn find_branch(&self, goal: &str) -> Option<&Branch<'a>> {
        self.git_branches.iter().find(|v| v.as_str() == goal)
    }

    /// De-registers from the git config any mapping where the worktree no
    /// longer exists.
    ///
    /// Notably, mappings where the branch is gone but the worktree still exists
    /// are kept.
    fn deregister_invalid_worktrees(&mut self) {
        self.git_config.retain(|_, worktree| {
            let worktree_exists = worktree.exists(&self.bundles);
            if !worktree_exists {
                log::info!("Deregister worktree: \x1b[33m{worktree}\x1b[m")
            }
            worktree_exists
        });
        self.git_config.save();
    }

    fn interactively_register_branches(&mut self) -> io::Result<()> {
        let mut input_buf = String::new();
        for i in 0..self.git_branches.len() {
            let branch = self.git_branches[i];
            if self.git_config.get(&branch).is_none() && self.config.interactive {
                self.prompt_user_for_worktree(&mut input_buf, branch)?;
            }
            if self.config.interactive {
                assert!(self.git_config.get(&branch).is_some())
            }
        }
        Ok(())
    }

    /// Runs the core business logic of `git-checkout4` with the target set to
    /// `goal`.
    pub fn execute(&mut self, goal: &str) -> io::Result<Outcome<'a>> {
        log::trace!("Called execute on \"{goal}\"");
        if self.is_in_submodule {
            log::info!("Bypass because in submodule");
            return Ok(Outcome::Bypass);
        }
        self.deregister_invalid_worktrees();
        self.auto_register();
        self.interactively_register_branches()?;

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
                return Ok(self.jump(&bundle.worktree, current_bundle));
            }
            log::info!("Goal \"{goal}\" is not a git branch, might be a file. Bypass.");
            return Ok(Outcome::Bypass);
        };

        // Get the worktree that the `goal` branch belongs to.
        let Some(mapped_worktree) = self.git_config.get(goal) else {
            panic!("Goal \"{goal}\" is not configured in git config.")
        };

        if let Some(current_bundle) = current_bundle {
            if mapped_worktree.as_str() == current_bundle.worktree.as_str() {
                // We permit the user to checkout the `goal` branch, since it
                // belongs to this worktree.
                log::info!("Goal \"{goal}\" belongs to worktree. Allow checkout.");
                return Ok(Outcome::Bypass);
            }
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
        current_bundle: Option<&Bundle>,
        goal: &Branch<'a>,
    ) -> io::Result<Outcome<'a>> {
        // Here, we shall help the shell assistant a bit by first checking out
        // the `goal` branch in the mapped worktree, and then figuring out the
        // best directory to cd to.
        // _ = git!("-C", mapped_worktree.as_str(), "checkout", goal.as_str()).output();

        let relpath = current_bundle
            .map(|current_bundle| self.resolve_relpath(current_bundle, mapped_worktree))
            .unwrap_or(Path::new("."));

        let target_bundle =
            self.bundles.iter().find(|v| &v.worktree == mapped_worktree).unwrap();

        if let Some(current_branch_at_target_bundle) = target_bundle.branch {
            if current_branch_at_target_bundle.eq(goal) {
                log::info!("Jump worktree without jump branch -> {relpath:?}");
                return Ok(Outcome::Jump { worktree: *mapped_worktree, relpath });
            }
        }

        log::info!("Jump worktree then jump branch -> {relpath:?}");
        Ok(Outcome::JumpAndCheckout {
            worktree: *mapped_worktree,
            branch: *goal,
            relpath,
        })
    }

    fn jump(
        &self,
        mapped_worktree: &Worktree<'a>,
        current_bundle: Option<&Bundle>,
    ) -> Outcome<'a> {
        let relpath = current_bundle
            .map(|current_bundle| self.resolve_relpath(current_bundle, mapped_worktree))
            .unwrap_or(Path::new("."));

        log::info!("Jump worktree then cd -> {relpath:?}");
        return Outcome::Jump { worktree: *mapped_worktree, relpath };
    }

    pub fn get_worktree(&self, branch: Branch) -> Option<Worktree<'a>> {
        self.bundles.iter().find(|v| v.branch == Some(branch)).map(|v| v.worktree)
    }

    /// Maps a `branch` to a git `worktree`, and saves it to the git config.
    pub fn map_branch(&mut self, branch: Branch<'a>, worktree: Worktree<'a>) {
        self.git_config.set(branch, worktree);
        self.git_config.save();
    }

    pub fn get_worktrees(&self) -> Vec<Worktree<'a>> {
        self.bundles.iter().map(|v| v.worktree).collect()
    }

    pub fn bundles(&self) -> &[Bundle<'a>] {
        &self.bundles
    }

    pub fn bundle(&self, branch: Branch<'a>) -> &Bundle<'a> {
        let Some(bundle) = self.bundles.iter().find(|v| v.branch == Some(branch)) else {
            panic!("App has no bundle that has branch \"{branch}\"");
        };
        bundle
    }

    pub fn mapped_worktree(&self, branch: Branch<'a>) -> Option<&Worktree<'a>> {
        self.git_config.get(&branch)
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
