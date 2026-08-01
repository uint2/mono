use crate::prelude::*;
use config::{DEFAULT_COORDINATE_TYPE, DEFAULT_DISTANCE_TYPE};

/// C: type for Coordinates.
/// D: type for Distance.
pub struct App<'app, C = DEFAULT_COORDINATE_TYPE, D = DEFAULT_DISTANCE_TYPE> {
    dpy: Display,
    screen: c_int,
    /// Screen size.
    /// Apparently dwm updates this in `void configurenotify(XEvent *)`, and
    /// that's probably how multipe monitors are supported.
    s: Size<D>,
    lrpad: D,
    bar_height: D,
    /// Owned list of moitors. It is guaranteed that for the lifetime of `Self`,
    /// this list is non-empty.
    mons: NonEmpty<Monitor<'app, C, D>>,
    /// Index of the selected monitor.
    selmon: usize,
    root: Window,
    cursors: CursorStateArray<Cursor>,
    colors: WindowColorStateArray<WindowColors<XftColor>>,
    status_text: String,
    numlockmask: NumLockMask,
    fonts: NonEmpty<Font>,
    running: bool,
}

/// Getters.
impl<'app> App<'app> {
    pub fn selmon(&self) -> &Monitor<'app> {
        &self.mons[self.selmon]
    }
}

/// Core Logic.
impl<'app, C: Copy, D: Copy + Eq> App<'app, C, D> {
    pub fn updategeom(&mut self) -> bool {
        let mut dirty = false;
        let m = self.mons.first_mut();
        if m.m.width() != self.s.width || m.m.height() != self.s.height {
            dirty = true;
            m.m.set_size(self.s);
            m.w.set_size(self.s);
            m.update_bar_pos(self.bar_height);
        }
        if dirty {
            self.selmon = 0; // we took the first monitor as `m`.
            self.selmon = self.window_to_monitor_idx(self.root);
        }
        dirty
    }

    /// Finds the monitor that contains `window`.
    /// Fallback: currently selected monitor.
    pub fn window_to_monitor_idx(&self, window: Window) -> usize {
        todo!()
    }
}

impl<'app, C> App<'app, C, c_int> {
    pub fn get_root_pointer() -> Loc<c_int> {
        todo!()
    }
}
