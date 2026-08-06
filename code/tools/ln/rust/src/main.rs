mod cmd;

use core::sync::atomic::{AtomicBool, Ordering};

use std::io;
use std::io::{BufRead, BufReader, BufWriter, LineWriter, Write, stdout};
use std::process::{ChildStdout, Command, Stdio};

const HEIGHT_RATIO: f32 = 0.7;

static RUNNING: AtomicBool = AtomicBool::new(true);

macro_rules! _write { ($f:expr, $($x:tt)+) => {{ let _ = std::write!($f, $($x)*); }}}

// Gets the upper bound on number of lines to print on a bounded run.
fn get_line_limit() -> (u16, u16) {
    let (width, height) = crossterm::terminal::size().unwrap();
    (width, (height as f32 * HEIGHT_RATIO) as u16)
}

fn get_time(timestamp: &str) -> (&str, &str) {
    let (n, u) = timestamp.split_once(' ').unwrap();
    (n, if u.starts_with("mo") { "M" } else { &u[0..1] })
}

struct LogLineWriter<W: Write> {
    bw: BufWriter<W>,
    written: usize,
}

impl<W: Write> LogLineWriter<W> {
    pub fn write(&mut self, value: &str) -> io::Result<()> {
        self.written += value.chars().count();
        self.bw.write(value.as_bytes()).map(|_| ())
    }

    pub fn lines_consumed(&self, width: u16) -> u16 {
        (self.written as u16).div_ceil(width)
    }

    pub fn write_line(&mut self, text: &str, timestamp: &str) -> io::Result<()> {
        let i_paren = text.rfind('(').unwrap();
        let light_gray = &text[i_paren + 1..];
        let i_space = text[..i_paren].rfind(' ').unwrap();
        let dark_gray = &text[i_space + 1..i_paren];
        let (timestamp, unit) = get_time(timestamp);

        self.write(text)?;
        self.write(light_gray)?;
        self.write(timestamp)?;
        self.write(unit)?;
        self.write(dark_gray)?;
        self.write(")\x1b[m\n")
    }
}

/// Iterates over the git log and writes the outputs to `f`.
fn run<W: Write>(is_bounded: bool, log: ChildStdout, target: W) {
    let mut buffer = String::with_capacity(256);
    let (width, height) = get_line_limit();
    let mut limit = is_bounded.then_some(height);

    let mut log = BufReader::new(log);
    let mut writer = LogLineWriter { bw: BufWriter::new(target), written: 0 };

    while RUNNING.load(Ordering::Relaxed) {
        buffer.clear();
        writer.written = 0;
        let line = match log.read_line(&mut buffer) {
            Ok(0) | Err(_) => break,
            _ => buffer.as_str(),
        };
        let result = if let Some((text, timestamp)) = line.rsplit_once('\u{2}') {
            writer.write_line(text, timestamp)
        } else {
            // The entire line is just a git log --graph visual line.
            writer.write(line)
        };

        if let Err(err) = result {
            if let io::ErrorKind::BrokenPipe = err.kind() {
                break;
            }
        }

        // r := remaining lines to print.
        let Some(r) = limit else { continue };
        let Some(r) = r.checked_sub(writer.lines_consumed(width)) else { break };
        limit = Some(r);
    }
}

fn parse_cli() -> (Command, bool) {
    let mut git_log = cmd::git_log();
    git_log.stdout(Stdio::piped());

    let mut is_bounded = false;
    for arg in std::env::args_os().skip(1) {
        if arg == "--bound" {
            is_bounded = true;
            continue;
        }
        git_log.arg(arg);
    }

    (git_log, is_bounded)
}

#[cfg(not(windows))]
fn signal_handling() {
    use signal_hook::{consts::SIGINT, iterator::Signals};
    let mut signals = Signals::new([SIGINT]).unwrap();

    std::thread::spawn(move || {
        for _ in signals.forever() {
            RUNNING.fetch_and(false, Ordering::Relaxed);
        }
    });
}

/// Here, we operate under the assumption that we ARE using this in a
/// tty context, and hence always have color on.
fn main() {
    #[cfg(not(windows))]
    signal_handling();

    let (mut git_log, is_bounded) = parse_cli();

    let mut git_log_p = git_log.spawn().unwrap(); // process
    let git_log_stdout = git_log_p.stdout.take().unwrap(); // stdout

    match cmd::less().spawn() {
        Ok(mut less) => {
            // `less` found: pass the git log output to less.
            let less_stdin = less.stdin.take().unwrap();
            run(is_bounded, git_log_stdout, LineWriter::new(less_stdin));
            let _ = less.wait();
        }
        Err(_) => {
            // `less` not found: just run normal git log and print to stdout.
            run(is_bounded, git_log_stdout, LineWriter::new(stdout().lock()));
        }
    }
}
