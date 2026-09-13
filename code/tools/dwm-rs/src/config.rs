#![allow(unused)]

use crate::prelude::*;

#[allow(non_upper_case_globals)]
pub const rules: [Rule; 0] = [];

#[allow(non_upper_case_globals)]
pub const tags: [&'static str; 5] = ["1", "2", "3", "4", "T"];

pub const TAGMASK: c_uint = (1 << tags.len()) - 1;
