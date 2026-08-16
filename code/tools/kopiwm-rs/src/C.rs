#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![allow(unnecessary_transmutes)]

include!("generated_bindings.rs");

impl Default for XSetWindowAttributes {
    /// Initialize with zeroes to all values. XSetWindowAttributes is a struct
    /// full of integers anyway.
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

mod hard_coded_missing_enums {
    use super::Atom;

    pub const XA_ATOM: Atom = 4;
}

pub use hard_coded_missing_enums::*;
