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

pub struct Rule {
    pub class: Option<&'static str>,
    pub instance: Option<&'static str>,
    pub title: Option<&'static str>,
    pub tags: u32,
    pub is_floating: bool,
}

impl Rule {
    pub fn is_match(&self, class: &str, instance: &str, client_name: &str) -> bool {
        self.class.map_or(true, |v| class.contains(v))
            && self.instance.map_or(true, |v| instance.contains(v))
            && self.title.map_or(true, |v| client_name.contains(v))
    }
}

pub struct Tag {
    pub name: &'static str,
    pub key: C::KeySym,
}
