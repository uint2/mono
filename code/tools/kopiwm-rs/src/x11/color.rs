use crate::C;
use crate::prelude::*;

impl Scheme<C::XftColor> {}

pub struct ColorScheme {
    dpy: Display,
    screen: Screen,
    scheme: Scheme<C::XftColor>,
}

struct Helper {
    dpy: Display,
    visual: *mut C::Visual,
    cmap: C::Colormap,
}

impl Helper {
    fn new(dpy: Display, screen: Screen) -> Self {
        Self {
            dpy,
            visual: dpy.default_visual(screen),
            cmap: dpy.default_colormap(screen),
        }
    }

    pub fn create(&self, name: &str, dest: &mut C::XftColor) -> Result<(), ()> {
        let dpy = self.dpy.c();
        let visual = self.visual;
        let cmap = self.cmap;
        let cname = name.c_str();
        let result =
            unsafe { C::XftColorAllocName(dpy, visual, cmap, cname.as_ptr(), dest) };
        if result == 0 {
            log::error!("Cannot allocator color: {name}");
            return Err(());
        }
        // Force maximum opacity.
        dest.pixel |= 0xff << 24;
        Ok(())
    }

    fn destroy(&self, color: &mut C::XftColor) {
        let dpy = self.dpy.c();
        let visual = self.visual;
        let cmap = self.cmap;
        unsafe { C::XftColorFree(dpy, visual, cmap, color) }
    }
}

impl ColorScheme {
    pub fn new(dpy: Display, screen: Screen, scheme: Scheme<&str>) -> Option<Self> {
        log::info!("Allocating colorscheme {scheme:?}...");
        let mut z: Scheme<C::XftColor> = unsafe { core::mem::zeroed() };
        let h = Helper::new(dpy, screen);
        h.create(scheme.fg, &mut z.fg).ok()?;
        h.create(scheme.bg, &mut z.bg).ok()?;
        h.create(scheme.border, &mut z.border).ok()?;
        Some(Self { dpy, screen, scheme: z })
    }
}

impl Drop for ColorScheme {
    fn drop(&mut self) {
        log::info!("Deallocating colorscheme {:p}...", self);
        let h = Helper::new(self.dpy, self.screen);
        h.destroy(&mut self.scheme.fg);
        h.destroy(&mut self.scheme.bg);
        h.destroy(&mut self.scheme.border);
    }
}
