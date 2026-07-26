use crate::prelude::*;

pub trait ConvertToC<T> {
    fn c(self) -> T;
}

impl ConvertToC<CString> for &str {
    fn c(self) -> CString {
        CString::new(self).unwrap()
    }
}
