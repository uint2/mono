use crate::prelude::*;

/// Models after a C pointer.
pub struct Ptr<T>(Option<Arc<RwLock<T>>>);

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Self(self.0.as_ref().map(Arc::clone))
    }
}

#[allow(unused)]
impl<T> Ptr<T> {
    pub fn new(value: T) -> Self {
        Self(Some(Arc::new(RwLock::new(value))))
    }

    pub const NULL: Self = Self(None);

    pub fn r<'a>(&'a self) -> RwLockReadGuard<'a, T> {
        self.read().unwrap()
    }

    pub fn w<'a>(&'a self) -> RwLockWriteGuard<'a, T> {
        self.write().unwrap()
    }

    pub fn read<'a>(&'a self) -> Option<RwLockReadGuard<'a, T>> {
        let Some(value) = self.0.as_ref() else { return None };
        Some(value.read().unwrap())
    }

    pub fn write<'a>(&'a self) -> Option<RwLockWriteGuard<'a, T>> {
        let Some(value) = self.0.as_ref() else { return None };
        Some(value.write().unwrap())
    }

    pub const fn is_null(&self) -> bool {
        self.0.is_none()
    }
}
