use crate::prelude::*;

/// Models after a C pointer.
pub struct Ptr<T>(Option<Arc<RwLock<T>>>);

#[allow(unused)]
impl<T> Ptr<T> {
    pub fn new(value: T) -> Self {
        Self(Some(Arc::new(RwLock::new(value))))
    }

    pub const NULL: Self = Self(None);

    pub fn read<'a>(&'a self) -> Option<RwLockReadGuard<'a, T>> {
        let Some(value) = self.0.as_ref() else { return None };
        Some(value.read().unwrap())
    }

    pub fn write<'a>(&'a self) -> Option<RwLockWriteGuard<'a, T>> {
        let Some(value) = self.0.as_ref() else { return None };
        Some(value.write().unwrap())
    }
}
