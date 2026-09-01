use crate::C;
use crate::prelude::*;

/// Thinnest wrapper around FcPattern to manage drops.
pub struct FcPattern(NonNull<C::FcPattern>);
make_new!(FcPattern);

impl Drop for FcPattern {
    fn drop(&mut self) {
        unsafe { C::FcPatternDestroy(self.c()) };
    }
}

impl FcPattern {
    pub fn from_name(name: &str) -> Option<Self> {
        let pattern = unsafe { C::FcNameParse(name.c_str().as_ptr() as *const u8) };
        Self::new(pattern)
    }
}
