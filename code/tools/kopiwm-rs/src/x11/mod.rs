mod display;
mod enums;
mod screen;
mod window;
mod window_attributes;
mod wrapped;

use crate::C;
use crate::prelude::*;

pub mod prelude {
    use super::*;

    pub use super::XPtr;
    pub use display::{Display, dpy};
    pub use enums::*;
    pub use screen::Screen;
    pub use window::Window;
    pub use wrapped::*;
}

#[allow(non_snake_case)]
pub fn XInternAtom(
    name: &str,
    // If only_if_exists is False, the atom is created if it does not exist.
    only_if_exists: bool,
) -> Option<C::Atom> {
    let name = CString::new(name).unwrap();
    let name = name.as_ptr();
    let atom = unsafe { C::XInternAtom(dpy.c(), name, only_if_exists as c_int) };
    // To quote from X11/X11.h:
    // ```c
    // #ifndef None
    // #define None 0L /* universal null resource or null atom */
    // #endif
    // ```
    if atom == C::None as C::Atom {
        return None;
    } else {
        Some(atom)
    }
}

pub fn gettextprop(window: &Window, atom: C::Atom, text: &mut String) -> bool {
    text.clear();
    text.push_str("<broken>");

    let mut name: C::XTextProperty = unsafe { core::mem::zeroed() };
    let result = unsafe { C::XGetTextProperty(dpy.c(), window.c(), &mut name, atom) };
    if result == 0 || name.nitems == 0 {
        return false;
    }

    const XA_STRING: c_ulong = 31; // Hard-coded, inspected from C source.
    if name.encoding == XA_STRING {
        let s = unsafe { core::slice::from_raw_parts(name.value, name.nitems as usize) };
        let s = core::str::from_utf8(s).unwrap();
        text.clear();
        text.push_str(s);
    } else {
        let mut list: *mut *mut c_char = core::ptr::null_mut();
        let mut n = 0;
        let result =
            unsafe { C::XmbTextPropertyToTextList(dpy.c(), &name, &mut list, &mut n) };
        if result >= 0 && n > 0 && !list.is_null() {
            let mut len = 0;
            let first = unsafe { *list } as *const u8;
            loop {
                match unsafe { *first.add(len) } {
                    0 => break,
                    _ => len += 1,
                }
            }
            let s = unsafe { core::slice::from_raw_parts(first, len) };
            let s = core::str::from_utf8(s).unwrap();
            text.clear();
            text.push_str(s);
        }
        unsafe { C::XFreeStringList(list) };
    }
    unsafe { C::XFree(name.value as *mut c_void) };
    true
}

pub struct XPtr<T>(NonNull<T>);

impl<T> XPtr<T> {
    pub const fn new(value: *mut T) -> Option<Self> {
        let Some(value) = NonNull::new(value) else { return None };
        Some(Self(value))
    }

    pub const fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
}

impl<T> Drop for XPtr<T> {
    fn drop(&mut self) {
        unsafe { C::XFree(self.0.as_ptr() as *mut c_void) };
    }
}

impl<T> core::ops::Deref for XPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> core::ops::DerefMut for XPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
