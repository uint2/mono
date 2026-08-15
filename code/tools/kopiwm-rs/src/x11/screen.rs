use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Screen {
    screen: c_int,
}

impl Screen {
    pub const fn c(&self) -> c_int {
        self.screen
    }

    pub const fn from_c(screen: c_int) -> Self {
        Self { screen }
    }
}
