mod monitor;

use crate::prelude::*;

pub struct Monitor {
    /// Master window factor.
    pub mfact: f32,
    /// Number of master windows.
    pub nmaster: u8,
    /// Status bar's y-coordinate.
    pub by: Coordinate,
    /// The Rect that every pixel on the monitor lives in.
    pub m: Rect,
    /// The Rect that windows live in. This is simply the monitor's Rect minus
    /// the status bar's Rect.
    pub w: Rect,
    /// The bitmask of visible tags. Initialize with the first tag visible.
    pub tags: u32,
    /// false means hide bar.
    pub show_bar: bool,
    pub bar_pos: BarPosition,
    /// List of clients, ordered by stacking order.
    ///
    /// That is, the order in which windows appear visually. If window A covers
    /// window B, or is laid on top of it, then A is before B in the stacking
    /// order.
    pub clients: Vec<Arc<RwLock<Client>>>,
    /// Selected client, as an index of our own set of clients.
    pub sel: Option<Arc<RwLock<Client>>>,

    /// The X window that manages the status bar. The only time when this is
    /// none should be when the monitor is freshly created, and we just haven't
    /// initialized the bar window.
    pub bar_window: Option<OwnedWindow>,

    pub lt: Toggle<&'static Layout>,
}
