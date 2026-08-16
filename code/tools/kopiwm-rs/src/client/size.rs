use crate::C;
use crate::prelude::*;

pub struct ClientSizes {
    pub base: Size,
    /// Incremental size when resizing.
    pub inc: Size,
    pub max: Size,
    pub min: Size,
    /// Maximum aspect ratio (width / height).
    pub max_ar: f64,
    /// Minimum aspect ratio (height / width).
    /// Note that this is the reciprocal of the conventional notion of the
    /// aspect ratio because of how we'll be using it.
    pub min_ar: f64,
}

macro_rules! size {
    ($width:expr, $height:expr) => {
        Size::new($width as Distance, $height as Distance)
    };
}

impl ClientSizes {
    pub const fn new() -> Self {
        let zero = Size::new(0, 0);
        Self { base: zero, inc: zero, max: zero, min: zero, max_ar: 0., min_ar: 0. }
    }

    pub fn update(&mut self, hints: &C::XSizeHints) {
        let flags = hints.flags as c_uint;

        macro_rules! flags {
            ($bitmask:expr) => {
                flags & $bitmask != 0
            };
        }

        self.base = if flags!(C::PBaseSize) {
            size!(hints.base_width, hints.base_height)
        } else if flags!(C::PMinSize) {
            size!(hints.min_width, hints.min_height)
        } else {
            Size::new(0, 0)
        };

        self.inc = if flags!(C::PResizeInc) {
            size!(hints.width_inc, hints.height_inc)
        } else {
            Size::new(0, 0)
        };

        self.max = if flags!(C::PMaxSize) {
            size!(hints.max_width, hints.max_height)
        } else {
            Size::new(0, 0)
        };

        self.min = if flags!(C::PMinSize) {
            size!(hints.min_width, hints.min_height)
        } else if flags!(C::PBaseSize) {
            size!(hints.base_width, hints.base_height)
        } else {
            Size::new(0, 0)
        };

        if flags!(C::PAspect) {
            self.min_ar = hints.min_aspect.y as f64 / hints.min_aspect.x as f64;
            self.max_ar = hints.max_aspect.x as f64 / hints.max_aspect.y as f64;
        } else {
            self.min_ar = 0.;
            self.max_ar = 0.;
        }
    }

    /// Check to see if the max and min dimensions are the same, because if they
    /// are, then there is no resizing this client, and hence it's fixed-sized.
    pub fn is_fixed(&self) -> bool {
        self.max.width > 0
            && self.max.height > 0
            && self.max.width == self.min.width
            && self.max.height == self.min.height
    }
}
