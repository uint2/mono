use crate::C;
use crate::prelude::*;

use core::alloc::Layout;

/// Thinnest wrapper around XftColor to manage drops.
pub struct XftColor {
    dpy: Display,
    screen: Screen,
    color: NonNull<C::XftColor>,
}
c!(XftColor, color);

const LAYOUT: Layout = Layout::new::<C::XftColor>();

impl Drop for XftColor {
    fn drop(&mut self) {
        log::trace!("Deallocating color {:p}...", self.color);
        let dpy = self.dpy.c();
        let visual = self.dpy.default_visual(self.screen);
        let cmap = self.dpy.default_colormap(self.screen);
        unsafe { C::XftColorFree(dpy, visual, cmap, self.c()) };
        unsafe { std::alloc::dealloc(self.c() as *mut u8, LAYOUT) };
    }
}

impl XftColor {
    /// Create a XftColor from a name (hex codes are allowed and encouraged).
    pub fn from_name(dpy: Display, screen: Screen, name: &str) -> Option<Self> {
        let visual = dpy.default_visual(screen);
        let cmap = dpy.default_colormap(screen);
        let color = unsafe { std::alloc::alloc(LAYOUT) } as *mut C::XftColor;
        let cname = name.c_str();
        let cname = cname.as_ptr();
        let result = unsafe { C::XftColorAllocName(dpy.c(), visual, cmap, cname, color) };
        if result == 0 {
            log::error!("X cannot allocator color: {name}");
            return None;
        }
        let Some(mut color) = NonNull::new(color) else {
            unsafe { C::XftColorFree(dpy.c(), visual, cmap, color) };
            unsafe { std::alloc::dealloc(color as *mut u8, LAYOUT) };
            return None;
        };
        // Force maximum opacity.
        unsafe { color.as_mut() }.pixel |= 0xff << 24;
        let value = Self { dpy, screen, color };
        log::trace!("Allocated color {:p}...", value.color);
        Some(value)
    }
}
