use crate::C;
use crate::prelude::*;

/// TODO: rename this to OwnedWindow or something that clearly differentiates
/// that `XDestroyWindow` is called on this one upon `Drop`.
///
/// NOTE: We do NOT implement `clone` for this struct because that would imply
/// that we call `XDestroyWindow` twice.
pub struct Window {
    window: C::Window,
}

impl Drop for Window {
    fn drop(&mut self) {
        if self.window == 0 {
            return;
        }
        unsafe { C::XDestroyWindow(dpy.c(), self.window) };
    }
}

impl Window {
    pub const fn new(window: C::Window) -> Self {
        Self { window }
    }

    pub const fn c(&self) -> C::Window {
        self.window
    }

    pub fn check_win(root: &Window) -> Self {
        let check_win =
            unsafe { C::XCreateSimpleWindow(dpy.c(), root.c(), 0, 0, 1, 1, 0, 0, 0) };
        Self::new(check_win)
    }

    /// A wrapped call to `XGetWindowProperty`.
    pub fn get_property(&self, prop: C::Atom) -> C::Atom {
        let mut atom: C::Atom = 0;
        let mut da: C::Atom = 0; // dummy atom.
        let mut format: c_int = 0;
        let mut n_items = 0;
        let mut dl = 0;
        let mut property = core::ptr::null_mut();

        let result = unsafe {
            C::XGetWindowProperty(
                dpy.c(),
                self.c(),
                prop,
                0,
                core::mem::size_of::<C::Atom>() as c_long,
                0,
                C::XA_ATOM,
                &mut da,
                &mut format,
                &mut n_items,
                &mut dl,
                &mut property,
            )
        };

        if result == C::Success as c_int
            && let Some(property) = XPtr::new(property)
        {
            if n_items > 0 && format == 32 {
                atom = unsafe { *(property.as_ptr() as *mut c_long) } as C::Atom;
            }
        }

        atom
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.window == other.window
    }
}
