#![allow(unused)]

use crate::C;
use crate::prelude::*;
use config::{Coordinate, Distance};

/// C: type for Coordinates.
/// D: type for Distance.
pub struct App {
    pub root: Window,
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
    monitors: NonEmpty<Monitor>,
}

pub struct AppInitParams {
    pub screen: Screen,
    pub s: Size,
    pub lrpad: Distance,
    pub monitors: NonEmpty<Monitor>,
    pub cursors: CursorStateArray<Cursor>,
    pub colors: WindowColorStateArray<WindowColors<XftColor>>,
    pub numlockmask: NumLockMask,
    pub fonts: Fonts,
}

impl App {
    pub fn new(root: Window, params: AppInitParams) -> Self {
        Self {
            root,
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
impl App {
    pub const fn selmon(&self) -> &Monitor {
        self.monitors.sel()
    }
}

/// Core Logic.
impl App {
    pub fn updategeom(&mut self) -> bool {
        let mut dirty = false;
        let m = self.monitors.first_mut();
        if m.m.width != self.s.width || m.m.height != self.s.height {
            dirty = true;
            m.m.set_size(self.s);
            m.w.set_size(self.s);
            m.update_bar_pos(self.bar_height);
        }
        if dirty {
            let id = self.window_to_monitor(&self.root);
            let idx = self.monitors.position(|v| v.id == id).unwrap();
            self.monitors.set_sel(idx);
        }
        dirty
    }

    /// Finds the monitor that contains `window`.
    /// Fallback: currently selected monitor.
    pub fn window_to_monitor(&self, window: &Window) -> MonitorId {
        if window == &self.root {
            if let Some(loc) = self.get_root_ptr() {
                let r = Rect { x: loc.x, y: loc.y, width: 1, height: 1 };
                // To guarantee a return value, we deviate a tad from dwm's
                // behaviour and return `selmon` if nothing is found.
                return self.rect_to_monitor(&r);
            }
        }

        if let Some(m) =
            self.monitors.find(|m| m.bar_window().map_or(false, |w| w == window))
        {
            return m.id;
        }

        if let Some(client) = self.window_to_client(window) {
            if let Some(m) = self.monitors.find(|m| m.id == client.mon) {
                return m.id;
            }
        }

        self.selmon().id
    }

    pub fn window_to_client(&self, window: &Window) -> Option<&Client> {
        self.monitors.iter().flat_map(|m| &m.clients).find(|c| c.win.eq(window))
    }

    pub fn c_window_to_client(&self, window: C::Window) -> Option<&Client> {
        self.monitors.iter().flat_map(|m| &m.clients).find(|c| c.win.c() == window)
    }

    /// Searches the list of monitors for the one with the biggest intersection
    /// with `self` (using Monitor.w), and returns that one.
    ///
    /// If nothing is found, return the currently selected monitor.
    pub fn rect_to_monitor(&self, rect: &Rect) -> MonitorId {
        let mut id = self.selmon().id;
        let mut max_area = 0;
        for mon in &self.monitors {
            let area = rect.intersect(&mon.w);
            if max_area < area {
                max_area = area;
                id = mon.id;
            }
        }
        id
    }

    pub fn get_root_ptr(&self) -> Option<Loc> {
        let mut root_return: C::Window = 0;
        let mut child_return: C::Window = 0;
        let mut root_x_return: Coordinate = 0;
        let mut root_y_return: Coordinate = 0;
        let mut win_x_return: Coordinate = 0;
        let mut win_y_return: Coordinate = 0;
        let mut mask_return: c_uint = 0;

        let result = unsafe {
            C::XQueryPointer(
                dpy.c(),
                self.root.c(),
                &mut root_return,
                &mut child_return,
                &mut root_x_return,
                &mut root_y_return,
                &mut win_x_return,
                &mut win_y_return,
                &mut mask_return,
            )
        };
        match result {
            // If XQueryPointer returns False, the pointer is not on the same
            // screen as the specified window, and XQueryPointer returns None to
            // child_return and zero to win_x_return and win_y_return.
            0 => None,
            // If XQueryPointer returns True, the pointer coordinates returned
            // to win_x_return and win_y_return are relative to the origin of
            // the specified window. In this case, XQueryPointer returns the
            // child that contains the pointer, if any, or else None to
            // child_return.
            _ => Some(Loc::new(win_x_return, win_y_return)),
        }
    }

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
        if syms.is_null() {
            return;
        }

        for keycode in start..=end {
            for key in &config::KEYS {
                let offset = (keycode - start) * skip;
                let keysym = unsafe { *syms.add(offset as usize) };
                if key.keysym == keysym {
                    self.numlockmask.grabkey(&self.root, key, keycode);
                }
            }
        }
        unsafe { C::XFree(syms as *mut c_void) };
    }

    pub fn focus(&mut self, client: Option<&Client>) {
        // TODO: implement from dwm's C impl.

        // if (!c || !ISVISIBLE(c))
        // 	for (c = selmon->stack; c && !ISVISIBLE(c); c = c->snext);
        // if (selmon->sel && selmon->sel != c)
        // 	unfocus(selmon->sel, 0);
        // if (c) {
        // 	if (c->mon != selmon)
        // 		selmon = c->mon;
        // 	if (c->isurgent)
        // 		seturgent(c, 0);
        // 	detachstack(c);
        // 	attachstack(c);
        // 	grabbuttons(c, 1);
        // 	XSetWindowBorder(dpy, c->win, scheme[SchemeSel][ColBorder].pixel);
        // 	setfocus(c);
        // } else {
        // 	XSetInputFocus(dpy, root, RevertToPointerRoot, CurrentTime);
        // 	XDeleteProperty(dpy, root, netatom[NetActiveWindow]);
        // }
        // selmon->sel = c;
        // drawbars();
    }

    pub fn applyrules(&mut self, c: &mut Client) {}

    pub fn manage(&mut self, window: Window, attrs: &C::XWindowAttributes) {
        let mut c = Client::new(self.selmon(), window, attrs);
        let w = c.win.c();
        c.update_title();

        let mut trans = C::None as C::Window;
        let result = unsafe { C::XGetTransientForHint(dpy.c(), w, &mut trans) };
        match (result, self.c_window_to_client(trans)) {
            (result, Some(t)) if result != 0 => {
                c.mon = t.mon;
                c.tags = t.tags;
            }
            _ => {
                c.mon = self.selmon().id;
                c.apply_rules(self.monitors.as_slice());
            }
        };

        let c_mon = c.mon(self.monitors.as_slice());
        let c_width = c.width();
        let c_height = c.height();
        let mut r = c.pos.as_mut();

        // If client is too far right, shift it left.
        if (r.x + c_width as Coordinate > c_mon.w.r()) {
            r.x = c_mon.w.r() - c_width as Coordinate;
        }
        // If client is too far down, shift it up.
        if (r.y + c_height as Coordinate > c_mon.w.b()) {
            r.y = c_mon.w.b() - c_height as Coordinate;
        }
        r.x = Coordinate::max(r.x, c_mon.w.x); // If client is too far left, truncate it.
        r.y = Coordinate::max(r.y, c_mon.w.y); // If client is too far up, truncate it.
        c.border_width.set(config::BORDER_PX);

        let mut wc: C::XWindowChanges = unsafe { core::mem::zeroed() };
        wc.border_width = *c.border_width as c_int;

        unsafe { C::XConfigureWindow(dpy.c(), w, C::CWBorderWidth, &mut wc) };
        // unsafe { C::XSetWindowBorder(dpy.c(), w, C::CWBorderWidth, &mut wc) };

        // XSetWindowBorder(dpy, w, scheme[SchemeNorm][ColBorder].pixel);
        // configure(c); /* propagates border_width, if size doesn't change */
        // updatewindowtype(c);
        // updatesizehints(c);
        // updatewmhints(c);
        // XSelectInput(dpy, w, EnterWindowMask|FocusChangeMask|PropertyChangeMask|StructureNotifyMask);
        // grabbuttons(c, 0);
        // if (!c->isfloating)
        // 	c->isfloating = c->oldstate = trans != None || c->isfixed;
        // if (c->isfloating)
        // 	XRaiseWindow(dpy, c->win);
        // attach(c);
        // attachstack(c);
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
