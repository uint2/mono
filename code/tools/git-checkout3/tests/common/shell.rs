use super::Test;

use std::fs;
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Output, Stdio},
};

#[derive(Debug)]
pub struct Output2 {
    pub stdout: String,
    pub stderr: String,
    pub status: ExitStatus,
}

pub fn commit_file<P: AsRef<Path>>(t: &Test, pathspec: P) {
    let pathspec = pathspec.as_ref();
    let _ = fs::create_dir_all(pathspec.parent().unwrap());
    let mut f = File::options().create(true).append(true).open(pathspec).unwrap();
    writeln!(f, "data({})", t.id()).unwrap();
    git!("add", pathspec).snw();
    git!("commit", "-m", format!("Updated \"{}\"", pathspec.display())).snw();
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

    /// Spawn and wait.
    fn snw(&mut self) {
        let output = self.get();
        if !output.stderr.trim().is_empty() {
            println!("stderr: {}", output.stderr.trim());
        }
        if !output.stdout.trim().is_empty() {
            println!("stdout: {}", output.stdout.trim());
        }
    }

    fn get_stdout(&mut self) -> String {
        let output = self.stdout(Stdio::piped()).output().unwrap();
        let stdout = core::str::from_utf8(&output.stdout).unwrap();
        stdout.trim().to_string()
    }

    fn get(&mut self) -> Output2 {
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
