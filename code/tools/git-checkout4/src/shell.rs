use crate::prelude::*;

#[cfg(unix)]
pub fn run(mut cmd: Command) -> ! {
    use std::os::unix::process::CommandExt;
    let err = cmd.exec();
    eprintln!("Failed execvp call: {err}");
    std::process::exit(1);
}

#[cfg(not(unix))]
pub fn run(mut cmd: Command) -> ! {
    let x = cmd.spawn().unwrap().wait().unwrap();
    std::process::exit(x.code().unwrap_or(1));
}
