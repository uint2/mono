use std::path::Path;

mod makefile;
mod trim_whitespaces;
mod workflow;

fn main() {
    assert!(
        Path::new(".git").is_dir(),
        "Please run the healthcheck from the project root."
    );
    workflow::main();
    trim_whitespaces::main();
    makefile::main();
}
