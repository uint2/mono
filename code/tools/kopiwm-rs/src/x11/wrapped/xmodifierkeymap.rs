use crate::C;
use crate::prelude::*;

/// Thinnest wrapper around XModifierKeymap to manage drops.
pub struct XModifierKeymap(NonNull<C::XModifierKeymap>);
make_new!(XModifierKeymap);
