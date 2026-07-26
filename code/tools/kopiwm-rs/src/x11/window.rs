use crate::C;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Window(C::Window);

impl Window {
    pub const fn c(&self) -> C::Window {
        self.0
    }

    pub const fn from_c(window: C::Window) -> Self {
        Self(window)
    }
}
