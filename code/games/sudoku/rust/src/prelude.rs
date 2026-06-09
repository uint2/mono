// #![allow(unused)]

pub(crate) use crate::board::*;
pub(crate) use crate::cell::*;
pub(crate) use crate::game_data::*;
pub(crate) use crate::location::Location;
pub(crate) use crate::ringbuf::*;
pub(crate) use crate::stats::{GameStat, GlobalStats};
pub(crate) use crate::{location, rules};

pub(crate) use core::cmp::Ordering;
pub(crate) use core::fmt;
pub(crate) use core::ops::{BitOr, Index, IndexMut};
pub(crate) use core::time::Duration;

pub(crate) use std::fs::File;
pub(crate) use std::io::Write;
pub(crate) use std::time::Instant;

pub(crate) enum Error {
    NoCandidates(u8),
    InvalidBoardFromClash(u8, u8),
    BoardNotSolved,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCandidates(loc) => {
                write!(f, "Invalid by no candidates at {:?}", loc.coords())
            }
            Self::InvalidBoardFromClash(loc1, loc2) => write!(
                f,
                "Invalid board, clash at locations A{:?} and B{:?}",
                loc1.coords(),
                loc2.coords(),
            ),
            Self::BoardNotSolved => write!(f, "The board is not solved yet."),
        }
    }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;
