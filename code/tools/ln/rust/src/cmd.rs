use std::process::{Command, Stdio};

/// Git log PRETTY FORMATS options.
/// %h  : abbreviated commit hash
/// %ar : author date, relative
/// %s  : subject
/// %D  : ref names without the " (", ")" wrapping.
const GIT_LOG_ARGS: [&str; 6] = [
    "-c",
    "color.diff.commit=241", // Colors the parentheses around the refs.
    "log",
    "--graph",
    "--color=always",
    concat!(
        "--format=",
        "%C(yellow)%h",                                        // commit SHA
        "%C(auto)",                                            // ref colors
        "%(decorate:prefix= {,suffix=},pointer= \x1b[33m-> )", // refs
        " %s ",                                                // commit subject (message)
        "%C(240)(%C(246)\u{2}",
        "%ar", // relative author time
    ),
];

/// Gets the base `git log` command.
pub fn git_log() -> Command {
    let mut git = Command::new("git");
    git.args(GIT_LOG_ARGS);
    git
}

/// Gets the `less` command. The `-R` flag to support color in the
/// output it scrolls. The `-F` flag tells `less` to quit if the
/// content is less than that of one screen.
pub fn less() -> Command {
    let mut less = Command::new("less");
    less.arg("-RF");
    #[cfg(feature = "less_cmd")]
    less.arg("--cmd=/smash\nkkkkkkkkkk");
    less.stdin(Stdio::piped());
    less
}
