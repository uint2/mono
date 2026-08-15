use crate::prelude::*;

pub struct ClientSizes {
    base: Option<Size>,
    /// Incremental size when resizing.
    inc: Option<Size>,
    max: Option<Size>,
    min: Option<Size>,
    /// Maximum aspect ratio (width / height).
    max_ar: Option<Size>,
    /// Minimum aspect ratio (height / width).
    /// Note that this is the reciprocal of the conventional notion of the
    /// aspect ratio because of how we'll be using it.
    min_ar: Option<Size>,
}

impl ClientSizes {
    pub const fn new() -> Self {
        Self { base: None, inc: None, max: None, min: None, max_ar: None, min_ar: None }
    }
}

pub struct Client<'monitor> {
    id: ClientId,
    /// The parent monitor to this client.
    pub mon: &'monitor Monitor<'monitor>,
    win: Window,
    /// Bitmask of active tags.
    tags: u32,
    name: String,
    /// Position, current and previous.
    pos: Toggle<Rect>,
    sz: ClientSizes,
    hints_valid: bool,
    /// Border width.
    border_width: Toggle<c_uint>,
    is_fixed: bool,
    is_floating: Toggle<bool>,
    isurgent: bool,
    neverfocus: bool,
    isfullscreen: bool,
    /// Next client in the linked list of clients.
    next: Rc<Self>,
    /// Next client in the stacking order. That is, the order in which windows
    /// appear visually. If window A covers window B, or is laid on top of it,
    /// then A is before B in the stacking order.
    snext: Rc<Self>,
}

impl<'monitor> Client<'monitor> {
    getter!(id, ClientId);

    pub fn new() -> Self {
        todo!()
    }

    pub const fn win(&self) -> &Window {
        &self.win
    }

    /// A client is visible if and only if there exists a bit that matches
    /// between its own bitmask, and that of its owning monitor.
    pub fn is_visible(&self) -> bool {
        self.tags & self.mon.tags != 0
    }
}
