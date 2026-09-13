pub(crate) use std::ffi::{c_float, c_int, c_uint};
pub(crate) use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub(crate) use core::ptr;

#[allow(unused)]
pub(crate) use crate::{
    c,
    config::{TAGMASK, rules, tags},
    dtypes::*,
    globals::{dpy, mons},
    pointer::Ptr,
};
