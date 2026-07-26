use ignore::WalkBuilder;

use std::fs;
use std::io::Read;

fn contains_seq(line: &str, seq: &[&str]) -> bool {
    match seq.len() {
        0 => true,
        _ => {
            let Some((_, line)) = line.split_once(seq[0]) else { return false };
            contains_seq(line, &seq[1..])
        }
    }
}

pub fn main() {
    let walk = WalkBuilder::new(".").hidden(true).build();
    let mut buffer = String::new();

    for entry in walk {
        let Ok(entry) = entry else { continue };
        if entry.path().extension().map_or(false, |v| v == "rs") {
            continue; // Skip rust files.
        }
        let mut f = fs::File::open(entry.path()).unwrap();

        buffer.clear();
        let Ok(_n) = f.read_to_string(&mut buffer) else { continue };
        drop(f);

        for l in buffer.lines() {
            // MAKEFILE_PATH
            if contains_seq(l, &["MAKEFILE_", ":=", "lastword", "MAKEFILE_"]) {
                assert_eq!(
                    l,
                    "MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))",
                    "Bad makefile (MAKEFILE_PATH) at {:?}",
                    entry.path()
                )
            }
            // MAKEFILE_DIR
            else if contains_seq(l, &["MAKEFILE_", ":=", "dir", "MAKEFILE_"]) {
                assert_eq!(
                    l,
                    "MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))",
                    "Bad makefile (MAKEFILE_DIR) at {:?}",
                    entry.path()
                )
            }
        }
    }
}

/*
MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))
*/
