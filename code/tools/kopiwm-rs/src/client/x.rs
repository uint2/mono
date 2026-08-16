use super::*;

impl Client {
    pub fn update_title(&mut self) {
        let win = &self.win;
        const XA_WM_NAME: C::Atom = 39;
        if !x11::gettextprop(win, atom::net(Net::WMName), &mut self.name) {
            x11::gettextprop(win, XA_WM_NAME, &mut self.name);
        }
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
        let state = self.win.get_property(atom::net(Net::WMState));
        let wtype = self.win.get_property(atom::net(Net::WMWindowType));

        if state == atom::net(Net::WMFullscreen) {
            self.set_fullscreen(true);
        }
        if wtype == atom::net(Net::WMWindowTypeDialog) {
            self.is_floating.set(true);
        }
    }

    pub fn update_size_hints(&mut self) {
        let mut msize = 0;
        let mut size: C::XSizeHints = unsafe { core::mem::zeroed() };
        let result =
            unsafe { C::XGetWMNormalHints(dpy.c(), self.win.c(), &mut size, &mut msize) };
        if result == 0 {
            // size is uninitialized, ensure that size.flags aren't used.
            size.flags = C::PSize as c_long;
        }

        self.sz.update_base(&size);
        self.sz.update_inc(&size);
        self.sz.update_max(&size);
        self.sz.update_min(&size);

        if size.flags as c_uint & C::PAspect != 0 {
            self.sz.min_ar = size.min_aspect.y as f64 / size.min_aspect.x as f64;
            self.sz.max_ar = size.max_aspect.x as f64 / size.max_aspect.y as f64;
        } else {
            self.sz.min_ar = 0.;
            self.sz.max_ar = 0.;
        }
        self.is_fixed = self.sz.is_fixed();
        self.hints_valid = true;
    }
}
