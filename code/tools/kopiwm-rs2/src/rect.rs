use crate::prelude::*;
use config::{Coordinate, Distance};

pub struct Size<T = Distance> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<T: Copy> Size<T> {
    pub fn convert<U: TryFrom<T>>(&self) -> Size<U> {
        let Ok(width) = self.width.try_into() else { panic!("Invalid conversion") };
        let Ok(height) = self.height.try_into() else { panic!("Invalid conversion") };
        Size { width, height }
    }
}

#[rustfmt::skip]
impl<T: Clone> Clone for Size<T> { fn clone(&self) -> Self { Self { width: self.width.clone(), height: self.height.clone() } } }
impl<T: Copy> Copy for Size<T> {}

pub struct Loc<T = Coordinate> {
    pub x: T,
    pub y: T,
}

impl<T> Loc<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy> Loc<T> {
    pub fn convert<U: TryFrom<T>>(&self) -> Loc<U> {
        let Ok(x) = self.x.try_into() else { panic!("Invalid conversion") };
        let Ok(y) = self.y.try_into() else { panic!("Invalid conversion") };
        Loc { x, y }
    }
}

#[rustfmt::skip]
impl<T: Clone> Clone for Loc<T> { fn clone(&self) -> Self { Self { x: self.x.clone(), y: self.y.clone() } } }
impl<T: Copy> Copy for Loc<T> {}

/// As per X, the coordinate system's x-value increases from left to right, and
/// the y-value increases from top to bottom.
#[derive(Clone, Copy)]
pub struct Rect {
    /// The left-most coordinate of the rectangle.
    pub x: Coordinate,
    /// The top-most coordinate of the rectangle.
    pub y: Coordinate,
    pub width: Distance,
    pub height: Distance,
}

/// The four extremes of a Rect.
impl Rect {
    /// The left-most coordinate.
    #[inline]
    pub const fn l(self: &Self) -> Coordinate {
        self.x
    }

    /// The right-most coordinate.
    #[inline]
    pub const fn r(self: &Self) -> Coordinate {
        self.x + self.width as Coordinate
    }

    /// The top-most coordinate.
    #[inline]
    pub const fn t(self: &Self) -> Coordinate {
        self.y
    }

    /// The bottom-most coordinate.
    #[inline]
    pub const fn b(self: &Self) -> Coordinate {
        self.y + self.height as Coordinate
    }
}

impl Rect {
    pub const fn zero() -> Self {
        Self { x: 0, y: 0, width: 0, height: 0 }
    }

    pub const fn set_location(&mut self, location: Loc) {
        self.x = location.x;
        self.y = location.y;
    }

    pub const fn set_size(&mut self, size: Size) {
        self.width = size.width;
        self.height = size.height;
    }

    /// Get the area of intersection. Always returns a non-negative value.
    pub fn intersect(&self, rhs: &Self) -> Distance {
        let width = self.r().min(rhs.r()) - self.l().max(rhs.l());
        let height = self.b().min(rhs.b()) - self.t().max(rhs.t());
        width.max(0) as Distance * height.max(0) as Distance
    }
}
