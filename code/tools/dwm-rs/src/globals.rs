use crate::prelude::*;

#[allow(non_upper_case_globals)]
pub static mut dpy: *mut c::Display = ptr::null_mut();

#[allow(non_upper_case_globals)]
pub static mons: Ptr<Monitor> = Ptr::NULL;
