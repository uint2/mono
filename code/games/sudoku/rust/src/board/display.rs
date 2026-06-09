use super::*;

#[rustfmt::skip]
mod line {
// pub const TOP: &str = "\x1b[37m┌─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐\x1b[m";
// pub const MID: &str = "\x1b[37m├─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤\x1b[m";
// pub const BOT: &str = "\x1b[37m└─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘\x1b[m";
pub const TOP: &str = "\x1b[37m╓─────────┬─────────┬─────────╥─────────┬─────────┬─────────╥─────────┬─────────┬─────────╖\x1b[m";
pub const MID: &str = "\x1b[37m╟─────────┼─────────┼─────────╫─────────┼─────────┼─────────╫─────────┼─────────┼─────────╢\x1b[m";
pub const BOT: &str = "\x1b[37m╙─────────┴─────────┴─────────╨─────────┴─────────┴─────────╨─────────┴─────────┴─────────╜\x1b[m";
// pub const TOP: &str = "╔═════════╤═════════╤═════════╦═════════╤═════════╤═════════╦═════════╤═════════╤═════════╗";
// pub const MID: &str = "╠═════════╪═════════╪═════════╬═════════╪═════════╪═════════╬═════════╪═════════╪═════════╣";
// pub const BOT: &str = "╚═════════╧═════════╧═════════╩═════════╧═════════╧═════════╩═════════╧═════════╧═════════╝";
}

impl Board {
    pub fn count_ones(&self) -> u16 {
        self.cells.iter().map(Cell::count_ones).sum()
    }
}

const COMMA: &str = "\x1b[37m┊\x1b[m";
// const VERT: &str = "\x1b[37m│\x1b[m";
const VERT: &str = "\x1b[37m║\x1b[m";

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", line::TOP)?;
        for (j, row) in self.cells.chunks(9).enumerate() {
            if j != 0 && j % 3 == 0 {
                writeln!(f, "{}", line::MID)?;
            }
            write!(f, "{VERT}")?;
            for bx in row.chunks(3) {
                write!(f, "{}{COMMA}{}{COMMA}{}{VERT}", bx[0], bx[1], bx[2])?;
            }
            writeln!(f)?;
        }
        write!(f, "{} ({})", line::BOT, self.count_ones())
    }
}
