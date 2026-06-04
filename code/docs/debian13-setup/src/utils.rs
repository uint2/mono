use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

static mut HOME_DIR: Option<PathBuf> = None;

pub fn initialize_home_dir() {
    let home_dir = std::env::home_dir().unwrap();
    unsafe { HOME_DIR = Some(home_dir) };
}

/// This is safe because it is initialized once ever, at the start.
#[allow(static_mut_refs)]
pub fn home_dir() -> &'static Path {
    unsafe { HOME_DIR.as_ref().unwrap().as_path() }
}

/// Obtains the filesize using the `du` program.
pub fn get_file_size_with_du<P: AsRef<Path>>(filepath: P) -> usize {
    let out = cmd!("du", filepath.as_ref())
        .collect_stdout()
        .run()
        .expect("Failed to run `du`.");
    let size = out
        .stdout
        .as_ref()
        .and_then(|v| v.split_once('\t'))
        .map(|v| v.0)
        .and_then(|v| v.parse::<usize>().ok());
    size.expect("Failed to parse `du` stdout.")
}

/// Searches a directory for matches, and returns absolute paths.
pub fn search_dir<P, F>(dir: P, predicate: F) -> Vec<PathBuf>
where
    P: AsRef<Path>,
    F: FnMut(&DirEntry) -> bool,
{
    let dir = dir.as_ref();
    fs::read_dir(dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(predicate)
        .filter_map(|v| dir.join(v.path()).canonicalize().ok())
        .collect()
}

pub fn has_ext<P: AsRef<Path>>(path: P, ext: &str) -> bool {
    path.as_ref().extension().map_or(false, |v| v == ext)
}

pub fn curl<P: AsRef<Path>>(output: P, url: &str) {
    sh!(
        "curl",
        "--fail",
        "--location",
        "--silent",
        "--show-error",
        "--output",
        output.as_ref(),
        url
    );
}

/// Writes `contents` to the file at `filepath`.
pub fn echo<P>(contents: &str, filepath: P, sudo: bool)
where
    P: AsRef<Path>,
{
    let filepath = filepath.as_ref();
    let shell_cmd = format!("echo '{contents}' > {filepath:?}");
    match sudo {
        true => sh!("sudo", "sh", "-c", shell_cmd),
        false => sh!("sh", "-c", shell_cmd),
    }
}

pub fn sha256sum<P: AsRef<Path>>(filepath: P) -> String {
    let output =
        cmd!("sha256sum", filepath.as_ref()).collect_stdout().run().unwrap();
    let stdout = output.stdout.unwrap();
    stdout.split_once(' ').unwrap().0.to_string()
}
