use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Screen(c_int);

impl Screen {
    pub const fn c(&self) -> c_int {
        self.0
    }

    pub const fn from_c(screen: c_int) -> Self {
        Self(screen)
    }
}
