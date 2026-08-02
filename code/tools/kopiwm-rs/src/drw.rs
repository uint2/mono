use crate::C;
use crate::prelude::*;

pub struct DrwParams {
    pub dpy: Display,
    pub screen: c_int,
    pub root: Window,
    pub size: Size,
}

pub struct Drw {
    dpy: Display,
    screen: Screen,
    root: Window,
    /// Width and height of the drawing area.
    sz: Size,
    drawable: C::Drawable,
    /// Graphics context.
    gc: C::GC,
    scheme: Option<WindowColors<XftColor>>,
}

impl Drw {
    pub fn new(dpy: Display, root: Window, screen: Screen, screen_size: Size) -> Self {
        let depth = dpy.default_depth(screen);
        let drawable = dpy.create_pixmap(&root, screen_size, depth as c_uint);
        let gc = dpy.create_graphics_ctx(&root);
        let drw = Self { dpy, screen, root, sz: screen_size, drawable, gc, scheme: None };
        drw.set_line_attributes(1, LineStyle::Solid, CapStyle::Butt, JoinStyle::Miter);
        drw
    }
}

impl Drop for Drw {
    fn drop(&mut self) {
        self.dpy.free_pixmap(self.drawable);
        self.dpy.free_graphics_ctx(self.gc);
    }
}

impl Drw {
    pub fn set_line_attributes(
        &self,
        line_width: c_uint,
        line_style: LineStyle,
        cap_style: CapStyle,
        join_style: JoinStyle,
    ) {
        self.dpy
            .set_line_attributes(self.gc, line_width, line_style, cap_style, join_style)
    }
}
