use super::*;

impl Client {
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
            is_urgent: false,
            never_focus: false,
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

    pub fn apply_rules(&mut self, mons: &[Monitor]) {
        self.is_floating.set(false);
        self.tags = 0;
        let mut ch = C::XClassHint {
            res_name: core::ptr::null_mut(),
            res_class: core::ptr::null_mut(),
        };
        unsafe { C::XGetClassHint(dpy.c(), self.win.c(), &mut ch) };
        let class = XPtr::new(ch.res_class);
        let instance = XPtr::new(ch.res_name);

        let class = class.and_then(|v| v.to_str()).unwrap_or("broken");
        let instance = instance.and_then(|v| v.to_str()).unwrap_or("broken");

        for rule in config::RULES {
            if rule.is_match(class, instance, self.name.as_str()) {
                self.is_floating.set(rule.is_floating);
                self.tags = rule.tags;
            }
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
