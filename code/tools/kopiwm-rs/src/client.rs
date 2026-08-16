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
    id: ClientId,
    /// The parent monitor to this client.
    pub mon: MonitorId,
    pub win: Window,
    /// Bitmask of active tags.
    pub tags: u32,
    name: String,
    /// Position, current and previous.
    pos: Toggle<Rect>,
    sz: ClientSizes,
    hints_valid: bool,
    /// Border width.
    border_width: Toggle<Distance>,
    is_fixed: bool,
    is_floating: Toggle<bool>,
    isurgent: bool,
    neverfocus: bool,
    isfullscreen: bool,
    /// Next client in the linked list of clients.
    next: Option<ClientId>,
    /// Next client in the stacking order. That is, the order in which windows
    /// appear visually. If window A covers window B, or is laid on top of it,
    /// then A is before B in the stacking order.
    snext: Option<ClientId>,
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
}
