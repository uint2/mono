use crate::prelude::*;

pub struct Client {
    id: ClientId,
    mon: MonitorId,
    win: Window,
}

impl Client {
    getter!(mon, MonitorId);
    getter!(id, ClientId);

    pub const fn win(&self) -> &Window {
        &self.win
    }
}
