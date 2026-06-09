use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl<'a> fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl<T> Index<&Point> for Vec<Vec<T>> {
    type Output = T;
    fn index<'a>(&'a self, p: &Point) -> &'a T {
        &self[p.row][p.col]
    }
}

impl<T> IndexMut<&Point> for Vec<Vec<T>> {
    fn index_mut<'a>(&'a mut self, p: &Point) -> &'a mut T {
        &mut self[p.row][p.col]
    }
}

#[derive(Clone, Copy)]
pub enum State {
    Water,
    Air,
    None,
}

impl State {
    pub fn as_char(&self) -> char {
        match self {
            State::Water => '■',
            State::Air => '×',
            State::None => ' ',
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            State::None => true,
            _ => false,
        }
    }

    pub fn is_fluid(&self) -> bool {
        match self {
            State::Water | State::Air => true,
            _ => false,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            State::Water => State::Air,
            State::Air => State::Water,
            _ => State::None,
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
