use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;

/// Simply a non-empty `Vec<T>`.
#[derive(Debug, Clone)]
pub struct NonEmpty<T> {
    inner: Vec<T>,
}

#[allow(unused)]
impl<T> NonEmpty<T> {
    pub fn new(initial_value: T) -> Self {
        Self { inner: vec![initial_value] }
    }

    pub fn from_vec(vec: Vec<T>) -> Option<Self> {
        if vec.is_empty() { None } else { Some(Self { inner: vec }) }
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    /// Gets the first element. Guaranteed to exist due to non-empty property.
    pub fn first(&self) -> &T {
        self.inner.first().unwrap()
    }

    /// Gets the last element. Guaranteed to exist due to non-empty property.
    pub fn last(&self) -> &T {
        self.inner.last().unwrap()
    }

    /// Gets the first element. Guaranteed to exist due to non-empty property.
    pub fn first_mut(&mut self) -> &mut T {
        self.inner.first_mut().unwrap()
    }

    /// Gets the last element. Guaranteed to exist due to non-empty property.
    pub fn last_mut(&mut self) -> &mut T {
        self.inner.last_mut().unwrap()
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for NonEmpty<T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for NonEmpty<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
