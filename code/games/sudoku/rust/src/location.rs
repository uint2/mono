//! u8 operations relating to location within a flattened Board.
#![allow(unused)]

include!("./generated.rs");

/// A location that does not exist in the sudoku board.
pub const INVALID: u8 = u8::MAX;

#[inline(always)]
pub const fn row(v: u8) -> u8 {
    v / 9
}

#[inline(always)]
pub const fn column(v: u8) -> u8 {
    v % 9
}

#[inline(always)]
pub const fn box_start(v: u8) -> u8 {
    row(v) / 3 * 27 + column(v) / 3 * 3
}

/// Gets the locations of the row that contains `v`.
pub fn row_group(v: u8) -> [u8; 9] {
    let v = v - v % 9; // start of row.
    [v, v + 1, v + 2, v + 3, v + 4, v + 5, v + 6, v + 7, v + 8]
}

/// Gets the locations of the column that contains `v`.
pub const fn column_group(v: u8) -> [u8; 9] {
    let v = v % 9; // start of column.
    [v, v + 9, v + 18, v + 27, v + 36, v + 45, v + 54, v + 63, v + 72]
}

/// Gets the other locations from within the box, given its top-left element.
pub const fn box_group(v: u8) -> [u8; 9] {
    let v = box_start(v);
    [v, v + 1, v + 2, v + 9, v + 10, v + 11, v + 18, v + 19, v + 20]
}

pub trait Location {
    fn coords(self) -> (u8, u8);
}

impl Location for u8 {
    fn coords(self) -> (u8, u8) {
        (row(self) + 1, column(self) + 1)
    }
}
