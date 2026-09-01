use crate::C;
use crate::prelude::*;

use core::ops::{Deref, DerefMut};

pub struct XPtr<T>(NonNull<T>);

impl<T> XPtr<T> {
    pub const fn new(value: *mut T) -> Option<Self> {
        let Some(value) = NonNull::new(value) else { return None };
        Some(Self(value))
    }

    pub const fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    pub unsafe fn get(&self, offset: usize) -> &T {
        unsafe { self.0.add(offset).as_ref() }
    }
}

impl<T> Drop for XPtr<T> {
    fn drop(&mut self) {
        unsafe { C::XFree(self.0.as_ptr() as *mut c_void) };
    }
}

impl XPtr<i8> {
    pub fn to_str<'a>(&self) -> Option<&'a str> {
        let data = self.as_ptr();
        let n = unsafe { libc::strlen(data) };
        let slice = unsafe { core::slice::from_raw_parts(data as *const u8, n) };
        core::str::from_utf8(slice).ok()
    }
}

impl XPtr<u8> {
    pub fn to_str<'a>(&self) -> Option<&'a str> {
        let data = self.as_ptr();
        let n = unsafe { libc::strlen(data as *const i8) };
        let slice = unsafe { core::slice::from_raw_parts(data, n) };
        core::str::from_utf8(slice).ok()
    }
}

impl<T> Deref for XPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> DerefMut for XPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
