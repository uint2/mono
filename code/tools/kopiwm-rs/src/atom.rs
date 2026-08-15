use crate::C;
use crate::prelude::*;

fn _net_atoms(dpy: Option<Display>) -> &'static [C::Atom; Net::COUNT] {
    static ATOMS: OnceLock<[C::Atom; Net::COUNT]> = OnceLock::new();
    if let Some(dpy) = dpy {
        let atoms = core::array::from_fn(|i| {
            x11::XInternAtom(&dpy, Net::ALL[i].as_str(), false).unwrap()
        });
        ATOMS.set(atoms);
    }
    ATOMS.get().unwrap()
}

fn _wm_atoms(dpy: Option<Display>) -> &'static [C::Atom; WM::COUNT] {
    static ATOMS: OnceLock<[C::Atom; WM::COUNT]> = OnceLock::new();
    if let Some(dpy) = dpy {
        let atoms = core::array::from_fn(|i| {
            x11::XInternAtom(&dpy, WM::ALL[i].as_str(), false).unwrap()
        });
        ATOMS.set(atoms);
    }
    ATOMS.get().unwrap()
}

pub fn init_all(dpy: Display) {
    _net_atoms(Some(dpy));
    _wm_atoms(Some(dpy));
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

// TODO: come try this again sometime
// pub fn _dpy(init: Option<()>) -> *mut C::Display {
//     static DISPLAY: OnceLock<Box<C::Display>> = OnceLock::new();
//     if let Some(_) = init {
//         let x = unsafe { C::XOpenDisplay(std::ptr::null()) };
//         let x = unsafe { Box::from_raw(x) };
//         DISPLAY.set(x).unwrap();
//     }
//     Box::clone(DISPLAY.get().unwrap());
//     let x = DISPLAY.get().unwrap().clone();
//     Box::into_raw(x)
// }
