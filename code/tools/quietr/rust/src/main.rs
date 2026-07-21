use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, ExitStatus};
use std::time;

fn fingerprint() -> u64 {
    let mut s = DefaultHasher::new();
    time::UNIX_EPOCH.elapsed().unwrap().hash(&mut s);
    return s.finish();
}

fn build_bypassed_command() -> Option<Command> {
    let mut args = std::env::args_os();
    args.next();
    let mut cmd = Command::new(args.next()?);
    cmd.args(args);
    Some(cmd)
}

const STDOUT_FILENAME: &str = "stdout.txt";
const STDERR_FILENAME: &str = "stderr.txt";

fn timestamp_to_log_dir(now: jiff::Zoned) -> PathBuf {
    Path::new(".cache")
        .join(now.strftime("%Y-%m-%d").to_string())
        .join(&format!("{:x}", fingerprint())[0..7])
}

fn try_main() -> Result<(ExitStatus, String), ()> {
    let Some(mut cmd) = build_bypassed_command() else {
        return Err(println!("[quietr] Nothing happend - no args passed."));
    };

    // let mut cmd = Command::new("sh");
    // cmd.arg("-c").arg(std::env::args().skip(1).collect::<Vec<_>>().join(" "));

    let Some(home_dir) = std::env::home_dir() else {
        return Err(println!("[quietr] Error: home directory not found."));
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
    if let Err(_) = std::fs::create_dir_all(&abs_log_dir) {
        return Err(println!("Failed to create log dir: {pretty_log_dir:?}"));
    }

    let fp_stdout = abs_log_dir.join(STDOUT_FILENAME);
    let fpp_stdout = pretty_log_dir.join(STDOUT_FILENAME);
    let fp_stderr = abs_log_dir.join(STDERR_FILENAME);
    let fpp_stderr = pretty_log_dir.join(STDERR_FILENAME);

    match File::create_new(&fp_stdout) {
        Ok(v) => cmd.stdout(v),
        _ => return Err(println!("Failed to create stdout file: {fpp_stdout:?}")),
    };
    match File::create_new(&fp_stderr) {
        Ok(v) => cmd.stderr(v),
        _ => return Err(println!("Failed to create stderr file: {fpp_stderr:?}")),
    };

    let pretty_args = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    println!("\x1b[37m$ \x1b[36m{pretty_args}\x1b[m");
    println!("  stdout> \x1b[33m{}\x1b[m", fpp_stdout.display());
    println!("  stderr> \x1b[33m{}\x1b[m", fpp_stderr.display());

    let Ok(mut child) = cmd.spawn() else {
        return Err(println!("Failed to spawn: {pretty_args}"));
    };
    let Ok(result) = child.wait() else {
        return Err(println!("Failed at wait: {pretty_args}"));
    };

    Ok((result, pretty_args))
}

fn main() -> ExitCode {
    let t = time::Instant::now();
    let Ok((status, label)) = try_main() else { return ExitCode::FAILURE };

    let badge = match status.success() {
        true => "\x1b[32m[OK]\x1b[m",
        false => "\x1b[31m[FAIL]\x1b[m",
    };

    let elapsed = t.elapsed().as_secs_f32();
    println!("{badge} ({elapsed:.2}s) {label}");

    if status.success() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
