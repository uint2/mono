use crate::prelude::*;

pub struct Client<'monitor> {
    id: ClientId,
    mon: MonitorId,
    mon2: &'monitor Monitor<'monitor>,
    win: Window,
    tags: u32,
}

impl<'monitor> Client<'monitor> {
    getter!(mon, MonitorId);
    getter!(id, ClientId);

    pub const fn win(&self) -> &Window {
        &self.win
    }

    /// A client is visible if and only if there exists a bit that matches
    /// between its own bitmask, and that of its owning monitor.
    pub fn is_visible(&self) -> bool {
        self.tags & self.mon2.tags != 0
    }
}
