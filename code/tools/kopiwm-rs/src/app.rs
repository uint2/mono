#![allow(unused)]

use crate::C;
use crate::prelude::*;
use config::{Coordinate, Distance};

/// C: type for Coordinates.
/// D: type for Distance.
pub struct App {
    dpy: Display,
    screen: c_int,
    /// Screen size.
    /// Apparently dwm updates this in `void configurenotify(XEvent *)`, and
    /// that's probably how multipe monitors are supported.
    s: Size,
    lrpad: Distance,
    bar_height: Distance,
    /// Owned list of moitors. It is guaranteed that for the lifetime of `Self`,
    /// this list is non-empty.
    mons: NonEmpty<Monitor>,
    root: Window,
    cursors: CursorStateArray<Cursor>,
    colors: WindowColorStateArray<WindowColors<XftColor>>,
    status_text: String,
    numlockmask: NumLockMask,
    fonts: NonEmpty<Font>,
    running: bool,

    net_atoms: NetArray<C::Atom>,
    wm_atoms: WMArray<C::Atom>,
}

pub struct AppInitParams {
}

impl App {
    pub fn new(dpy: Display) -> Self {
        Self {
            dpy,
            screen: todo!(),
            s: todo!(),
            lrpad: todo!(),
            bar_height: todo!(),
            mons: todo!(),
            root: todo!(),
            cursors: todo!(),
            colors: todo!(),
            status_text: todo!(),
            numlockmask: todo!(),
            fonts: todo!(),
            running: todo!(),
            net_atoms: todo!(),
            wm_atoms: todo!(),
        }
    }
}

/// Getters.
impl App {
    pub fn selmon(&self) -> &Monitor {
        self.mons.sel()
    }

    /// Gets an Atom from the pre-computed array.
    pub const fn wm(&self, atom: WM) -> C::Atom {
        *self.wm_atoms.get(atom).unwrap()
    }

    /// Gets an Atom from the pre-computed array.
    pub const fn net(&self, atom: Net) -> C::Atom {
        *self.net_atoms.get(atom).unwrap()
    }
}

/// Core Logic.
impl App {
    pub fn updategeom(&mut self) -> bool {
        let mut dirty = false;
        let m = self.mons.first_mut();
        if m.m.width != self.s.width || m.m.height != self.s.height {
            dirty = true;
            m.m.set_size(self.s);
            m.w.set_size(self.s);
            m.update_bar_pos(self.bar_height);
        }
        if dirty {
            let id = self.window_to_monitor(&self.root);
            let idx = self.mons.position(|v| v.id() == id).unwrap();
            self.mons.set_sel(idx);
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

        if let Some(m) = self.mons.find(|m| m.bar_window().map_or(false, |w| w == window))
        {
            return m.id();
        }

        if let Some(client) = self.window_to_client(window) {
            let mon_id = client.mon();
            if let Some(m) = self.mons.find(|m| m.id() == mon_id) {
                return m.id();
            }
        }

        self.selmon().id()
    }

    pub fn window_to_client(&self, window: &Window) -> Option<&Client> {
        self.mons.iter().flat_map(Monitor::clients).find(|c| c.win() == window)
    }

    /// Searches the list of monitors for the one with the biggest intersection
    /// with `self` (using Monitor.w), and returns that one.
    ///
    /// If nothing is found, return the currently selected monitor.
    pub fn rect_to_monitor(&self, rect: &Rect) -> MonitorId {
        let mut id = self.selmon().id();
        let mut max_area = 0;
        for mon in &self.mons {
            let area = rect.intersect(&mon.w);
            if max_area < area {
                max_area = area;
                id = mon.id();
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
                self.dpy.c(),
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
        let dpy = &self.dpy;
        let root = self.root.c();
        self.numlockmask.update(dpy);

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
}
