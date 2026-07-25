use std::ffi::CString;

use crate::c as C;

pub struct Display(*mut C::Display);

impl Display {
    /// The XOpenDisplay function returns a Display structure that serves as the
    /// connection to the X server and that contains all the information about that
    /// X server. XOpenDisplay connects your application to the X server through
    /// TCP or DECnet communications protocols, or through some local inter-process
    /// communication protocol. If the hostname is a host machine name and a single
    /// colon (:) separates the hostname and display number, XOpenDisplay connects
    /// using TCP streams. If the hostname is not specified, Xlib uses whatever it
    /// believes is the fastest transport. If the hostname is a host machine name
    /// and a double colon (::) separates the hostname and display number,
    /// XOpenDisplay connects using DECnet. A single X server can support any or
    /// all of these transport mechanisms simultaneously. A particular Xlib
    /// implementation can support many more of these transport mechanisms.
    ///
    /// If successful, XOpenDisplay returns a pointer to a Display structure, which
    /// is defined in <X11/Xlib.h>. If XOpenDisplay does not succeed, it returns
    /// NULL. After a successful call to XOpenDisplay, all of the screens in the
    /// display can be used by the client. The screen number specified in the
    /// display_name argument is returned by the DefaultScreen macro (or the
    /// XDefaultScreen function). You can access elements of the Display and Screen
    /// structures only by using the information macros or functions. For
    /// information about using macros and functions to obtain information from the
    /// Display structure, see section 2.2.1.
    ///
    /// source: https://x.org/releases/X11R7.7/doc/man/man3/XOpenDisplay.3.xhtml
    pub fn open() -> Self {
        Self(unsafe { C::XOpenDisplay(std::ptr::null()) })
    }
}
