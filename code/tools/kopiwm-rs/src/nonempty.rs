use core::ops::{Index, IndexMut};
use core::slice::{self, SliceIndex};

/// Simply a non-empty `Vec<T>`, with an added feature of having a "selected"
/// element.
#[derive(Debug, Clone)]
pub struct NonEmpty<T> {
    inner: Vec<T>,
    selected_index: usize,
}

#[derive(Debug)]
pub enum NonEmptyError {
    IndexOutOfBounds,
}
type Result<T, E = NonEmptyError> = core::result::Result<T, E>;

#[allow(unused)]
impl<T> NonEmpty<T> {
    pub fn new(initial_value: T) -> Self {
        Self { inner: vec![initial_value], selected_index: 0 }
    }

    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    pub const fn sel_idx(&self) -> usize {
        self.selected_index
    }

    pub fn from_vec(vec: Vec<T>) -> Option<Self> {
        if vec.is_empty() { None } else { Some(Self { inner: vec, selected_index: 0 }) }
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    /// Gets the first element. Guaranteed to exist due to non-empty property.
    pub const fn first(&self) -> &T {
        &self.inner.as_slice()[0]
    }

    /// Gets the last element. Guaranteed to exist due to non-empty property.
    pub fn last(&self) -> &T {
        &self.inner.as_slice()[self.len() - 1]
    }

    /// Gets the first element. Guaranteed to exist due to non-empty property.
    pub const fn first_mut(&mut self) -> &mut T {
        &mut self.inner.as_mut_slice()[0]
    }

    /// Gets the last element. Guaranteed to exist due to non-empty property.
    pub const fn last_mut(&mut self) -> &mut T {
        let n = self.len() - 1;
        &mut self.inner.as_mut_slice()[n]
    }

    /// Gets the currently selected element.
    pub const fn sel(&self) -> &T {
        &self.inner.as_slice()[self.selected_index]
    }

    /// Gets the currently selected element.
    pub const fn sel_mut(&mut self) -> &mut T {
        &mut self.inner.as_mut_slice()[self.selected_index]
    }

    /// Update the selected value
    pub const fn set_sel(&mut self, index: usize) -> Result<()> {
        if index < self.inner.len() {
            Ok(self.selected_index = index)
        } else {
            Err(NonEmptyError::IndexOutOfBounds)
        }
    }

    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> {
        self.inner.iter()
    }

    pub fn find<P: Fn(&T) -> bool>(&self, predicate: P) -> Option<&T> {
        for v in self.inner.iter() {
            if predicate(v) {
                return Some(v);
            }
        }
        None
    }

    pub fn position<P: Fn(&T) -> bool>(&self, predicate: P) -> Option<usize> {
        let mut j = 0;
        while j < self.inner.len() {
            if predicate(&self.inner[j]) {
                return Some(j);
            }
            j += 1;
        }
        None
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

impl<'a, T> IntoIterator for &'a NonEmpty<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut NonEmpty<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}
