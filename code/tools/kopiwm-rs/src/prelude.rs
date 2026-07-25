#![allow(unused)]

// crate::*
pub(crate) use crate::app::App;
pub(crate) use crate::client::Client;
pub(crate) use crate::enum_array::{EnumArray, ToUsizeIndex};
pub(crate) use crate::enums::{Clk, CursorState, Scheme, SchemeState};
pub(crate) use crate::font::Font;
pub(crate) use crate::monitor::Monitor;
pub(crate) use crate::numlockmask::NumLockMask;
pub(crate) use crate::rect::{Loc, Rect, Size};
pub(crate) use crate::x11::enums::*;
pub(crate) use crate::x11::{ColorScheme, Cursor, Display, Window};

// std::*
pub(crate) use std::collections::LinkedList;
pub(crate) use std::marker::PhantomData;
pub(crate) use std::process::ExitCode;

// core::*
pub(crate) use core::cmp::Ordering;
pub(crate) use core::ffi::{c_int, c_long, c_uint, c_ulong};
pub(crate) use core::ptr;

// external.
pub(crate) use strum::EnumCount;

// build-time consts.
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const NAME: &str = env!("CARGO_PKG_NAME");

pub(crate) type Result<T, E = ()> = core::result::Result<T, E>;
