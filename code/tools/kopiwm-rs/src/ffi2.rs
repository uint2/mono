use crate::prelude::*;

pub trait ConvertToC<T> {
    fn c_str(self) -> T;
}

impl ConvertToC<CString> for &str {
    fn c_str(self) -> CString {
        CString::new(self).unwrap()
    }
}

pub fn u8_to_str<'a>(data: *const u8) -> Option<&'a str> {
    if data.is_null() {
        return None;
    }
    let n = unsafe { libc::strlen(data as *const i8) };
    let slice = unsafe { core::slice::from_raw_parts(data, n) };
    core::str::from_utf8(slice).ok()
}

pub fn i8_to_str<'a>(data: *const i8) -> Option<&'a str> {
    if data.is_null() {
        return None;
    }
    let n = unsafe { libc::strlen(data) };
    let slice = unsafe { core::slice::from_raw_parts(data as *const u8, n) };
    core::str::from_utf8(slice).ok()
}
