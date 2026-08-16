use crate::C;
use crate::prelude::*;

fn _net_atoms(init: Option<()>) -> &'static [C::Atom; Net::COUNT] {
    static ATOMS: OnceLock<[C::Atom; Net::COUNT]> = OnceLock::new();
    if let Some(_) = init {
        let atoms = core::array::from_fn(|i| {
            x11::XInternAtom(Net::ALL[i].as_str(), false).unwrap()
        });
        ATOMS.set(atoms);
    }
    ATOMS.get().unwrap()
}

fn _wm_atoms(init: Option<()>) -> &'static [C::Atom; WM::COUNT] {
    static ATOMS: OnceLock<[C::Atom; WM::COUNT]> = OnceLock::new();
    if let Some(_) = init {
        let atoms = core::array::from_fn(|i| {
            x11::XInternAtom(WM::ALL[i].as_str(), false).unwrap()
        });
        ATOMS.set(atoms);
    }
    ATOMS.get().unwrap()
}

pub fn init_all() {
    _net_atoms(Some(()));
    _wm_atoms(Some(()));
}

#[inline]
pub fn net(idx: Net) -> C::Atom {
    _net_atoms(None)[idx as usize]
}

#[inline]
pub fn net_atoms() -> &'static [C::Atom; Net::COUNT] {
    _net_atoms(None)
}

#[inline]
pub fn wm(idx: WM) -> C::Atom {
    _wm_atoms(None)[idx as usize]
}

#[inline]
pub fn wm_atoms() -> &'static [C::Atom; WM::COUNT] {
    _wm_atoms(None)
}
