use crate::C;
use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum BarPosition {
    Top,
    Bottom,
}

pub struct Monitor<'app, C = c_int, D = c_uint> {
    /// Master window factor.
    mfact: f32,
    /// Number of master windows.
    nmaster: u8,
    /// Status bar's y-coordinate.
    by: C,
    /// The Rect that every pixel on the monitor lives in.
    m: Rect<C, D>,
    /// The Rect that windows live in. This is simply the monitor's Rect minus
    /// the status bar's Rect.
    w: Rect<C, D>,
    /// The bitmask of visible tags. Initialize with the first tag visible.
    tags: u32,
    /// false means hide bar.
    show_bar: bool,
    bar_pos: BarPosition,
    /// Owned list of clients.
    clients: Vec<Client>,
    /// Selected client
    sel: Option<&'app Client>,
    /// Clients ordered by stacking order. That is, the order in which windows
    /// appear visually. If window A covers window B, or is laid on top of it,
    /// then A is before B in the stacking order.
    stack: Vec<&'app Client>,

    /// The X window that manages the status bar. The only time when this is
    /// none should be when the monitor is freshly created, and we just haven't
    /// initialized the bar window.
    bar_window: Option<Window>,

    lt: Toggle<&'static Layout>,
}

impl<'app> Monitor<'app, c_int, c_uint> {
    pub fn new() -> Self {
        Self {
            mfact: config::MFACT,
            nmaster: config::NMASTER,
            by: 0,
            m: Rect::new(0, 0, 0, 0),
            w: Rect::new(0, 0, 0, 0),
            tags: 0b1,
            show_bar: config::SHOW_BAR,
            bar_pos: config::BAR_POSITION,
            clients: vec![],
            sel: None,
            stack: vec![],
            bar_window: None,
            lt: Toggle::new(&EMPTY_LAYOUT),
        }
    }
}
