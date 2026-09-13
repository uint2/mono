use crate::prelude::*;

/// Models after a C pointer.
pub struct Ptr<T>(Option<Arc<RwLock<T>>>);

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Self(self.0.as_ref().map(Arc::clone))
    }
}

impl<T> PartialEq for Ptr<T> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Some(lhs), Some(rhs)) => Arc::ptr_eq(lhs, rhs),
            _ => false,
        }
    }
}
impl<T> Eq for Ptr<T> {}

#[allow(unused)]
impl<T> Ptr<T> {
    pub fn new(value: T) -> Self {
        Self(Some(Arc::new(RwLock::new(value))))
    }

    pub const NULL: Self = Self(None);

    pub fn r(&self) -> RwLockReadGuard<'_, T> {
        self.read().unwrap()
    }

    pub fn w(&self) -> RwLockWriteGuard<'_, T> {
        self.write().unwrap()
    }

    pub fn read(&self) -> Option<RwLockReadGuard<'_, T>> {
        let Some(value) = self.0.as_ref() else { return None };
        Some(value.read().unwrap())
    }

    pub fn write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        let Some(value) = self.0.as_ref() else { return None };
        Some(value.write().unwrap())
    }

    pub fn update(&mut self, value: Arc<RwLock<T>>) {
        self.0 = Some(value);
    }

    pub const fn is_null(&self) -> bool {
        self.0.is_none()
    }
}
