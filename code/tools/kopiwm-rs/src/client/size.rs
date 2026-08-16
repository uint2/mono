use crate::C;
use crate::prelude::*;

pub struct ClientSizes {
    pub base: Option<Size>,
    /// Incremental size when resizing.
    pub inc: Option<Size>,
    pub max: Option<Size>,
    pub min: Option<Size>,
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
        Self { base: None, inc: None, max: None, min: None, max_ar: 0., min_ar: 0. }
    }

    pub fn update_base(&mut self, hints: &C::XSizeHints) {
        let flags = hints.flags as c_uint;
        self.base = Some(if flags & C::PBaseSize != 0 {
            size!(hints.base_width, hints.base_height)
        } else if flags & C::PMinSize != 0 {
            size!(hints.min_width, hints.min_height)
        } else {
            Size::new(0, 0)
        });
    }

    pub fn update_inc(&mut self, hints: &C::XSizeHints) {
        let flags = hints.flags as c_uint;
        self.inc = Some(if flags & C::PResizeInc != 0 {
            size!(hints.width_inc, hints.height_inc)
        } else {
            Size::new(0, 0)
        });
    }

    pub fn update_max(&mut self, hints: &C::XSizeHints) {
        let flags = hints.flags as c_uint;
        self.max = Some(if flags & C::PMaxSize != 0 {
            size!(hints.max_width, hints.max_height)
        } else {
            Size::new(0, 0)
        });
    }

    pub fn update_min(&mut self, hints: &C::XSizeHints) {
        let flags = hints.flags as c_uint;
        self.min = Some(if flags & C::PMinSize != 0 {
            size!(hints.min_width, hints.min_height)
        } else if flags & C::PBaseSize != 0 {
            size!(hints.base_width, hints.base_height)
        } else {
            Size::new(0, 0)
        });
    }

    /// Check to see if the max and min dimensions are the same, because if they
    /// are, then there is no resizing this client, and hence it's fixed-sized.
    pub fn is_fixed(&self) -> bool {
        let Some(max) = &self.max else { return false };
        let Some(min) = &self.min else { return false };
        max.width > 0
            && max.height > 0
            && max.width == min.width
            && max.height == min.height
    }
}
