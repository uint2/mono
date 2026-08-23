use ignore::WalkBuilder;

use std::fs;
use std::io::{Read, Write};

pub fn main() {
    let walk = WalkBuilder::new(".").hidden(true).build();
    let mut buffer = String::new();
    let mut lines = Vec::with_capacity(0x800);

    let mut dirty = false;

    for entry in walk {
        let Ok(entry) = entry else { continue };
        if entry.path_is_symlink() {
            continue;
        }
        let Ok(mut f) = fs::File::open(entry.path()) else {
            panic!("Failed to open {}", entry.path().display());
        };

        buffer.clear();
        let Ok(_n) = f.read_to_string(&mut buffer) else { continue };
        drop(f);
        if buffer.lines().all(|line| line.trim_end() == line) {
            continue; // All lines don't end with whitespace. Very good.
        }
        dirty = true;
        // Trim out the whitespaces.
        lines.clear();
        for line in buffer.lines() {
            lines.push(line.trim_end().to_string());
        }
        let mut f = fs::File::create(entry.path()).unwrap();
        write!(f, "{}", lines.join("\n")).unwrap();
        f.flush().unwrap();
    }

    if dirty {
        panic!("Found files that ended with whitespaces");
    }
}
