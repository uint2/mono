use crate::prelude::*;

/// Enum array for enums `K` whose [discriminants](https://doc.rust-lang.org/reference/items/enumerations.html)
/// are dense (or just left unspecified)..
pub struct EnumArray<K: EnumCount + ToUsizeIndex, V> {
    data: Vec<Option<V>>,
    phantom: PhantomData<K>,
}

pub trait ToUsizeIndex {
    fn to_usize_index(&self) -> usize;
}

impl<K: EnumCount + ToUsizeIndex, V> EnumArray<K, V> {
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(K::COUNT);
        data.resize_with(K::COUNT, || None);
        Self { data, phantom: PhantomData }
    }
}

impl<K: EnumCount + ToUsizeIndex, V> EnumArray<K, V> {
    pub fn set(&mut self, key: K, value: V) {
        self.data[key.to_usize_index()] = Some(value);
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let Some(v) = self.data.get(key.to_usize_index()) else { return None };
        v.as_ref()
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let Some(v) = self.data.get_mut(key.to_usize_index()) else { return None };
        v.as_mut()
    }
}
