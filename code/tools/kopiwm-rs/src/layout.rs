use crate::prelude::*;

pub struct Layout {
    symbol: &'static str,
    arrange: Option<fn(&mut App, &mut Monitor) -> ()>,
}

pub const EMPTY_LAYOUT: Layout = Layout { symbol: "><>", arrange: None };
