#![allow(unused)]

use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output, Stdio};

fn cmd_to_vec(cmd: &Command) -> Vec<String> {
    fn s(s: &std::ffi::OsStr) -> String {
        s.to_str().unwrap().to_string()
    }
    let mut vec = vec![s(cmd.get_program())];
    vec.extend(cmd.get_args().map(s));
    vec
}

fn cwd(cmd: &Command) -> Option<&Path> {
    let cwd = cmd.get_current_dir()?;
    Some(cwd.strip_prefix(env!("CARGO_TARGET_TMPDIR")).unwrap_or(cwd))
}

fn report(cmd: &Command) {
    let cli_args = cmd_to_vec(cmd);
    // println!("[running] \x1b[33m{}\x1b[m @ \x1b[37m{:?}\x1b[m", cli_args.join(" "), cwd(cmd));
}

#[derive(Debug)]
pub struct Output2 {
    pub stdout: String,
    pub stderr: String,
    pub status: ExitStatus,
}

pub trait CommandExt {
    fn run(&mut self) -> Output;
    fn get_stdout(&mut self) -> String;
    fn get(&mut self) -> Output2;
    /// Spawn and wait.
    fn snw(&mut self);
}

impl CommandExt for Command {
    fn run(&mut self) -> Output {
        self.output().unwrap()
    }

    fn snw(&mut self) {
        let output = self.get();
        if !output.stderr.trim().is_empty() {
            tracing::info!("[stderr]\n{}", output.stderr.trim());
        }
        if !output.stdout.trim().is_empty() {
            tracing::info!("[stdout]\n{}", output.stdout.trim());
        }
    }

    fn get_stdout(&mut self) -> String {
        report(self);
        let output = self.stdout(Stdio::piped()).output().unwrap();
        let stdout = core::str::from_utf8(&output.stdout).unwrap();
        stdout.trim().to_string()
    }

    fn get(&mut self) -> Output2 {
        report(self);
        let output = self.stdout(Stdio::piped()).stderr(Stdio::piped()).output().unwrap();
        let stdout = core::str::from_utf8(&output.stdout).unwrap();
        let stderr = core::str::from_utf8(&output.stderr).unwrap();
        Output2 {
            stdout: stdout.trim().to_string(),
            stderr: stderr.trim().to_string(),
            status: output.status,
        }
    }
}

pub trait OutputExt {
    fn stdout(&self) -> &str;
    fn stderr(&self) -> &str;
}

impl OutputExt for Output {
    fn stdout(&self) -> &str {
        core::str::from_utf8(&self.stdout).unwrap()
    }
    fn stderr(&self) -> &str {
        core::str::from_utf8(&self.stderr).unwrap()
    }
}
