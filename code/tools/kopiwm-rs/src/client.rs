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

    pub fn is_visible(&self) -> bool {
        // ((C->tags & C->mon->tagset[C->mon->seltags]))
        true
    }
}
