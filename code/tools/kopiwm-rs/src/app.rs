#![allow(unused)]

use crate::C;
use crate::prelude::*;
use config::{Coordinate, Distance};

/// C: type for Coordinates.
/// D: type for Distance.
pub struct App {
    pub root: OwnedWindow,
    screen: Screen,
    /// Screen size.
    /// Apparently dwm updates this in `void configurenotify(XEvent *)`, and
    /// that's probably how multipe monitors are supported.
    s: Size,
    lrpad: Distance,
    bar_height: Distance,
    cursors: CursorStateArray<Cursor>,
    colors: WindowColorStateArray<WindowColors<XftColor>>,
    status_text: String,
    numlockmask: NumLockMask,
    fonts: Fonts,
    running: bool,

    /// Owned list of moitors. It is guaranteed that for the lifetime of `Self`,
    /// this list is non-empty.
    monitors: NonEmpty<Arc<RwLock<Monitor>>>,
    selmon: Arc<RwLock<Monitor>>,
}

pub struct AppInitParams {
    pub screen: Screen,
    pub s: Size,
    pub lrpad: Distance,
    pub monitors: NonEmpty<Arc<RwLock<Monitor>>>,
    pub cursors: CursorStateArray<Cursor>,
    pub colors: WindowColorStateArray<WindowColors<XftColor>>,
    pub numlockmask: NumLockMask,
    pub fonts: Fonts,
}

impl App {
    pub fn new(root: OwnedWindow, params: AppInitParams) -> Self {
        Self {
            root,
            selmon: Arc::clone(params.monitors.first()),
            screen: params.screen,
            s: params.s,
            lrpad: params.lrpad,
            bar_height: config::BAR_HEIGHT,
            monitors: params.monitors,
            cursors: params.cursors,
            colors: params.colors,
            status_text: String::new(),
            numlockmask: params.numlockmask,
            fonts: params.fonts,
            running: true,
        }
    }
}

/// Getters.
impl App {}

/// Core Logic.
impl App {
    pub fn updategeom(&mut self) -> bool {
        let mut dirty = false;
        let m = Arc::clone(self.monitors.first());
        let mut m = m.write().unwrap();
        if m.m.width != self.s.width || m.m.height != self.s.height {
            dirty = true;
            m.m.set_size(self.s);
            m.w.set_size(self.s);
            m.update_bar_pos(self.bar_height);
        }
        if dirty {
            let mon = self.window_to_monitor(self.root.as_ref());
            self.selmon = Arc::clone(mon);
        }
        dirty
    }

    /// Gets the index of the monitor that contains the window.
    /// Fallback: currently selected monitor.
    pub fn window_to_monitor(&self, window: Window) -> &Arc<RwLock<Monitor>> {
        if self.root.eq(&window) {
            if let Some(loc) = self.root.as_ref().get_ptr() {
                let r = Rect { x: loc.x, y: loc.y, width: 1, height: 1 };
                // To guarantee a return value, we deviate a tad from dwm's
                // behaviour and return `selmon` if nothing is found.
                return self.rect_to_monitor(&r);
            }
        }

        if let Some(m) = self
            .monitors
            .find(|m| m.read().unwrap().bar_window().map_or(false, |w| w == window))
        {
            return m;
        }

        for (idx, m) in self.monitors.iter().enumerate() {
            let mon = m.read().unwrap();
            if mon.clients.iter().any(|c| c.read().unwrap().win == window) {
                return m;
            }
        }

        &self.selmon
    }

    /// Searches all clients in the current App instance for one that manages
    /// window `window`.
    fn window_to_client(&self, window: Window) -> Option<Arc<RwLock<Client>>> {
        for (m_idx, m) in self.monitors.iter().enumerate() {
            let m = m.read().unwrap();
            for (c_idx, c) in m.clients.iter().enumerate() {
                let c = Arc::clone(c);
                if c.read().unwrap().win == window {
                    return Some(c);
                }
            }
        }
        None
    }

    /// Searches the list of monitors for the one with the biggest intersection
    /// with `self` (using Monitor.w), and returns the index of that one.
    ///
    /// If nothing is found, return the currently selected monitor.
    pub fn rect_to_monitor(&self, rect: &Rect) -> &Arc<RwLock<Monitor>> {
        let mut candidate = &self.selmon;
        let mut max_area = 0;
        for m in &self.monitors {
            let area = rect.intersect(&m.read().unwrap().w);
            if max_area < area {
                max_area = area;
                candidate = m;
            }
        }
        candidate
    }

    /// (dwm) static void grabkeys(void);
    pub fn grabkeys(&mut self) {
        let root = self.root.c();
        self.numlockmask.update();

        let mut start: c_int = 0;
        let mut end: c_int = 0;
        let mut skip: c_int = 0;

        unsafe {
            C::XUngrabKey(dpy.c(), C::AnyKey as c_int, C::AnyModifier as c_uint, root);
            C::XDisplayKeycodes(dpy.c(), &mut start, &mut end);
        };

        let syms: *mut C::KeySym = unsafe {
            C::XGetKeyboardMapping(
                dpy.c(),
                start as C::KeyCode,
                end - start + 1,
                &mut skip,
            )
        };
        let Some(syms) = XPtr::new(syms) else { return };

        for keycode in start..=end {
            for key in &config::KEYS {
                let offset = (keycode - start) * skip;
                let keysym = unsafe { *syms.get(offset as usize) };
                if key.keysym == keysym {
                    self.numlockmask.grabkey(self.root.as_ref(), key, keycode);
                }
            }
        }
    }

    /// (dwm) static void grabbuttons(Client *c, int focused);
    pub fn grabbuttons(&mut self, window: Window, focused: bool) {
        let root = self.root.c();
        self.numlockmask.update();

        const BUTTONMASK: c_uint =
            C::ButtonPressMask as c_uint | C::ButtonReleaseMask as c_uint;

        unsafe {
            C::XUngrabButton(
                dpy.c(),
                C::AnyButton as c_uint,
                C::AnyModifier as c_uint,
                root,
            );
        };
        if (!focused) {
            unsafe {
                C::XGrabButton(
                    dpy.c(),
                    C::AnyButton as c_uint,
                    C::AnyModifier as c_uint,
                    root,
                    C::False as c_int,
                    BUTTONMASK,
                    C::GrabModeSync as c_int,
                    C::GrabModeSync as c_int,
                    C::None as C::Window,
                    C::None as C::Cursor,
                );
            }
        }
        for button in config::BUTTONS {
            let Clk::ClientWin = button.click else { continue };
            for modifier in self.numlockmask.modifiers() {
                unsafe {
                    C::XGrabButton(
                        dpy.c(),
                        button.button,
                        button.mask | modifier,
                        window.c(),
                        C::False as c_int,
                        BUTTONMASK,
                        C::GrabModeAsync as c_int,
                        C::GrabModeSync as c_int,
                        C::None as C::Window,
                        C::None as C::Cursor,
                    );
                }
            }
        }
    }

    /// (dwm) static void applyrules(Client *c);
    pub fn apply_rules(&mut self, client: &Arc<RwLock<Client>>) {
        let mut c = client.write().unwrap();
        c.is_floating.set(false);
        c.tags = 0;
        let mut ch = C::XClassHint {
            res_name: core::ptr::null_mut(),
            res_class: core::ptr::null_mut(),
        };
        unsafe { C::XGetClassHint(dpy.c(), c.win.c(), &mut ch) };
        let class = XPtr::new(ch.res_class);
        let instance = XPtr::new(ch.res_name);

        let class = class.and_then(|v| v.to_str()).unwrap_or("broken");
        let instance = instance.and_then(|v| v.to_str()).unwrap_or("broken");

        for rule in config::RULES {
            if rule.is_match(class, instance, c.name.as_str()) {
                c.is_floating.set(rule.is_floating);
                c.tags = rule.tags;
            }
        }
        if c.tags & config::TAGMASK != 0 {
            c.tags = c.tags & config::TAGMASK;
        } else {
            c.tags = c.mon.upgrade().unwrap().read().unwrap().tags;
        }
    }

    pub fn fit_in_screen(&mut self, client: &Arc<RwLock<Client>>) {
        let mut c = client.write().unwrap();
        let m = c.mon.upgrade().unwrap();
        let m = m.read().unwrap();

        let c_width = c.width();
        let c_height = c.height();
        let mut r = c.pos.as_mut();

        // If client is too far right, shift it left.
        if (r.x + c_width as Coordinate > m.w.r()) {
            r.x = m.w.r() - c_width as Coordinate;
        }
        // If client is too far down, shift it up.
        if (r.y + c_height as Coordinate > m.w.b()) {
            r.y = m.w.b() - c_height as Coordinate;
        }
        r.x = Coordinate::max(r.x, m.w.x); // If client is too far left, truncate it.
        r.y = Coordinate::max(r.y, m.w.y); // If client is too far up, truncate it.
    }

    fn is_selected(&self, client: &Arc<RwLock<Client>>) -> bool {
        let Ok(selmon) = self.selmon.read() else { return false };
        let Some(ref sel) = selmon.sel else { return false };
        Arc::ptr_eq(sel, client)
    }

    /// (dwm) static void updatewmhints(Client *c);
    fn update_wm_hints(&mut self, client: &Arc<RwLock<Client>>) {
        let win = client.read().unwrap().win.c();
        let hints = unsafe { C::XGetWMHints(dpy.c(), win) };
        let Some(mut hints) = XPtr::new(hints) else { return };

        let urgency_hint = hints.flags & C::XUrgencyHint as c_long != 0;

        if self.is_selected(&client) {
            hints.flags &= !C::XUrgencyHint as c_long;
            unsafe { C::XSetWMHints(dpy.c(), win, hints.as_ptr()) };
        } else {
            client.write().unwrap().is_urgent = urgency_hint;
        }

        if hints.flags & C::InputHint as c_long != 0 {
            client.write().unwrap().never_focus = hints.input == 0;
        } else {
            client.write().unwrap().never_focus = false;
        }
    }

    /// (dwm) static void manage(Window w, XWindowAttributes *wa);
    pub fn manage(&mut self, window: Window, attrs: &C::XWindowAttributes) {
        let mut c = Client::new(window, attrs, Arc::downgrade(&self.selmon));
        let w = c.win.c();
        c.update_title();
        let mut trans = c.win.get_transient_for_hint();
        let c = Arc::new(RwLock::new(c));
        match trans.and_then(|t| self.window_to_client(t)) {
            Some(t) => {
                let t = t.read().unwrap();
                c.write().unwrap().mon = Weak::clone(&t.mon);
                c.write().unwrap().tags = t.tags;
            }
            None => {
                c.write().unwrap().mon = Arc::downgrade(&self.selmon);
                self.apply_rules(&c);
            }
        };

        self.fit_in_screen(&c);
        c.write().unwrap().border_width.set(config::BORDER_PX);

        let mut wc: C::XWindowChanges = unsafe { core::mem::zeroed() };
        wc.border_width = *c.read().unwrap().border_width as c_int;

        unsafe { C::XConfigureWindow(dpy.c(), w, C::CWBorderWidth, &mut wc) };
        let color = self.colors[WindowColorState::Normal].border.pixel();
        unsafe { C::XSetWindowBorder(dpy.c(), w, color) };

        c.read().unwrap().configure(); // propagates border_width, if size doesn't change
        c.write().unwrap().update_window_type();
        c.write().unwrap().update_size_hints();
        self.update_wm_hints(&c);
        unsafe {
            C::XSelectInput(
                dpy.c(),
                w,
                C::EnterWindowMask as c_long
                    | C::FocusChangeMask as c_long
                    | C::PropertyChangeMask as c_long
                    | C::StructureNotifyMask as c_long,
            );
        };
        self.grabbuttons(c.read().unwrap().win, false);
        if !*c.read().unwrap().is_floating {
            let is_fixed = c.read().unwrap().is_fixed;
            c.write().unwrap().is_floating.set(trans.is_some() || is_fixed);
        }
        if *c.read().unwrap().is_floating {
            unsafe { C::XRaiseWindow(dpy.c(), c.read().unwrap().win.c()) };
        }

        // Skip these calls because the client happens to be in the right
        // position already.
        // attach(c); attachstack(c);

        // TODO: continue off from here
        // XChangeProperty(dpy, root, netatom[NetClientList], XA_WINDOW, 32, PropModeAppend,
        // 	(unsigned char *) &(c->win), 1);
        // XMoveResizeWindow(dpy, c->win, c->x + 2 * sw, c->y, c->w, c->h); /* some windows require this */
        // setclientstate(c, NormalState);
        // if (c->mon == selmon)
        // 	unfocus(selmon->sel, 0);
        // c->mon->sel = c;
        // arrange(c->mon);
        // XMapWindow(dpy, c->win);
        // focus(NULL);
    }
}
