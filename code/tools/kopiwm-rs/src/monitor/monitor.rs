use crate::prelude::*;
use config::{Coordinate, Distance};

impl Monitor {
    pub fn new() -> Self {
        Self {
            mfact: config::MFACT,
            nmaster: config::NMASTER,
            by: 0,
            m: Rect::zero(),
            w: Rect::zero(),
            tags: 0b1,
            show_bar: config::SHOW_BAR,
            bar_pos: config::BAR_POSITION,
            clients: vec![],
            sel: None,
            bar_window: None,
            lt: Toggle::new(&EMPTY_LAYOUT),
        }
    }

    pub const fn bar_window(&self) -> Option<Window> {
        let Some(ref window) = self.bar_window else { return None };
        Some(window.as_ref())
    }
}

impl Drop for Monitor {
    fn drop(&mut self) {
        use crate::C as X;

        if let Some(barwin) = self.bar_window.take() {
            unsafe { X::XUnmapWindow(dpy.c(), barwin.c()) };
            unsafe { X::XDestroyWindow(dpy.c(), barwin.c()) };
        }
    }
}

impl Monitor {
    pub fn update_bar_pos(&mut self, bar_height: Distance) {
        if !self.show_bar {
            // If the bar is not shown, then the dimensions of the windows
            // display area simply become the entire monitor.
            self.w = self.m;
            // Send the bar out of the screen.
            self.by = self.m.y - 2 * bar_height as Coordinate;
            return;
        }

        // Otherwise, the height of the display area is shortened by precisely
        // the bar height.
        self.w.height = self.m.height - bar_height;

        match self.bar_pos {
            BarPosition::Top => {
                self.by = self.m.y;
                self.w.y = self.m.y + bar_height as Coordinate;
            }
            BarPosition::Bottom => {
                self.by = self.m.b() - bar_height as Coordinate;
                self.w.y = self.m.y;
            }
        }
    }
}
