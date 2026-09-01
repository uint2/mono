use core::ops::{BitAnd, BitOr, Not};

#[derive(Clone, Copy)]
pub struct TagMask(u32);

impl BitOr for TagMask {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for TagMask {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Not for TagMask {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl TagMask {
    pub const EMPTY: Self = Self(0);

    pub const fn non_zero(&self) -> bool {
        self.0 != 0
    }
}
