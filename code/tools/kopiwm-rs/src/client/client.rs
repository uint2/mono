use super::*;

impl Client {
    pub fn new(
        window: Window,
        attrs: &C::XWindowAttributes,
        mon: Weak<RwLock<Monitor>>,
    ) -> Self {
        let rect = Rect::from(attrs);
        let mut pos = Toggle::new(rect);
        pos.set(rect);
        Self {
            win: window,
            mon,
            tags: 0,
            name: String::new(),
            pos,
            sz: ClientSizes::new(),
            hints_valid: false,
            border_width: Toggle::new(attrs.border_width as Distance),
            is_fixed: false,
            is_floating: Toggle::new(false),
            is_urgent: false,
            never_focus: false,
            isfullscreen: false,
            next: None,
            snext: None,
        }
    }

    pub fn width(&self) -> Distance {
        self.pos.width + 2 * *self.border_width
    }

    pub fn height(&self) -> Distance {
        self.pos.height + 2 * *self.border_width
    }

    /// A client is visible if and only if there exists a bit that matches
    /// between its own bitmask, and that of its owning monitor.
    pub fn is_visible(&self) -> bool {
        let m = self.mon.upgrade().unwrap();
        let m = m.read().unwrap();
        self.tags & m.tags != 0
    }

    /// Update the fullscreen state to `is_fullscreen`.
    pub fn set_fullscreen(&mut self, is_fullscreen: bool) {
        let wmstate = atom::net(Net::WMState);
        let w = self.win.c();
        const PMR: c_int = C::PropModeReplace as c_int;

        if is_fullscreen && !self.isfullscreen {
            let mut atom = atom::net(Net::WMFullscreen);
            let atom = core::ptr::from_mut(&mut atom) as *const u8;
            unsafe {
                C::XChangeProperty(dpy.c(), w, wmstate, C::XA_ATOM, 32, PMR, atom, 1)
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
                C::XChangeProperty(dpy.c(), w, wmstate, C::XA_ATOM, 32, PMR, atom, 1)
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
