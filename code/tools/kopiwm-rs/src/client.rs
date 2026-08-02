use crate::prelude::*;

pub struct Client {
    id: ClientId,
    mon: MonitorId,
    win: Window,
}

impl Client {
    getter!(win, Window);
    getter!(mon, MonitorId);
    getter!(id, ClientId);
}
