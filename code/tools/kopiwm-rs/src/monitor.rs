use crate::C;
use crate::prelude::*;
use config::{Coordinate, Distance};

#[derive(Clone, Copy)]
pub enum BarPosition {
    Top,
    Bottom,
}

pub struct Monitor {
    dpy: Display,
    id: MonitorId,
    /// Master window factor.
    mfact: f32,
    /// Number of master windows.
    nmaster: u8,
    /// Status bar's y-coordinate.
    by: Coordinate,
    /// The Rect that every pixel on the monitor lives in.
    pub m: Rect,
    /// The Rect that windows live in. This is simply the monitor's Rect minus
    /// the status bar's Rect.
    pub w: Rect,
    /// The bitmask of visible tags. Initialize with the first tag visible.
    tags: u32,
    /// false means hide bar.
    show_bar: bool,
    bar_pos: BarPosition,
    /// Owned list of clients.
    clients: Vec<Client>,
    /// Selected client, as an index of our own set of clients.
    /// TODO: Figure out this exact architecture later.
    sel: Option<usize>,
    /// Clients ordered by stacking order. That is, the order in which windows
    /// appear visually. If window A covers window B, or is laid on top of it,
    /// then A is before B in the stacking order.
    /// TODO: figure out this architecture later too.
    // stack: Vec<&'app Client>,

    /// The X window that manages the status bar. The only time when this is
    /// none should be when the monitor is freshly created, and we just haven't
    /// initialized the bar window.
    bar_window: Option<Window>,

    lt: Toggle<&'static Layout>,
}

impl Monitor {
    pub fn new(dpy: Display) -> Self {
        Self {
            dpy,
            id: MonitorId::new(),
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
            // stack: vec![],
            bar_window: None,
            lt: Toggle::new(&EMPTY_LAYOUT),
        }
    }

    getter!(id, MonitorId);
    getter!(bar_window, Option<Window>);

    pub const fn clients(&self) -> &[Client] {
        self.clients.as_slice()
    }
}

impl Drop for Monitor {
    fn drop(&mut self) {
        use crate::C as X;

        if let Some(barwin) = self.bar_window.take() {
            unsafe { X::XUnmapWindow(self.dpy.c(), barwin.c()) };
            unsafe { X::XDestroyWindow(self.dpy.c(), barwin.c()) };
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
