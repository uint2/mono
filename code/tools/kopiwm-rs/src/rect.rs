use crate::prelude::*;

pub struct Size<T = c_uint> {
    width: T,
    height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

pub struct Loc<T = c_int> {
    x: T,
    y: T,
}

impl<T> Loc<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// C: type for Coordinates.
/// D: type for Distance.
pub struct Rect<C = c_int, D = c_uint> {
    /// Location/Position.
    loc: Loc<C>,
    /// Size.
    sz: Size<D>,
}

impl<C, D: Copy> Rect<C, D> {
    #[inline]
    pub const fn width(&self) -> D {
        self.sz.width
    }

    #[inline]
    pub const fn height(&self) -> D {
        self.sz.height
    }
}

impl<C: Copy, D> Rect<C, D> {
    #[inline]
    pub const fn x(&self) -> C {
        self.loc.x
    }

    #[inline]
    pub const fn y(&self) -> C {
        self.loc.y
    }
}
