use crate::C;
use crate::prelude::*;

pub enum Arg {
    Int(c_int),
    Uint(c_uint),
    Float(f32),
    Func(fn() -> ()),
    None,
}

pub struct Key {
    pub modifier: c_uint,
    // TODO: Create a KeySym struct that holds a C::KeySym.
    pub keysym: C::KeySym,
    pub func: fn(&Arg) -> (),
    pub arg: Arg,
}

pub struct Button {
    pub click: c_uint,
    pub mask: c_uint,
    pub button: c_uint,
    pub func: fn(&Arg) -> (),
    pub arg: Arg,
}
