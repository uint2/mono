use crate::C;
use crate::prelude::*;

pub(crate) struct DrwParams<'a> {
    pub dpy: Display,
    pub screen: c_int,
    pub root: Window,
    pub size: Size,
    pub colors: &'a EnumArray<SchemeState, Scheme<&'static str>>,
}

pub(crate) struct Drw {
    dpy: Display,
    screen: c_int,
    root: Window,
    /// Width and height of the drawing area.
    sz: Size,
    drawable: C::Drawable,
    /// Graphics context.
    gc: C::GC,
    scheme: Option<ColorScheme>,
}

impl Drw {
    pub fn new(params: DrwParams) -> Self {
        let depth = params.dpy.default_depth(params.screen);
        let dpy = params.dpy;
        let drawable = dpy.create_pixmap(params.root, params.size, depth as c_uint);
        let gc = dpy.create_graphics_ctx(params.root);
        let drw = Self {
            dpy,
            screen: params.screen,
            root: params.root,
            sz: params.size,
            drawable,
            gc,
            scheme: None,
        };
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
