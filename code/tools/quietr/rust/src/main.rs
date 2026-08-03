use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, ExitStatus};
use std::time::{Duration, Instant};
use std::{fs, io, time};

use core::fmt;

const APP_NAME: &str = "quietr";
const STDOUT_FILENAME: &str = "stdout.txt";
const STDERR_FILENAME: &str = "stderr.txt";

struct App {
    show_progress: bool,
    cmd: Command,
    pretty_args: String,
}

fn fingerprint() -> u64 {
    let mut s = DefaultHasher::new();
    time::UNIX_EPOCH.elapsed().unwrap().hash(&mut s);
    return s.finish();
}

fn build_bypassed_command() -> Option<App> {
    let args = std::env::args().collect::<Vec<_>>();
    let mut args = &args[1..]; // Skip this binary name.

    let mut show_progress = false;
    match args.get(0).map(|v| v.as_str())? {
        "-p" => {
            show_progress = true;
            args = &args[1..];
        }
        _ => {}
    }

    let mut cmd = Command::new(args.first()?);
    cmd.args(&args[1..]);
    let pretty_args = args.iter().map(|v| v.as_str()).collect::<Vec<_>>().join(" ");
    Some(App { cmd, show_progress, pretty_args })
}

fn timestamp_to_log_dir(now: jiff::Zoned) -> PathBuf {
    Path::new(".cache")
        .join(APP_NAME)
        .join(now.strftime("%Y-%m-%d").to_string())
        .join(&format!("{:x}", fingerprint())[0..7])
}

struct Duration2(Duration);

impl fmt::Display for Duration2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total = self.0.as_secs();

        let h = total / 3600;
        let m = (total % 3600) / 60;
        let s = total % 60;

        if h > 0 {
            write!(f, "{h}h {m}m")
        } else if m > 0 {
            write!(f, "{m}m {s}s")
        } else {
            write!(f, "{:.2}s", self.0.as_secs_f32())
        }
    }
}

fn try_main() -> Result<(ExitStatus, String), ()> {
    let Some(mut app) = build_bypassed_command() else {
        return Err(println!("[{APP_NAME}] Nothing happend - no args passed."));
    };

    // let mut cmd = Command::new("sh");
    // cmd.arg("-c").arg(std::env::args().skip(1).collect::<Vec<_>>().join(" "));

    let Some(home_dir) = std::env::home_dir() else {
        return Err(println!("[{APP_NAME}] Error: home directory not found."));
    };

    let mut log_dir: PathBuf;
    let mut abs_log_dir: PathBuf;
    loop {
        log_dir = timestamp_to_log_dir(jiff::Zoned::now());
        abs_log_dir = home_dir.join(&log_dir);
        if !abs_log_dir.exists() {
            break;
        }
    }
    let pretty_log_dir = Path::new("~").join(&log_dir);
    if let Err(_) = fs::create_dir_all(&abs_log_dir) {
        return Err(println!("Failed to create log dir: {pretty_log_dir:?}"));
    }

    let fp_stdout = abs_log_dir.join(STDOUT_FILENAME);
    let fpp_stdout = pretty_log_dir.join(STDOUT_FILENAME);
    let fp_stderr = abs_log_dir.join(STDERR_FILENAME);
    let fpp_stderr = pretty_log_dir.join(STDERR_FILENAME);

    match File::create_new(&fp_stdout) {
        Ok(v) => app.cmd.stdout(v),
        _ => return Err(println!("Failed to create stdout file: {fpp_stdout:?}")),
    };
    match File::create_new(&fp_stderr) {
        Ok(v) => app.cmd.stderr(v),
        _ => return Err(println!("Failed to create stderr file: {fpp_stderr:?}")),
    };

    println!("\x1b[37m$ \x1b[36m{}\x1b[m", app.pretty_args);
    println!("  stdout> \x1b[33m{}\x1b[m", fpp_stdout.display());
    println!("  stderr> \x1b[33m{}\x1b[m", fpp_stderr.display());

    let Ok(mut child) = app.cmd.spawn() else {
        return Err(println!("Failed to spawn: {}", app.pretty_args));
    };

    let exit_status = if app.show_progress {
        let start_t = Instant::now();
        const INTERVAL: Duration = Duration::from_millis(365);
        print!("Elapsed: ...");
        _ = io::stdout().flush();
        let thread = std::thread::spawn(move || child.wait());
        while !thread.is_finished() {
            std::thread::sleep(INTERVAL);
            print!("\x1b[2K\r");
            let elapsed = Duration2(start_t.elapsed());
            print!("Elapsed: {elapsed}");
            _ = io::stdout().flush();
        }
        print!("\x1b[2K\r");
        thread.join().unwrap()
    } else {
        child.wait()
    };
    let Ok(exit_status) = exit_status else {
        return Err(println!("Failed at wait: {}", app.pretty_args));
    };
    Ok((exit_status, app.pretty_args))
}

fn main() -> ExitCode {
    let t = time::Instant::now();
    let Ok((status, label)) = try_main() else { return ExitCode::FAILURE };

    let badge = match status.success() {
        true => "\x1b[32m[OK]\x1b[m",
        false => "\x1b[31m[FAIL]\x1b[m",
    };

    let elapsed = Duration2(t.elapsed());
    println!("{badge} ({elapsed}) {label}");

    if status.success() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
