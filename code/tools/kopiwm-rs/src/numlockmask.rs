use crate::C;
use crate::prelude::*;

pub struct NumLockMask {
    modifiers: [c_uint; 4],
}

impl NumLockMask {
    pub const fn new() -> Self {
        Self { modifiers: [0, C::LockMask, 0, C::LockMask] }
    }

    /// Updates the numlockmask, which is located at `self.modifiers[2]`.
    pub fn update(&mut self) {
        // Reset numlockmask.
        self.modifiers[2] = 0;

        let Some(modmap) = dpy.get_modifier_mapping() else {
            log::warn!("Unable to get modifier mapping");
            return;
        };
        let mkpm = modmap.max_keypermod();
        let mmap = modmap.modifiermap();
        for i in 0..8 {
            for j in 0..mkpm {
                let keycode = mmap[i * mkpm + j];
                if keycode == dpy.keysym_to_keycode(C::XK_Num_Lock as C::KeySym) {
                    self.modifiers[2] = 1 << i;
                }
            }
        }
        self.modifiers[3] = self.modifiers[2] | C::LockMask;
    }

    pub const fn modifiers(&self) -> &[c_uint; 4] {
        &self.modifiers
    }

    pub fn cleanmask(&self, mask: c_uint) -> c_uint {
        const ALL_MASK: c_uint = C::ShiftMask
            | C::ControlMask
            | C::Mod1Mask
            | C::Mod2Mask
            | C::Mod3Mask
            | C::Mod4Mask
            | C::Mod5Mask;
        return (mask & !self.modifiers[3]) & ALL_MASK;
    }

    pub fn grabkey(&self, root: &Window, key: &Key, keycode: c_int) {
        for modifier in self.modifiers {
            unsafe {
                C::XGrabKey(
                    dpy.c(),
                    keycode,
                    key.modifier | modifier,
                    root.c(),
                    C::True as c_int,
                    C::GrabModeAsync as c_int,
                    C::GrabModeAsync as c_int,
                );
            }
        }
    }
}
