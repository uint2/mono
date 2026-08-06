pub const CONST_PREFIX_CONFIG_KEY: &str = "checkout.const-prefix";
pub const CONST_PREFIX_NO_JUMP_MESSAGE: &str = "\
Cannot checkout a prefix-constrained branch.
See `git config --get checkout.const-prefix` for the prefixes fixed.
These are only checkout-able at worktrees whose directory matches the prefix.
";
