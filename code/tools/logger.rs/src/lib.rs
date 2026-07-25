/*!
Defines a super simple logger that works with the `log` crate.
Pulled from the ripgrep project. Alomst everything below is a direct requote of
their code/comments.

We don't do anything fancy. We just need basic log levels and the ability to
print to stderr. We therefore avoid bringing in extra dependencies just for
this functionality.
*/

use ext_log as log;

pub use log::{Level, LevelFilter};

// Re-export the logging functions.
pub use log::{debug, error, info, trace, warn};

/// Like eprintln, but locks stdout to prevent interleaving lines.
///
/// This locks stdout, not stderr, even though this prints to stderr. This
/// avoids the appearance of interleaving output when stdout and stderr both
/// correspond to a tty.
macro_rules! eprintln_locked {
    ($($tt:tt)*) => {{
        {
            use std::io::Write;

            // This is a bit of an abstraction violation because we explicitly
            // lock stdout before printing to stderr. This avoids interleaving
            // lines within ripgrep because `search_parallel` uses `termcolor`,
            // which accesses the same stdout lock when writing lines.
            let stdout = std::io::stdout().lock();
            let mut stderr = std::io::stderr().lock();
            // We specifically ignore any errors here. One plausible error we
            // can get in some cases is a broken pipe error. And when that
            // occurs, we should exit gracefully. Otherwise, just abort with
            // an error code because there isn't much else we can do.
            //
            // See: https://github.com/BurntSushi/ripgrep/issues/1966
            if let Err(err) = writeln!(stderr, $($tt)*) {
                if err.kind() == std::io::ErrorKind::BrokenPipe {
                    std::process::exit(0);
                } else {
                    std::process::exit(2);
                }
            }
            drop(stdout);
        }
    }}
}

/// The simplest possible logger that logs to stderr.
///
/// This logger does no filtering. Instead, it relies on the `log` crates
/// filtering via its global max_level setting.
#[derive(Debug)]
pub(crate) struct Logger(());

/// A singleton used as the target for an implementation of the `Log` trait.
const LOGGER: &'static Logger = &Logger(());

impl Logger {
    /// Create a new logger that logs to stderr and initialize it as the
    /// global logger. If there was a problem setting the logger, then an
    /// error is returned.
    pub(crate) fn init() -> Result<(), log::SetLoggerError> {
        log::set_logger(LOGGER)
    }
}

static mut IS_TERMINAL: Option<bool> = None;

fn is_terminal() -> bool {
    match unsafe { IS_TERMINAL } {
        Some(true) => true,
        _ => false,
    }
}

fn color(level: Level) -> &'static str {
    if let None = unsafe { IS_TERMINAL } {
        use std::io::IsTerminal;
        let is_terminal = std::io::stderr().is_terminal();
        unsafe { IS_TERMINAL = Some(is_terminal) };
    }
    return if is_terminal() {
        match level {
            Level::Trace => "\x1b[34m",
            Level::Debug => "\x1b[35m",
            Level::Info => "\x1b[32m",
            Level::Warn => "\x1b[33m",
            Level::Error => "\x1b[31m",
        }
    } else {
        ""
    };
}

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        // We set the log level via log::set_max_level, so we don't need to
        // implement filtering here.
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        let target = record.target();
        let level = record.level();
        let color = color(level);
        let args = record.args();

        let tcolor0 = if is_terminal() { "\x1b[37m" } else { "" };
        let reset = if is_terminal() { "\x1b[m" } else { "" };
        let colon = if is_terminal() { "\x1b[37m:\x1b[m" } else { ":" };

        match (record.file(), record.line()) {
            (Some(file), Some(line)) => {
                eprintln_locked!(
                    "{color}{level} {tcolor0}{target}{reset} {}:{}{colon} {args}",
                    file,
                    line
                );
            }
            (Some(file), None) => {
                eprintln_locked!(
                    "{color}{level} {tcolor0}{target}{reset} {}{colon} {args}",
                    file
                );
            }
            _ => eprintln_locked!("{color}{level} {tcolor0}{target}{colon} {args}"),
        }
    }

    fn flush(&self) {
        // We use eprintln_locked! which is flushed on every call.
    }
}

pub fn init(level_filter: Option<LevelFilter>) {
    if let Some(level_filter) = level_filter {
        log::set_max_level(level_filter);
    }
    Logger::init().expect("Unable to initialize logger");
}
