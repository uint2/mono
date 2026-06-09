use crate::prelude::*;

/// Represents the smallest unit of data in a Sudoku game. Each bit represents
/// if this cell can take on a particular value.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell(u16);

#[rustfmt::skip]
mod bits {
    pub const ONE:    u16 = 0b0000000000000001;
    pub const NINE:   u16 = 0b0000000100000000;
    pub const FRESH:  u16 = 0b0000000111111111;
}

pub const NINE_CELLS: [Cell; 9] = [
    Cell(bits::ONE),
    Cell(0b0000000000000010),
    Cell(0b0000000000000100),
    Cell(0b0000000000001000),
    Cell(0b0000000000010000),
    Cell(0b0000000000100000),
    Cell(0b0000000001000000),
    Cell(0b0000000010000000),
    Cell(bits::NINE),
];

impl Cell {
    pub const fn new() -> Self {
        Self(bits::FRESH)
    }

    /// Check if there is any value that's locked-in.
    #[inline(always)]
    pub const fn is_solved(&self) -> bool {
        self.0.count_ones() == 1
    }

    /// Eliminate candidates from `other`.
    #[inline(always)]
    pub fn eliminate(&mut self, other: Self) {
        self.0 &= !other.0
    }

    /// Retain candidates from `other`.
    #[inline(always)]
    pub fn retain(&mut self, other: Self) {
        self.0 &= other.0
    }

    #[inline(always)]
    pub fn has_no_candidates(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub fn intersect(&self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline(always)]
    pub fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Get the last one standing, if any.
    pub const fn unique(&self) -> Option<u16> {
        match self.0 {
            0b0000000000000001 => Some(1),
            0b0000000000000010 => Some(2),
            0b0000000000000100 => Some(3),
            0b0000000000001000 => Some(4),
            0b0000000000010000 => Some(5),
            0b0000000000100000 => Some(6),
            0b0000000001000000 => Some(7),
            0b0000000010000000 => Some(8),
            0b0000000100000000 => Some(9),
            _ => None,
        }
    }

    pub const fn from_u8(value: u8) -> Self {
        match value {
            1 => Cell(bits::ONE),
            2 => Cell(0b0000000000000010),
            3 => Cell(0b0000000000000100),
            4 => Cell(0b0000000000001000),
            5 => Cell(0b0000000000010000),
            6 => Cell(0b0000000000100000),
            7 => Cell(0b0000000001000000),
            8 => Cell(0b0000000010000000),
            9 => Cell(bits::NINE),
            _ => panic!(),
        }
    }

    pub const fn count_ones(&self) -> u16 {
        self.0.count_ones() as u16
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(v) = self.unique() {
            return write!(f, "    \x1b[32m{v}\x1b[m    ");
        }
        let mut v = self.0;
        let mut j = 1;
        while j <= 9 {
            if v & bits::ONE != 0 {
                write!(f, "\x1b[33m{j}\x1b[m")?;
            } else {
                write!(f, "\x1b[37m.\x1b[m")?;
            }
            v >>= 1;
            j += 1;
        }
        Ok(())
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
        // write!(f, "Cell({:0>9b})", self.0)
    }
}

impl BitOr for Cell {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
