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
    pub use display::Display;
    pub use enums::*;
    pub use screen::Screen;
    pub use window::Window;
    pub use wrapped::*;
}

#[allow(non_snake_case)]
pub fn XInternAtom(
    dpy: &Display,
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
