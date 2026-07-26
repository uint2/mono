use crate::C;
use crate::prelude::*;

impl Scheme<C::XftColor> {}

pub struct ColorScheme {
    dpy: Display,
    scheme: Scheme<C::XftColor>,
}

struct Helper {
    dpy: Display,
    visual: *mut C::Visual,
    cmap: C::Colormap,
}

impl Helper {
    pub fn create(&self, name: &str, dest: &mut C::XftColor) -> Result<(), ()> {
        let dpy = self.dpy.c();
        let visual = self.visual;
        let cmap = self.cmap;
        let cname = name.c_str().as_ptr();
        let result = unsafe { C::XftColorAllocName(dpy, visual, cmap, cname, dest) };
        if result == 0 {
            log::error!("Cannot allocator color: {name}");
            return Err(());
        }
        // Force maximum opacity.
        dest.pixel |= 0xff << 24;
        Ok(())
    }
}

impl ColorScheme {
    pub fn new(dpy: Display, screen: Screen, scheme: Scheme<&str>) -> Option<Self> {
        let visual = dpy.default_visual(screen);
        let mut z: Scheme<C::XftColor> = unsafe { core::mem::zeroed() };
        let h = Helper {
            dpy,
            visual: dpy.default_visual(screen),
            cmap: dpy.default_colormap(screen),
        };
        h.create(scheme.fg, &mut z.fg).ok()?;
        h.create(scheme.bg, &mut z.bg).ok()?;
        h.create(scheme.border, &mut z.border).ok()?;
        Some(Self { dpy, scheme: z })
    }
}
