use crate::C;
use crate::prelude::*;

/// Thinnest wrapper around XModifierKeymap to manage drops.
pub struct XModifierKeymap(NonNull<C::XModifierKeymap>);
make_new!(XModifierKeymap);

impl Drop for XModifierKeymap {
    fn drop(&mut self) {
        unsafe { C::XFreeModifiermap(self.c()) };
    }
}

impl XModifierKeymap {
    pub const fn max_keypermod(&self) -> usize {
        unsafe { self.0.as_ref() }.max_keypermod as usize
    }

    pub fn modifiermap(&self) -> &[u8] {
        let modmap = unsafe { self.0.as_ref() }.modifiermap;
        let len = 8 * self.max_keypermod();
        unsafe { core::slice::from_raw_parts(modmap, len) }
    }
}
