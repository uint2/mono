use crate::C;
use crate::prelude::*;

pub struct DrwParams {
    pub screen: c_int,
    pub root: Window,
    pub size: Size,
}

pub struct Drw {
    screen: Screen,
    root: C::Window,
    /// Width and height of the drawing area.
    sz: Size,
    drawable: C::Drawable,
    /// Graphics context.
    gc: C::GC,
    scheme: Option<WindowColors<XftColor>>,
}

impl Drw {
    pub fn new(root: C::Window, screen: Screen, screen_size: Size) -> Self {
        let depth = dpy.default_depth(screen);
        let drawable = dpy.create_pixmap(root, screen_size, depth as c_uint);
        let gc = dpy.create_graphics_ctx(root);
        let drw = Self { screen, root, sz: screen_size, drawable, gc, scheme: None };
        drw.set_line_attributes(1, LineStyle::Solid, CapStyle::Butt, JoinStyle::Miter);
        drw
    }
}

impl Drop for Drw {
    fn drop(&mut self) {
        dpy.free_pixmap(self.drawable);
        dpy.free_graphics_ctx(self.gc);
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
        dpy.set_line_attributes(self.gc, line_width, line_style, cap_style, join_style)
    }
}
