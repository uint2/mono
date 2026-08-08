use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Create a new branch named <new-branch>, start it at <start-point>,
    /// and check the resulting branch out;
    #[arg(short = 'b')]
    branch: String,

    /// Creates the branch <new-branch>, start it at <start-point>; if it
    /// already exists, then reset it to <start-point>. And then check the
    /// resulting branch out. This is equivalent to running "git branch" with
    /// "-f" followed by "git checkout" of that branch; see git-branch(1) for
    /// details.

    #[arg(short = 'B')]
    branch2: String,

    /// Number of times to greet
    start_point: u8,
}
