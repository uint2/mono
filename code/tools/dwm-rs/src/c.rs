#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![allow(unnecessary_transmutes)]

include!("generated_bindings.rs");

impl Default for XSetWindowAttributes {
    /// Initialize with zeroes to all values. XSetWindowAttributes is a struct
    /// full of integers anyway.
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

mod hard_coded_missing_enums {
    use super::Atom;

    pub const XA_ATOM: Atom = 4;
}

pub use hard_coded_missing_enums::*;

pub fn undefined<T>() -> T {
    unsafe { core::mem::zeroed() }
}

pub fn to_str<'a>(ptr: *const i8) -> Option<&'a str> {
    if ptr.is_null() {
        return Option::None;
    }
    (unsafe { std::ffi::CStr::from_ptr(ptr) }).to_str().ok()
}

pub fn xfree<T>(value: *mut T) {
    unsafe { XFree(value as *mut std::os::raw::c_void) };
}
