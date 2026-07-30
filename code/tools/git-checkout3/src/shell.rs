use std::process::{Command, ExitStatus};

pub struct ExitCode(i32);

impl ExitCode {
    pub const SUCCESS: Self = Self(0);
    pub const FAILURE: Self = Self(1);
    pub const ACCEPT: Self = Self(64);

    pub fn exit(&self) -> ! {
        std::process::exit(self.0);
    }

    pub fn of(value: ExitStatus) -> Self {
        Self(value.code().unwrap_or(1))
    }

    // pub const fn new(value: i32) -> Self { Self(value) }
}

#[cfg(unix)]
pub fn run(mut cmd: Command) -> ! {
    use std::os::unix::process::CommandExt;
    let err = cmd.exec();
    eprintln!("Failed execvp call: {err}");
    ExitCode::FAILURE.exit();
}

#[cfg(not(unix))]
pub fn run(mut cmd: Command) -> ! {
    ExitCode::of(cmd.spawn().unwrap().wait().unwrap()).exit();
}
