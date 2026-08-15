use std::fs::DirEntry;
use std::io::Write;
use std::path::Path;
use std::process::{Command, ExitCode};

use core::str;

macro_rules! banner {
    () => {
        "(\x1b[35mt\x1b[m)"
    };
    ($value:expr) => {
        concat!("(\x1b[35mt\x1b[m) \x1b[37m", $value, "\x1b[m\n")
    };
}

#[allow(unused)]
enum Trigger {
    /// Matches this file exactly.
    File(&'static str),
    /// Runs a predicate.
    Pred(fn(&Path, &[DirEntry]) -> bool),
    /// Checks for git remote.
    GitRemote(&'static str),
    Or(&'static [Self]),
    And(&'static [Self]),
}
#[allow(unused)]
use Trigger::{self as Tr, And, Or};

impl Trigger {
    fn hit(&self, cwd: &Path, files: &[DirEntry]) -> bool {
        match self {
            Self::File(path) => files.iter().any(|f| f.path().ends_with(path)),
            Self::Pred(f) => f(cwd, files),
            Self::GitRemote(remote) => {
                let output = Command::new("git")
                    .args(["config", "get", "remote.origin.url"])
                    .output();
                let Ok(output) = output else { return false };
                let Ok(output) = str::from_utf8(&output.stdout) else { return false };
                output.trim() == *remote
            }
            Self::Or(triggers) => triggers.iter().any(|v| v.hit(cwd, files)),
            Self::And(triggers) => triggers.iter().all(|v| v.hit(cwd, files)),
        }
    }
}

struct Matcher {
    trigger: Trigger,
    args: &'static [&'static str],
    message: Option<&'static str>,
}

const WORK_MATCHES: &'static [Matcher] = &[];

const MATCHERS: &'static [Matcher] = &[
    Matcher {
        trigger: And(&[
            Tr::GitRemote("https://github.com/neovim/neovim.git"),
            Tr::File(".git"),
        ]),
        args: &["echo", "this is neovim"],
        message: None,
    },
    Matcher {
        trigger: Tr::File("Makefile"),
        args: &["make", "--no-print-directory"],
        message: Some("Makefile"),
    },
    Matcher {
        trigger: Tr::File("Cargo.toml"),
        args: &["cargo", "run"],
        message: Some("cargo (Cargo.toml)"),
    },
    Matcher {
        trigger: Tr::File("package.json"),
        args: &["npm", "run"],
        message: Some("npm run (package.json)"),
    },
    Matcher {
        trigger: Tr::File("build.sh"),
        args: &["bash", "build.sh"],
        message: Some("bash (build.sh)"),
    },
    Matcher {
        trigger: Tr::File("run.py"),
        args: &["python3", "run.py"],
        message: Some("python3 (run.py)"),
    },
];

impl Matcher {
    fn run(&self) -> ExitCode {
        use std::os::unix::process::CommandExt;

        let mut cmd = Command::new(self.args[0]);
        cmd.args(&self.args[1..]).args(std::env::args_os().skip(1));
        let err = cmd.exec();
        println!("Error during `execvp` call: {err}");
        ExitCode::FAILURE
    }
}

fn try_run(cwd: &Path) -> Option<ExitCode> {
    let Ok(files) = cwd.read_dir() else {
        println!("Unable to list files at {:?}", cwd);
        return Some(ExitCode::FAILURE);
    };
    let files: Vec<_> = files.filter_map(|v| v.ok()).collect();

    let mut matchers = MATCHERS.iter().chain(WORK_MATCHES);

    let Some(m) = matchers.find(|v| v.trigger.hit(cwd, &files)) else {
        return None;
    };
    if let Some(message) = m.message {
        println!("{} \x1b[37m{message}\x1b[m", banner!());
    }
    Some(m.run())
}

struct App<'app> {
    cwd: &'app Path,
    /// Only check to see which command gets triggered, but don't run it.
    check_only: bool,
}

impl App<'_> {
    pub fn try_run(&self) -> Option<ExitCode> {
        let Ok(files) = self.cwd.read_dir() else {
            println!("Unable to list files at {:?}", self.cwd);
            return Some(ExitCode::FAILURE);
        };
        let files: Vec<_> = files.filter_map(|v| v.ok()).collect();

        let mut matchers = MATCHERS.iter().chain(WORK_MATCHES);

        let Some(m) = matchers.find(|v| v.trigger.hit(self.cwd, &files)) else {
            return None;
        };
        if let Some(message) = m.message {
            println!("{} \x1b[37m{message}\x1b[m", banner!());
        }
        if self.check_only {
            return Some(ExitCode::SUCCESS);
        }
        Some(m.run())
    }

    pub fn next(mut self) -> Option<Self> {
        self.cwd = self.cwd.parent()?;

        // Still ensure that there is a parent, as we don't want to run this in
        // the root directory.
        self.cwd.parent()?;

        std::env::set_current_dir(self.cwd).ok()?;

        Some(self)
    }
}

fn main() -> ExitCode {
    let cwd = std::env::current_dir().expect("Unable to get current working directory");
    let args = std::env::args().collect::<Vec<_>>();
    let mut app =
        App { cwd: cwd.as_path(), check_only: args.iter().any(|arg| arg == "--check") };

    if let Some(exit_code) = app.try_run() {
        return exit_code;
    }
    std::io::stdout().write(banner!("traversing upwards...").as_bytes()).unwrap();
    loop {
        app = match app.next() {
            Some(v) => v,
            None => break,
        };
        println!("{} \x1b[37m{}\x1b[m", banner!(), cwd.display());
        if let Some(exit_code) = try_run(&cwd) {
            return exit_code;
        }
    }
    std::io::stdout().write(banner!("nothing to do.").as_bytes()).unwrap();
    ExitCode::SUCCESS
}
