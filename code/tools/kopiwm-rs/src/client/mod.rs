use crate::C;
use crate::prelude::*;

mod client;
mod x;

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

pub struct Client {
    pub id: ClientId,
    /// The parent monitor to this client.
    pub mon: MonitorId,
    pub win: Window,
    /// Bitmask of active tags.
    pub tags: u32,
    pub name: String,
    /// Position, current and previous.
    pub pos: Toggle<Rect>,
    pub sz: ClientSizes,
    pub hints_valid: bool,
    /// Border width.
    pub border_width: Toggle<Distance>,
    pub is_fixed: bool,
    pub is_floating: Toggle<bool>,
    pub isurgent: bool,
    pub neverfocus: bool,
    pub isfullscreen: bool,
    /// Next client in the linked list of clients.
    pub next: Option<ClientId>,
    /// Next client in the stacking order. That is, the order in which windows
    /// appear visually. If window A covers window B, or is laid on top of it,
    /// then A is before B in the stacking order.
    pub snext: Option<ClientId>,
}
