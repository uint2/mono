use crate::prelude::*;

/// C: type for Coordinates.
/// D: type for Distance.
pub struct App<'app, C = c_int, D = c_uint> {
    dpy: Display,
    screen: c_int,
    /// Screen size.
    /// Apparently dwm updates this in `void configurenotify(XEvent *)`, and
    /// that's probably how multipe monitors are supported.
    s: Size<D>,
    lrpad: D,
    bar_height: D,
    /// Owned list of moitors.
    mons: Vec<Monitor<'app, C, D>>,
    selmon: &'app Monitor<'app, C, D>,
    root: Window,
    cursors: CursorStateArray<Cursor>,
    colors: WindowColorStateArray<WindowColors<XftColor>>,
    status_text: String,
    numlockmask: NumLockMask,
    fonts: Vec<Font>,
    running: bool,
}
