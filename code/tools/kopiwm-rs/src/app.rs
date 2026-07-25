use crate::prelude::*;

/// C: type for Coordinates.
/// D: type for Distance.
pub(crate) struct App<'app, D = c_uint> {
    dpy: Display,
    screen: c_int,
    /// Screen size.
    /// Apparently dwm updates this in `void configurenotify(XEvent *)`, and
    /// that's probably how multipe monitors are supported.
    s: Size<D>,
    lrpad: D,
    bar_height: D,
    mons: LinkedList<Monitor>,
    selmon: &'app Monitor,
    root: Window,
    cursors: EnumArray<CursorState, Cursor>,
    scheme: EnumArray<SchemeState, ColorScheme>,
    status_text: String,
    numlockmask: NumLockMask,
    fonts: Vec<Font>,
    running: bool,
}
