use crate::C;
use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Display(NonNull<C::Display>);

impl Display {
    pub const fn c(&self) -> *mut C::Display {
        self.0.as_ptr()
    }

    /// The XOpenDisplay function returns a Display structure that serves as
    /// the connection to the X server and that contains all the information
    /// about that X server. XOpenDisplay connects your application to the X
    /// server through TCP or DECnet communications protocols, or through some
    /// local inter-process communication protocol. If the hostname is a host
    /// machine name and a single colon (:) separates the hostname and display
    /// number, XOpenDisplay connects using TCP streams. If the hostname is not
    /// specified, Xlib uses whatever it believes is the fastest transport. If
    /// the hostname is a host machine name and a double colon (::) separates
    /// the hostname and display number, XOpenDisplay connects using DECnet. A
    /// single X server can support any or all of these transport mechanisms
    /// simultaneously. A particular Xlib implementation can support many more
    /// of these transport mechanisms.
    ///
    /// If successful, XOpenDisplay returns a pointer to a Display structure,
    /// which is defined in <X11/Xlib.h>. If XOpenDisplay does not succeed, it
    /// returns NULL. After a successful call to XOpenDisplay, all of the
    /// screens in the display can be used by the client. The screen number
    /// specified in the display_name argument is returned by the DefaultScreen
    /// macro (or the XDefaultScreen function). You can access elements of the
    /// Display and Screen structures only by using the information macros or
    /// functions. For information about using macros and functions to obtain
    /// information from the Display structure, see section 2.2.1.
    ///
    /// source: https://x.org/releases/X11R7.7/doc/man/man3/XOpenDisplay.3.xhtml
    pub fn open() -> Option<Self> {
        let dpy = unsafe { C::XOpenDisplay(std::ptr::null()) };
        NonNull::new(dpy).map(Self)
    }

    /// The XCloseDisplay function closes the connection to the X server for
    /// the display specified in the Display structure and destroys all
    /// windows, resource IDs (Window, Font, Pixmap, Colormap, Cursor, and
    /// GContext), or other resources that the client has created on this
    /// display, unless the close-down mode of the resource has been changed
    /// (see XSetCloseDownMode). Therefore, these windows, resource IDs, and
    /// other resources should never be referenced again or an error will be
    /// generated. Before exiting, you should call XCloseDisplay explicitly so
    /// that any pending errors are reported as XCloseDisplay performs a final
    /// XSync operation.
    ///
    /// XCloseDisplay can generate a BadGC error.
    ///
    /// source: https://x.org/releases/X11R7.7/doc/man/man3/XOpenDisplay.3.xhtml
    pub fn close(self) {
        unsafe { C::XCloseDisplay(self.c()) };
    }

    /// The XSelectInput function requests that the X server report the events
    /// associated with the specified event mask. Initially, X will not report any
    /// of these events. Events are reported relative to a window. If a window is
    /// not interested in a device event, it usually propagates to the closest
    /// ancestor that is interested, unless the do_not_propagate mask prohibits it.
    ///
    /// Setting the event-mask attribute of a window overrides any previous call
    /// for the same window but not for other clients. Multiple clients can select
    /// for the same events on the same window with the following restrictions:
    ///
    /// * Multiple clients can select events on the same window because their event
    ///   masks are disjoint. When the X server generates an event, it reports it to all
    ///   interested clients.
    ///
    /// * Only one client at a time can select CirculateRequest, ConfigureRequest, or
    ///   MapRequest events, which are associated with the event mask
    ///   SubstructureRedirectMask.
    ///
    /// * Only one client at a time can select a ResizeRequest event, which is
    ///   associated with the event mask ResizeRedirectMask.
    ///
    /// * Only one client at a time can select a ButtonPress event, which is associated
    ///   with the event mask ButtonPressMask.
    ///
    /// The server reports the event to all interested clients.
    ///
    /// XSelectInput can generate a BadWindow error.
    ///
    /// source: https://x.org/releases/X11R7.7/doc/man/man3/XSendEvent.3.xhtml
    pub fn select_input(&self, window: Window, event_mask: c_long) {
        unsafe { C::XSelectInput(self.c(), window.c(), event_mask) };
    }

    /// The XSync function flushes the output buffer and then waits until all
    /// requests have been received and processed by the X server. Any errors
    /// generated must be handled by the error handler. For each protocol error
    /// received by Xlib, XSync calls the client application's error handling
    /// routine. Any events generated by the server are enqueued into the library's
    /// event queue.
    ///
    /// Finally, if you passed False, XSync does not discard the events in the
    /// queue. If you passed True, XSync discards all events in the queue,
    /// including those events that were on the queue before XSync was called.
    /// Client applications seldom need to call XSync.
    ///
    /// source: https://x.org/releases/X11R7.7/doc/man/man3/XFlush.3.xhtml
    pub fn sync(&self, discard: bool) {
        // According to the docs in the source, the c_int output is only important
        // in the other functions documented on that html page, but not XSync. So
        // we discard it.
        unsafe { C::XSync(self.c(), discard as c_int) };
    }

    pub fn default_root_window(&self) -> Window {
        let window = unsafe { C::XDefaultRootWindow(self.c()) };
        Window::new(*self, window)
    }

    pub fn default_screen(&self) -> Screen {
        Screen::from_c(unsafe { C::XDefaultScreen(self.c()) })
    }

    pub fn default_visual(&self, screen: Screen) -> *mut C::Visual {
        unsafe { C::XDefaultVisual(self.c(), screen.c()) }
    }

    pub fn default_colormap(&self, screen: Screen) -> C::Colormap {
        unsafe { C::XDefaultColormap(self.c(), screen.c()) }
    }

    pub fn display_width(&self, screen: Screen) -> c_int {
        unsafe { C::XDisplayWidth(self.c(), screen.c()) }
    }

    pub fn display_height(&self, screen: Screen) -> c_int {
        unsafe { C::XDisplayHeight(self.c(), screen.c()) }
    }

    pub fn display_size(&self, screen: Screen) -> Size<c_int> {
        Size::new(self.display_width(screen), self.display_height(screen))
    }

    pub fn default_depth(&self, screen: Screen) -> c_int {
        unsafe { C::XDefaultDepth(self.c(), screen.c()) }
    }

    pub fn create_pixmap(
        &self,
        window: &Window,
        dimensions: Size<c_uint>,
        depth: c_uint,
    ) -> C::Pixmap {
        let w = dimensions.width;
        let h = dimensions.height;
        unsafe { C::XCreatePixmap(self.c(), window.c(), w, h, depth) }
    }

    pub fn free_pixmap(&self, pixmap: C::Pixmap) {
        unsafe { C::XFreePixmap(self.c(), pixmap) };
    }

    pub fn create_graphics_ctx(&self, window: &Window) -> C::GC {
        unsafe { C::XCreateGC(self.c(), window.c(), 0, ptr::null_mut()) }
    }

    pub fn free_graphics_ctx(&self, graphics_ctx: C::GC) {
        unsafe { C::XFreeGC(self.c(), graphics_ctx) };
    }

    pub fn set_line_attributes(
        &self,
        graphics_ctx: C::GC,
        line_width: c_uint,
        line_style: LineStyle,
        cap_style: CapStyle,
        join_style: JoinStyle,
    ) {
        unsafe {
            C::XSetLineAttributes(
                self.c(),
                graphics_ctx,
                line_width,
                line_style.c(),
                cap_style.c(),
                join_style.c(),
            );
        }
    }

    pub fn xft_font_open_name(&self, screen: Screen, font_name: &str) -> Option<XftFont> {
        let font = unsafe {
            C::XftFontOpenName(self.c(), screen.c(), font_name.c_str().as_ptr())
        };
        XftFont::new(*self, font)
    }

    pub fn xft_font_open_pattern(&self, pattern: &FcPattern) -> Option<XftFont> {
        let font = unsafe { C::XftFontOpenPattern(self.c(), pattern.c()) };
        XftFont::new(*self, font)
    }

    pub fn xft_text_extents_utf8(&self, font: &Font, text: &str) -> Size<c_int> {
        let text = text.c_str();
        let mut extents: C::XGlyphInfo = unsafe { core::mem::zeroed() };
        unsafe {
            C::XftTextExtentsUtf8(
                self.c(),
                font.xfont().c(),
                text.as_ptr() as *const u8,
                text.count_bytes() as c_int,
                &mut extents,
            )
        };
        Size::new(extents.xOff as c_int, font.height())
    }

    pub fn get_modifier_mapping(&self) -> Option<XModifierKeymap> {
        let x = unsafe { C::XGetModifierMapping(self.c()) };
        XModifierKeymap::new(x)
    }

    pub fn keysym_to_keycode(&self, keysym: C::KeySym) -> C::KeyCode {
        unsafe { C::XKeysymToKeycode(self.c(), keysym) }
    }

    pub fn keycode_to_keysym(&self, keycode: C::KeyCode) -> C::KeySym {
        const GROUP: c_int = 0;
        const LEVEL: c_int = 0;
        unsafe { C::XkbKeycodeToKeysym(self.c(), keycode, GROUP, LEVEL) }
    }
}
