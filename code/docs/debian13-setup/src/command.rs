use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Cursor, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;

use core::fmt;

macro_rules! cmd {
    ($bin:expr, $($arg:expr),* $(,)?) => {{
        let mut cmd = $crate::command::Command2::new($bin);
        cmd$(.arg($arg))*;
        cmd
    }};
    ($bin:expr) => {
         $crate::command::Command2::new($bin)
    };
}

/// Create a Command2 and run it right away.
macro_rules! sh {
    ($bin:expr, $($arg:expr),* $(,)?) => {{
        let mut cmd = $crate::command::Command2::new($bin);
        cmd$(.arg($arg))*;
        let _ = cmd.run();
    }};
}

impl fmt::Display for Command2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "$ {}", self.get_program())?;
        // Maximum length of an argument even after truncation.
        const N: usize = 20;
        for arg in self.command.get_args() {
            let Some(arg) = arg.to_str() else { continue };
            if arg.len() > N {
                write!(f, " {:?}...", &arg[..N - 3 - 2])
            } else {
                write!(f, " {arg}")
            }?;
        }
        Ok(())
    }
}

/// A special command for this setup system that tees the stdout and stderr to
/// potential target files.
pub struct Command2 {
    command: Command,
    /// The file to send STDOUT to, on top of the current tty.
    stdout: Option<PathBuf>,
    /// The file to send STDERR to, on top of the current tty.
    stderr: Option<PathBuf>,
    collect_stdout: bool,
    collect_stderr: bool,
}

#[derive(Debug)]
pub struct Output2 {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub status: ExitStatus,
}

/// Initialization code.
impl Command2 {
    pub fn new(program: &str) -> Self {
        let mut command = Command::new(program);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        Self {
            command,
            stdout: None,
            stderr: None,
            collect_stdout: false,
            collect_stderr: false,
        }
    }

    fn get_program(&self) -> &str {
        // Unwrap safety is guaranteed by the fact that at construction, we
        // used a `&str` type to initialize `self.command`.
        self.command.get_program().to_str().unwrap()
    }
}

/// Copy std::process::Command API.
impl Command2 {
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.command.arg(arg.as_ref());
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.command.args(args);
        self
    }

    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.command.current_dir(dir);
        self
    }

    pub fn env<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.command.env(key, value);
        self
    }

    // self.command.env();
}

/// Custom API.
impl Command2 {
    pub fn stdout<P: AsRef<Path>>(&mut self, filepath: P) -> &mut Self {
        self.stdout = Some(filepath.as_ref().to_path_buf());
        self
    }

    pub fn stderr<P: AsRef<Path>>(&mut self, filepath: P) -> &mut Self {
        self.stderr = Some(filepath.as_ref().to_path_buf());
        self
    }

    pub fn collect_stdout(&mut self) -> &mut Self {
        self.collect_stdout = true;
        self
    }

    pub fn collect_stderr(&mut self) -> &mut Self {
        self.collect_stderr = true;
        self
    }

    /// Spawn and then wait for termination.
    pub fn run(&mut self) -> io::Result<Output2> {
        println!("\x1b[33m{self}\x1b[m");

        let binary_name = self.get_program().to_string();
        let mut child = self.command.spawn()?;
        // Collected values.
        let mut c_stdout = self.collect_stdout.then(Vec::<u8>::new);
        let mut c_stderr = self.collect_stderr.then(Vec::<u8>::new);
        thread::scope(|scope| {
            let mut w_stdout = self.stdout.take().map(|filepath| {
                let file = File::create(filepath).unwrap();
                BufWriter::new(file)
            });
            let mut w_stderr = self.stderr.take().map(|filepath| {
                let file = File::create(filepath).unwrap();
                BufWriter::new(file)
            });
            let mut c_stdout = c_stdout.as_mut().map(Cursor::new);
            let mut c_stderr = c_stderr.as_mut().map(Cursor::new);
            let r_stdout = BufReader::new(child.stdout.take().unwrap());
            let r_stderr = BufReader::new(child.stderr.take().unwrap());

            let _h_stdout = scope.spawn(move || {
                for line in r_stdout.lines() {
                    let Ok(line) = line else { continue };
                    println!("stdout: {line}");
                    if let Some(w) = w_stdout.as_mut() {
                        writeln!(w, "{line}").unwrap();
                    }
                    if let Some(w) = c_stdout.as_mut() {
                        writeln!(w, "{line}").unwrap();
                    }
                }
            });
            let _h_stderr = scope.spawn(move || {
                for line in r_stderr.lines() {
                    let Ok(line) = line else { continue };
                    println!("stderr: {line}");
                    if let Some(w) = w_stderr.as_mut() {
                        writeln!(w, "{line}").unwrap();
                    }
                    if let Some(w) = c_stderr.as_mut() {
                        writeln!(w, "{line}").unwrap();
                    }
                }
            });
        });
        let Ok(status) = child.wait() else {
            panic!("Failed to wait for child: {binary_name}");
        };

        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Command returned a non-zero exit code.",
            ));
        }

        Ok(Output2 {
            stdout: c_stdout.and_then(|v| String::from_utf8(v).ok()),
            stderr: c_stderr.and_then(|v| String::from_utf8(v).ok()),
            status,
        })
    }
}
