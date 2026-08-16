use crate::prelude::*;

pub trait ConvertToC<T> {
    fn c_str(self) -> T;
}

impl ConvertToC<CString> for &str {
    fn c_str(self) -> CString {
        CString::new(self).unwrap()
    }
}
