mod branch;
mod worktree;

pub use branch::Branch;
pub use worktree::Worktree;

pub struct WorktreeState<'a> {
    worktree: Worktree<'a>,
    branch: Branch<'a>,
}
