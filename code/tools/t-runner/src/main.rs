use std::fs::DirEntry;
use std::io::Write;
use std::path::Path;
use std::process::{Command, ExitCode};

macro_rules! banner {
    () => {
        "(\x1b[35mt\x1b[m)"
    };
    ($value:expr) => {
        concat!("(\x1b[35mt\x1b[m) \x1b[37m", $value, "\x1b[m\n")
    };
}

enum Trigger {
    /// Matches this file exactly.
    Path(&'static str),
    /// Runs a predicate.
    Pred(fn(&Path, &[DirEntry]) -> bool),
}

impl Trigger {
    fn hit(&self, cwd: &Path, files: &[DirEntry]) -> bool {
        match self {
            Self::Path(path) => files.iter().any(|f| f.path().ends_with(path)),
            Self::Pred(f) => f(cwd, files),
        }
    }
}

struct Matcher {
    trigger: Trigger,
    args: &'static [&'static str],
    message: Option<&'static str>,
}

const MATCHERS: &'static [Matcher] = &[
    Matcher {
        trigger: Trigger::Path("Makefile"),
        args: &["make", "--no-print-directory"],
        message: Some("Makefile"),
    },
    Matcher {
        trigger: Trigger::Path("Cargo.toml"),
        args: &["cargo", "run"],
        message: Some("cargo (Cargo.toml)"),
    },
    Matcher {
        trigger: Trigger::Path("package.json"),
        args: &["npm", "run"],
        message: Some("npm run (package.json)"),
    },
    Matcher {
        trigger: Trigger::Path("build.sh"),
        args: &["bash", "build.sh"],
        message: Some("bash (build.sh)"),
    },
    Matcher {
        trigger: Trigger::Path("run.py"),
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

    let Some(m) = MATCHERS.into_iter().find(|v| v.trigger.hit(cwd, &files)) else {
        return None;
    };
    if let Some(message) = m.message {
        println!("{} \x1b[37m{message}\x1b[m", banner!());
    }
    Some(m.run())
}

fn main() -> ExitCode {
    let Ok(mut cwd) = std::env::current_dir() else {
        println!("Unable to get current working directory");
        return ExitCode::FAILURE;
    };

    if let Some(exit_code) = try_run(&cwd) {
        return exit_code;
    }
    std::io::stdout().write(banner!("traversing upwards...").as_bytes()).unwrap();
    while cwd.pop() && cwd.parent().is_some() {
        let Ok(()) = std::env::set_current_dir(&cwd) else {
            println!("Unable to set current working directory");
            return ExitCode::FAILURE;
        };
        println!("{} \x1b[37m{}\x1b[m", banner!(), cwd.display());
        if let Some(exit_code) = try_run(&cwd) {
            return exit_code;
        }
    }
    std::io::stdout().write(banner!("nothing to do.").as_bytes()).unwrap();
    ExitCode::SUCCESS
}
