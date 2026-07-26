mod makefile;
mod trim_whitespaces;
mod workflow;

fn main() {
    workflow::main();
    trim_whitespaces::main();
    makefile::main();
}
