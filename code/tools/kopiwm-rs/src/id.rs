use core::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MonitorId(u32);

impl MonitorId {
    /// Generates a fresh id.
    pub fn new() -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub const fn const_eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ClientId(u32);

impl ClientId {
    /// Generates a fresh id.
    pub fn new() -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub const fn const_eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
