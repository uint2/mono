use crate::prelude::*;

/// Models after a C pointer.
pub struct Ptr<T>(Arc<RwLock<T>>);

#[allow(unused)]
impl<T> Ptr<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }

    pub fn read<'a>(&'a self) -> RwLockReadGuard<'a, T> {
        self.0.read().unwrap()
    }

    pub fn write<'a>(&'a self) -> RwLockWriteGuard<'a, T> {
        self.0.write().unwrap()
    }
}
