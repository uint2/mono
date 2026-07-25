use crate::prelude::*;

pub struct Size<T = c_uint> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

#[rustfmt::skip]
impl<T: Clone> Clone for Size<T> { fn clone(&self) -> Self { Self { width: self.width.clone(), height: self.height.clone() } } }
impl<T: Copy> Copy for Size<T> {}

pub struct Loc<T = c_int> {
    pub x: T,
    pub y: T,
}

impl<T> Loc<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[rustfmt::skip]
impl<T: Clone> Clone for Loc<T> { fn clone(&self) -> Self { Self { x: self.x.clone(), y: self.y.clone() } } }
impl<T: Copy> Copy for Loc<T> {}

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
