#![allow(unused)]

// crate::*
pub(crate) use crate::app::App;
pub(crate) use crate::client::Client;
pub(crate) use crate::config;
pub(crate) use crate::drw::{Drw, DrwParams};
pub(crate) use crate::enums::{Clk, CursorState, WindowColorState, WindowColors};
pub(crate) use crate::enums::{CursorStateArray, WindowColorStateArray};
pub(crate) use crate::ffi2::ConvertToC;
pub(crate) use crate::font::{Font, Fonts};
pub(crate) use crate::linked_list::LinkedListNode;
pub(crate) use crate::monitor::Monitor;
pub(crate) use crate::numlockmask::NumLockMask;
pub(crate) use crate::rect::{Loc, Rect, Size};
pub(crate) use crate::x11::prelude::*;

// std::*
pub(crate) use std::collections::LinkedList;
pub(crate) use std::ffi::CString;
pub(crate) use std::process::ExitCode;

// core::*
pub(crate) use core::cmp::Ordering;
pub(crate) use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong};
pub(crate) use core::marker::{PhantomData, PhantomPinned};
pub(crate) use core::mem::MaybeUninit;
pub(crate) use core::pin::Pin;
pub(crate) use core::ptr::{self, NonNull};

// external.
pub(crate) use strum::EnumCount;

// build-time consts.
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const NAME: &str = env!("CARGO_PKG_NAME");

pub(crate) type Result<T, E = ()> = core::result::Result<T, E>;
