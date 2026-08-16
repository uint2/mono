use crate::C;
use crate::prelude::*;

pub struct ClientSizes {
    base: Option<Size>,
    /// Incremental size when resizing.
    inc: Option<Size>,
    max: Option<Size>,
    min: Option<Size>,
    /// Maximum aspect ratio (width / height).
    max_ar: Option<Size>,
    /// Minimum aspect ratio (height / width).
    /// Note that this is the reciprocal of the conventional notion of the
    /// aspect ratio because of how we'll be using it.
    min_ar: Option<Size>,
}

impl ClientSizes {
    pub const fn new() -> Self {
        Self { base: None, inc: None, max: None, min: None, max_ar: None, min_ar: None }
    }
}

pub struct Client {
    pub id: ClientId,
    /// The parent monitor to this client.
    pub mon: MonitorId,
    pub win: Window,
    /// Bitmask of active tags.
    pub tags: u32,
    pub name: String,
    /// Position, current and previous.
    pub pos: Toggle<Rect>,
    pub sz: ClientSizes,
    pub hints_valid: bool,
    /// Border width.
    pub border_width: Toggle<Distance>,
    pub is_fixed: bool,
    pub is_floating: Toggle<bool>,
    pub isurgent: bool,
    pub neverfocus: bool,
    pub isfullscreen: bool,
    /// Next client in the linked list of clients.
    pub next: Option<ClientId>,
    /// Next client in the stacking order. That is, the order in which windows
    /// appear visually. If window A covers window B, or is laid on top of it,
    /// then A is before B in the stacking order.
    pub snext: Option<ClientId>,
}

impl Client {
    getter!(id, ClientId);

    pub fn new(mon: &Monitor, window: Window, attrs: &C::XWindowAttributes) -> Self {
        let rect = Rect::from(attrs);
        let mut pos = Toggle::new(rect);
        pos.set(rect);
        Self {
            id: ClientId::new(),
            mon: mon.id,
            win: window,
            tags: 0,
            name: String::new(),
            pos,
            sz: ClientSizes::new(),
            hints_valid: false,
            border_width: Toggle::new(attrs.border_width as Distance),
            is_fixed: false,
            is_floating: Toggle::new(false),
            isurgent: false,
            neverfocus: false,
            isfullscreen: false,
            next: None,
            snext: None,
        }
    }

    pub fn mon<'a>(&self, monitors: &'a [Monitor]) -> &'a Monitor {
        monitors
            .iter()
            .find(|m| m.id == self.mon)
            .expect("Monitor not found. Dangling client.")
    }

    /// A client is visible if and only if there exists a bit that matches
    /// between its own bitmask, and that of its owning monitor.
    pub fn is_visible(&self, monitors: &[Monitor]) -> bool {
        self.tags & self.mon(monitors).tags != 0
    }

    pub fn update_title(&mut self) {
        let win = &self.win;
        const XA_WM_NAME: C::Atom = 39;
        if !x11::gettextprop(win, atom::net(Net::WMName), &mut self.name) {
            x11::gettextprop(win, XA_WM_NAME, &mut self.name);
        }
    }

    pub fn apply_rules(&mut self, mons: &[Monitor]) {
        self.is_floating.set(false);
        self.tags = 0;
        let mut ch = C::XClassHint {
            res_name: core::ptr::null_mut(),
            res_class: core::ptr::null_mut(),
        };
        unsafe { C::XGetClassHint(dpy.c(), self.win.c(), &mut ch) };
        let class = ffi2::i8_to_str(ch.res_class).unwrap_or("broken");
        let instance = ffi2::i8_to_str(ch.res_name).unwrap_or("broken");

        for rule in config::RULES {
            if rule.is_match(class, instance, self.name.as_str()) {
                self.is_floating.set(rule.is_floating);
                self.tags = rule.tags;
            }
        }
        if !ch.res_name.is_null() {
            unsafe { C::XFree(ch.res_name as *mut c_void) };
        }
        if !ch.res_class.is_null() {
            unsafe { C::XFree(ch.res_class as *mut c_void) };
        }
        if self.tags & config::TAGMASK != 0 {
            self.tags = self.tags & config::TAGMASK;
        } else {
            self.tags = self.mon(mons).tags;
        }
    }

    pub fn width(&self) -> Distance {
        self.pos.width + 2 * *self.border_width
    }

    pub fn height(&self) -> Distance {
        self.pos.height + 2 * *self.border_width
    }

    pub fn configure(&self) {
        let w = self.win.c();
        let xconfigure = C::XConfigureEvent {
            type_: C::ConfigureNotify as c_int,
            display: dpy.c(),
            event: w,
            window: w,
            x: self.pos.x,
            y: self.pos.y,
            width: self.pos.width as c_int,
            height: self.pos.height as c_int,
            border_width: *self.border_width as c_int,
            above: 0,
            override_redirect: 0,
            serial: 0,
            send_event: 0,
        };
        let mut event = C::XEvent { xconfigure };

        unsafe {
            C::XSendEvent(dpy.c(), w, 0, C::StructureNotifyMask as c_long, &mut event)
        };
    }

    /// Update Rust state based on the X window's properties.
    pub fn update_window_type(&mut self) {
        let state = self.get_window_property(atom::net(Net::WMState));
        let wtype = self.get_window_property(atom::net(Net::WMWindowType));

        if state == atom::net(Net::WMFullscreen) {
            self.set_fullscreen(true);
        }
        if wtype == atom::net(Net::WMWindowTypeDialog) {
            self.is_floating.set(true);
        }
    }

    pub fn get_window_property(&self, prop: C::Atom) -> C::Atom {
        let mut atom: C::Atom = 0;
        let mut da: C::Atom = 0; // dummy atom.
        let mut format: c_int = 0;
        let mut n_items = 0;
        let mut dl = 0;
        let mut property = core::ptr::null_mut();

        let result = unsafe {
            C::XGetWindowProperty(
                dpy.c(),
                self.win.c(),
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

        if result == C::Success as c_int && !property.is_null() {
            if n_items > 0 && format == 32 {
                atom = unsafe { *(property as *mut c_long) } as C::Atom;
            }
            unsafe { C::XFree(property as *mut c_void) };
        }
        atom
    }

    /// Update the fullscreen state to `is_fullscreen`.
    pub fn set_fullscreen(&mut self, is_fullscreen: bool) {
        let wmstate = atom::net(Net::WMState);
        let w = self.win.c();
        let pmr = C::PropModeReplace as c_int;

        if is_fullscreen && !self.isfullscreen {
            let mut atom = atom::net(Net::WMFullscreen);
            let atom = core::ptr::from_mut(&mut atom) as *const u8;
            unsafe {
                C::XChangeProperty(dpy.c(), w, wmstate, C::XA_ATOM, 32, pmr, atom, 1)
            };
            self.isfullscreen = true;
            self.is_floating.set(true);
            self.border_width.set(0);
            // TODO: implement these
            // 	resizeclient(c, c->mon->mx, c->mon->my, c->mon->mw, c->mon->mh);
            // 	XRaiseWindow(dpy, c->win);
        } else if !is_fullscreen && self.isfullscreen {
            let mut atom = 0 as C::Atom;
            let atom = core::ptr::from_mut(&mut atom) as *const u8;
            unsafe {
                C::XChangeProperty(dpy.c(), w, wmstate, C::XA_ATOM, 32, pmr, atom, 1)
            };
            self.isfullscreen = false;
            self.is_floating.revert();
            self.border_width.revert();
            // TODO: come back here after all is said and done, and check that
            // this revert in fact reverts back to pre-fullscreen state.
            self.pos.revert();
            // TODO: implement these
            // 	resizeclient(c, c->x, c->y, c->w, c->h);
            // 	arrange(c->mon);
        }
    }
}
