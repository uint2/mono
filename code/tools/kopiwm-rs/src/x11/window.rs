use crate::C;
use crate::prelude::*;

/// NOTE: We do NOT implement `clone` for this struct because that would imply
/// that we call `XDestroyWindow` twice.
pub struct OwnedWindow(Window);

impl Drop for OwnedWindow {
    fn drop(&mut self) {
        if self.0.0 == 0 {
            return;
        }
        unsafe { C::XDestroyWindow(dpy.c(), self.0.0) };
    }
}

impl OwnedWindow {
    pub const fn as_ref(&self) -> Window {
        self.0
    }

    pub const fn c(&self) -> C::Window {
        self.0.0
    }
}

#[rustfmt::skip]
impl PartialEq<Window> for OwnedWindow { fn eq(&self, other: &Window) -> bool { self.0.0 == other.0 } }
#[rustfmt::skip]
impl PartialEq<OwnedWindow> for Window { fn eq(&self, other: &OwnedWindow) -> bool { self.0 == other.0.0 } }

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Window(C::Window);

impl Window {
    pub const fn new(window: C::Window) -> Self {
        Self(window)
    }

    pub const fn to_owned_window(self) -> OwnedWindow {
        OwnedWindow(self)
    }

    pub const fn c(&self) -> C::Window {
        self.0
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

    /// (dwm) static int getrootptr(int *x, int *y);
    pub fn get_ptr(&self) -> Option<Loc> {
        let mut root_return: C::Window = 0;
        let mut child_return: C::Window = 0;
        let mut root_x_return: Coordinate = 0;
        let mut root_y_return: Coordinate = 0;
        let mut win_x_return: Coordinate = 0;
        let mut win_y_return: Coordinate = 0;
        let mut mask_return: c_uint = 0;

        let result = unsafe {
            C::XQueryPointer(
                dpy.c(),
                self.c(),
                &mut root_return,
                &mut child_return,
                &mut root_x_return,
                &mut root_y_return,
                &mut win_x_return,
                &mut win_y_return,
                &mut mask_return,
            )
        };
        match result {
            // If XQueryPointer returns False, the pointer is not on the same
            // screen as the specified window, and XQueryPointer returns None to
            // child_return and zero to win_x_return and win_y_return.
            0 => None,
            // If XQueryPointer returns True, the pointer coordinates returned
            // to win_x_return and win_y_return are relative to the origin of
            // the specified window. In this case, XQueryPointer returns the
            // child that contains the pointer, if any, or else None to
            // child_return.
            _ => Some(Loc::new(win_x_return, win_y_return)),
        }
    }

    pub fn get_transient_for_hint(&self) -> Option<Self> {
        let mut t: C::Window = C::None as C::Window;
        match unsafe { C::XGetTransientForHint(dpy.c(), self.c(), &mut t) } {
            0 => None,
            _ => Some(Self(t)),
        }
    }
}
