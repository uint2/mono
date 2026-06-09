use std::fs::File;
use std::io::Write;

const ROW_ANCHORS: [u8; 9] = [0, 9, 18, 27, 36, 45, 54, 63, 72];
const COLUMN_ANCHORS: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const BOX_ANCHORS: [u8; 9] = [0, 3, 6, 27, 30, 33, 54, 57, 60];

macro_rules! writeln {
    ($dst:expr, $($arg:tt)*) => {
        std::writeln!($dst, $($arg)*).unwrap()
    }
}

fn related(
    loc: u8,
    row_groups: &Vec<Vec<u8>>,
    column_groups: &Vec<Vec<u8>>,
    box_groups: &Vec<Vec<u8>>,
) -> Vec<u8> {
    let mut related = Vec::with_capacity(20);
    for &i in row_groups.iter().find(|v| v.contains(&loc)).unwrap() {
        if i != loc {
            related.push(i);
        }
    }
    for &i in column_groups.iter().find(|v| v.contains(&loc)).unwrap() {
        if i != loc {
            related.push(i);
        }
    }
    for &i in box_groups.iter().find(|v| v.contains(&loc)).unwrap() {
        if i != loc {
            related.push(i);
        }
    }
    related.sort();
    related.dedup();
    assert_eq!(related.len(), 20);
    related
}

const OUTPUT_FILE: &str = "src/generated.rs";

fn main() {
    println!("cargo::rerun-if-changed={OUTPUT_FILE}");
    let mut f = File::create(OUTPUT_FILE).unwrap();

    // let row_groups =
    // ROW_ANCHORS.iter().map(|&a| (a..a + 9).collect::<Vec<_>>());
    let row_groups =
        ROW_ANCHORS.iter().map(|&a| (a..a + 9).collect::<Vec<_>>()).collect::<Vec<_>>();

    let column_groups = COLUMN_ANCHORS
        .iter()
        .map(|&a| (a..a + 81).step_by(9).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let box_groups = BOX_ANCHORS
        .iter()
        .map(|&a| vec![a, a + 1, a + 2, a + 9, a + 10, a + 11, a + 18, a + 19, a + 20])
        .collect::<Vec<_>>();

    let mut all_groups = vec![];
    all_groups.extend(&row_groups);
    all_groups.extend(&column_groups);
    all_groups.extend(&box_groups);

    writeln!(f, "#[allow(unused)]");
    writeln!(f, "pub const ROW_GROUPS: [[u8; 9]; 9] = {row_groups:?};");

    writeln!(f, "#[allow(unused)]");
    writeln!(f, "pub const COLUMN_GROUPS: [[u8; 9]; 9] = {column_groups:?};");

    writeln!(f, "#[allow(unused)]");
    writeln!(f, "pub const BOX_GROUPS: [[u8; 9]; 9] = {box_groups:?};");

    writeln!(f, "/// All the constraint groups.");
    writeln!(f, "#[allow(unused)]");
    writeln!(f, "pub const ALL_GROUPS: [[u8; 9]; 27] = {all_groups:?};");

    writeln!(f, "/// The locations that correspond to the first element of each row.");
    writeln!(f, "#[allow(unused)]");
    writeln!(f, "pub const ROW_ANCHORS: [u8; 9] = {ROW_ANCHORS:?};");

    writeln!(f, "/// The locations that correspond to the first element of each column.");
    writeln!(f, "#[allow(unused)]");
    writeln!(f, "pub const COLUMN_ANCHORS: [u8; 9] = {COLUMN_ANCHORS:?};");

    writeln!(f, "/// The locations that correspond to the top-left element of each box.");
    writeln!(f, "#[allow(unused)]");
    writeln!(f, "pub const BOX_ANCHORS: [u8; 9] = {BOX_ANCHORS:?};");

    writeln!(f, "/// Locations relating to each location.");
    writeln!(f, "#[allow(unused)]");
    writeln!(f, "pub const fn related(v: u8) -> [u8; 20] {{");
    writeln!(f, "    match v {{");
    for i in 0..81 {
        writeln!(
            f,
            "    {i} => {:?},",
            related(i, &row_groups, &column_groups, &box_groups)
        );
    }
    writeln!(f, "        _ => panic!(\"Invalid location.\"),");
    writeln!(f, "    }}");
    writeln!(f, "}}");
}
