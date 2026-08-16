use crate::C;
use crate::prelude::*;

mod client;
mod size;
mod x;

use size::ClientSizes;

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
