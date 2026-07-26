use crate::C;
use crate::prelude::*;

pub(crate) struct NumLockMask {
    modifiers: [c_uint; 4],
}

impl NumLockMask {
    pub const fn new() -> Self {
        Self { modifiers: [0, C::LockMask, 0, C::LockMask] }
    }

    /// Updates the numlockmask, which is located at `self.modifiers[2]`.
    pub fn update(&mut self, dpy: &Display) {
        // Reset numlockmask.
        self.modifiers[2] = 0;
        // TODO: back here
    }
}
