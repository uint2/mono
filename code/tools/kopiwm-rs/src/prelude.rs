#![allow(unused)]

// crate::*
pub(crate) use crate::x11::{Display, Window};

// std::*
pub(crate) use std::process::ExitCode;

// core::*
pub(crate) use core::cmp::Ordering;
pub(crate) use core::ffi::{c_int, c_long, c_uint, c_ulong};
pub(crate) use core::ptr;

// build-time consts.
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const NAME: &str = env!("CARGO_PKG_NAME");

pub(crate) type Result<T, E = ()> = core::result::Result<T, E>;
