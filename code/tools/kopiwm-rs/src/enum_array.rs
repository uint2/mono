/*

/// Enum array for enums `K` whose [discriminants](https://doc.rust-lang.org/reference/items/enumerations.html)
/// are dense (or just left unspecified).
pub struct EnumArray<K: EnumCount + ToUsizeIndex, V, const N: usize> {
    data: [Option<V>; N],
    phantom: PhantomData<K>,
}

pub trait ToUsizeIndex {
    fn to_usize_index(&self) -> usize;
}

impl<K: EnumCount + ToUsizeIndex, V, const N: usize> EnumArray<K, V, N> {
    const M: usize = K::COUNT;

    pub const fn new() -> Self {
        // This is guaranteed to be filled with `None` values.
        let data = unsafe { core::mem::zeroed() };
        Self { data, phantom: PhantomData }
    }
}

impl<K: EnumCount + ToUsizeIndex, V, const N: usize> EnumArray<K, V, N> {
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
*/

macro_rules! enum_array {
    ($name:ident, $key:ident) => {
        pub struct $name<T> {
            data: [Option<T>; $key::COUNT],
        }

        impl<T> $name<T> {
            pub const fn new() -> Self {
                // This is guaranteed to be filled with `None` values.
                Self { data: unsafe { core::mem::zeroed() } }
            }

            pub const fn set(&mut self, key: $key, value: T) {
                use core::mem;
                let x = mem::replace(&mut self.data[key as usize], Some(value));
                mem::ManuallyDrop::new(x);
            }

            pub const fn get(&self, key: $key) -> Option<&T> {
                let i = key as usize;
                if i < self.data.len() { self.data[i].as_ref() } else { None }
            }

            pub const fn get_mut(&mut self, key: $key) -> Option<&mut T> {
                let i = key as usize;
                if i < self.data.len() { self.data[i].as_mut() } else { None }
            }
        }
    };
}
