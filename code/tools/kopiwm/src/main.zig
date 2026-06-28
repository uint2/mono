const std = @import("std");
const mem = std.mem;
const meta = std.meta;
const log = std.log;
const App = @import("app.zig");
const drw = @import("drw.zig").drw;
const cfg = @import("config.zig");
const Allocator = std.mem.Allocator;
const Monitor = @import("monitor.zig").Monitor;
const Client = @import("client.zig").Client;
const Direction = @import("lazy_fn.zig").Direction;
const Clk = @import("enums.zig").Clk;
const Arg = @import("lazy_fn.zig").Arg;
const Rect = @import("rect.zig").Rect;
const SchemeState = @import("color_scheme.zig").SchemeState;
const MOUSEMASK = @import("config.zig").MOUSEMASK;
const DwmError = @import("errors.zig").DwmError;
const HandlerFn = @import("enums.zig").HandlerFn;
const atoms = @import("atoms.zig");
const X = @import("x11.zig");
const M = @import("x11.zig").masks;
const EM = @import("x11.zig").eventMask;
const CW = @import("x11.zig").CW;
const E = @import("errors.zig");
const Font = @import("font.zig").Font;
const ColorScheme = @import("color_scheme.zig").ColorScheme;

const NAME = @import("build_opts").name;
const VERSION = @import("build_opts").version;

const LINE = "----------------------------------------------------------------------";

/// C standard library.
const C = @cImport({
    @cInclude("locale.h");
    @cInclude("signal.h");
    @cInclude("unistd.h");
});

pub const std_options: std.Options = .{
    .log_level = .debug,
    .logFn = @import("logger.zig").customLog,
};

/// Ensures that there are no other window managers currently running.
///
/// (dwm) checkotherwm
fn checkOtherWM(dpy: *X.Display) void {
    E.xerrorlib = X.XSetErrorHandler(E.xerrorstart);
    // this causes an error if some other window manager is running
    X.XSelectInput(dpy, X.DefaultRootWindow(dpy), EM.SubstructureRedirectMask);
    X.XSync(dpy, false);
    _ = X.XSetErrorHandler(E.xerror);
    X.XSync(dpy, false);
}

/// (dwm) INTERSECT
fn intersect(x: i32, y: i32, w: i32, h: i32, m: *Monitor) i32 {
    return @max(0, @min(x + w, m.wx + @as(i32, @intCast(m.ww))) - @max(x, m.wx)) *
        @max(0, @min(y + h, m.wy + @as(i32, @intCast(m.wh))) - @max(y, m.wy));
}

/// (dwm) wintoclient
/// Searches all the monitors and all of their clients for one that matches
/// the window search query. Returns the first hit.
fn winToClient(z: *App, w: X.Window) ?*Client {
    var m_opt: ?*Monitor = z.mons;
    var c_opt: ?*Client = null;
    while (m_opt) |m| : (m_opt = m.next) {
        c_opt = m.clients;
        while (c_opt) |c| : (c_opt = c.next) {
            if (c.win == w) return c;
        }
    }
    return null;
}

/// (dwm) getstate
fn getState(z: *App, w: X.Window) @typeInfo(X.WindowState).@"enum".tag_type {
    const int_type = @typeInfo(X.WindowState).@"enum".tag_type;
    log.info("::getState", .{});
    const atom = atoms.wm(.State);
    const data = X.XGetWindowProperty(z.dpy, w, atom, 0, 2, false, atom) orelse return -1;
    defer data.deinit();
    if (data.value.len() == 0) return -1;
    const result: int_type = switch (data.value) {
        .Fmt8 => |v| @intCast(v[0]),
        .Fmt16 => |v| @intCast(v[0]),
        .Fmt32 => |v| @intCast(v[0]),
    };
    log.info("::getState returns {d}", .{result});
    return result;
}

/// (dwm) manage
fn manage(z: *App, allocator: Allocator, w: X.Window, wa: *X.XWindowAttributes) error{OutOfMemory}!void {
    const c = try allocator.create(Client);
    c.* = .init(z, w, z.selmon, wa);
    const transient_window = X.XGetTransientForHint(z.dpy, w);

    log.info("Created client {*}", .{c});

    c.updateTitle();
    blk: {
        if (transient_window) |_| {
            // This seems to make very little sense if there is a bijection between
            // clients and windows.
            if (winToClient(z, w)) |other_client| {
                c.mon = other_client.mon;
                c.tags = other_client.tags;
                break :blk;
            }
        }
        c.mon = z.selmon;
        c.applyRules();
    }
    var r = &c.*.pos.now;

    // If client is too far right, shift it left.
    if (r.x + c.width() > c.mon.w.r()) {
        r.x = c.mon.w.r() - c.width();
    }
    // If client is too far down, shift it up.
    if (r.y + c.height() > c.mon.w.b()) {
        r.y = c.mon.w.b() - c.height();
    }
    r.x = @max(r.x, c.mon.w.x); // If client is too far left, truncate it.
    r.y = @max(r.y, c.mon.w.y); // If client is too far up, truncate it.
    c.bw.set(cfg.borderpx);

    var wc = X.XWindowChanges{ .border_width = @intCast(c.bw.now) };
    X.XConfigureWindow(z.dpy, w, CW.BorderWidth, &wc);
    X.XSetWindowBorder(z.dpy, w, z.scheme.get(.Normal).border.pixel);

    c.configure(z.dpy); // propagates border_width, if size doesn't change
    c.updateWindowType();
    c.updateSizeHints();
    c.updateWMHints();

    const mask = EM.EnterWindowMask | EM.FocusChangeMask | EM.PropertyChangeMask | EM.StructureNotifyMask;
    X.XSelectInput(z.dpy, w, mask);

    grabbuttons(z, c, false);

    if (!c.is_floating.now) {
        c.is_floating = .init(transient_window != null or c.is_fixed);
    }
    if (c.is_floating.now) {
        X.XRaiseWindow(z.dpy, c.win);
    }
    c.attach();
    c.attachStack();

    X.XChangeProperty(
        z.dpy,
        z.root,
        atoms.net(.ClientList),
        X.XA_WINDOW,
        32,
        .Append,
        @ptrCast(&c.win),
        1,
    );
    X.XMoveResizeWindow(
        z.dpy,
        c.win,
        c.pos.now.x + 2 * @as(i32, @intCast(z.s.w)),
        c.pos.now.y,
        c.pos.now.w,
        c.pos.now.h,
    ); // dwm: some windows require this.
    // me: I have no idea why. Looks like we're pushing the window off the screen.

    c.setState(.NormalState);
    if (c.mon == z.selmon) {
        unfocus(z, c.mon.sel, false);
    }
    c.mon.sel = c;
    arrange(z, allocator, c.mon);
    X.XMapWindow(z.dpy, c.win);
    focus(z, allocator, null);
}

/// (dwm) unmanage
/// Destroys a client and removes it from the monitor that owns it.
fn unmanage(z: *App, allocator: Allocator, c: *Client, destroyed: bool) void {
    c.detach();
    c.detachStack();

    if (!destroyed) {
        X.XGrabServer(z.dpy); // dwm: Avoid race conditions.
        _ = X.XSetErrorHandler(E.xerrordummy);
        X.XSelectInput(z.dpy, c.win, EM.NoEventMask);
        var wc = X.XWindowChanges{ .border_width = @intCast(c.bw.prev) };
        X.XConfigureWindow(z.dpy, c.win, CW.BorderWidth, &wc); // restore border
        X.XUngrabButton(z.dpy, X.AnyButton, M.AnyModifier, c.win);
        c.setState(.WithdrawnState);
        X.XSync(z.dpy, false);
        _ = X.XSetErrorHandler(E.xerror);
        X.XUngrabServer(z.dpy);
    }
    log.warn("Deallocate client: {*} (will arrange monitor {*})", .{ c, c.mon });
    const m = c.mon; // So that we can still access c.mon after freeing c.
    allocator.destroy(c);
    focus(z, allocator, null);
    updateClientList(z);
    arrange(z, allocator, m);
}

/// (dwm) updateclientlist
/// Updates the ClientList property in the X server.
fn updateClientList(z: *App) void {
    var m_opt: ?*Monitor = z.mons;
    var c_opt: ?*Client = undefined;
    // Delete the existing list.
    X.XDeleteProperty(z.dpy, z.root, atoms.net(.ClientList));
    // Rebuild the list.
    while (m_opt) |m| : (m_opt = m.next) {
        c_opt = m.clients;
        while (c_opt) |c| : (c_opt = c.next) {
            X.XChangeProperty(
                z.dpy,
                z.root,
                atoms.net(.ClientList),
                X.XA_WINDOW,
                32,
                .Append,
                @ptrCast(&c.win),
                1,
            );
        }
    }
}

/// (dwm) arrangemon
fn arrangeMon(m: *Monitor) void {
    if (m.lt.now.arrange) |f| {
        log.info("Arranging monitor {*} with algo \"{s}\"", .{ m, m.lt.now.symbol });
        f(m);
    }
}

/// (dwm) restack
fn restack(z: *App, allocator: Allocator, m: *Monitor) void {
    drawbar(z, allocator, m);

    const has_arrange = m.lt.now.arrange != null;

    const sel = m.sel orelse return;
    if (sel.is_floating.now or !has_arrange) {
        X.XRaiseWindow(z.dpy, sel.win);
    }
    if (has_arrange) {
        var wc = X.XWindowChanges{ .stack_mode = X.Below, .sibling = m.barwin };
        var c_opt = m.stack;
        while (c_opt) |c| : (c_opt = c.snext) {
            if (!c.is_floating.now and c.isVisible()) {
                X.XConfigureWindow(z.dpy, c.win, CW.Sibling | CW.StackMode, &wc);
                wc.sibling = c.win;
            }
        }
    }

    X.XSync(z.dpy, false);
    var ev: X.XEvent = undefined;
    while (X.XCheckMaskEvent(z.dpy, EM.EnterWindowMask, &ev)) {}
}

/// (dwm) arrange
fn arrange(z: *App, allocator: Allocator, monitor: ?*Monitor) void {
    if (monitor) |m| log.info("arranging monitor({*})", .{m}) else log.info("arranging monitor(null)", .{});

    var m_opt: ?*Monitor = null;
    if (monitor) |m| {
        if (m.stack) |c| c.showHide();
    } else {
        m_opt = monitor;
        while (m_opt) |m| : (m_opt = m.next) {
            if (m.stack) |c| c.showHide();
        }
    }
    if (monitor) |m| {
        arrangeMon(m);
        restack(z, allocator, m);
    } else {
        m_opt = monitor;
        while (m_opt) |m| : (m_opt = m.next) {
            arrangeMon(m);
        }
    }
}

/// Functions that are called in [r]eaction to events.
const R = struct {
    /// (dwm) buttonpress
    fn buttonPress(z: *App, allocator: Allocator, e: *X.XEvent) DwmError!void {
        const ev: X.XButtonPressedEvent = e.xbutton;
        var click: Clk = .RootWin;
        var arg: Arg = undefined;

        // Focus monitor if necessary.
        const m = wintomon(z, ev.window);
        if (m != z.selmon) {
            if (z.selmon.sel) |c| unfocus(z, c, true);
            z.selmon = m;
            focus(z, allocator, null);
        }

        // Locate the click, and populate the `click` variable.
        // This block searches for the click location in the bar window.
        if (ev.window == z.selmon.barwin) {
            var i: usize = 0;
            var x: u32 = 0;
            while (true) {
                x += z.TEXTW(allocator, cfg.tags[i].text);
                if (ev.x >= x) {
                    i += 1;
                    if (i < cfg.tags.len) continue;
                }
                break;
            }
            if (i < cfg.tags.len) {
                click = .TagBar;
                arg = .{ .ui = @as(u32, 1) << @intCast(i) };
            } else if (ev.x < x + z.TEXTW(allocator, z.selmon.lt.now.symbol)) {
                click = .LtSymbol;
            } else if (ev.x > z.selmon.w.w - z.TEXTW(allocator, z.stext.get()) + z.lrpad - 2) {
                click = .StatusText;
            } else {
                click = .WinTitle;
            }
        }
        // Locate the click, and populate the `click` variable.
        // This block searches for the click location in the client.
        if (winToClient(z, ev.window)) |c| {
            focus(z, allocator, c);
            restack(z, allocator, z.selmon);
            X.XAllowEvents(z.dpy, .ReplayPointer, X.CurrentTime);
            click = .ClientWin;
        }

        // Search the `buttons` map for a hit.
        for (cfg.buttons) |*button| {
            if (button.click != click or button.button != ev.button) continue;
            if (z.numlockmask.cleanMask(button.mask) == z.numlockmask.cleanMask(ev.state)) {
                const arg2 = switch (click) {
                    .TagBar => &arg,
                    else => &button.lf.arg,
                };
                switch (button.lf.func) {
                    .MightError => |f| try f(z, arg2),
                    .NoError => |f| f(z, arg2),
                    .MightErrorA => |f| try f(z, allocator, arg2),
                    .NoErrorA => |f| f(z, allocator, arg2),
                }
            }
        }
    }

    /// (dwm) clientmessage
    fn clientMessage(z: *App, e: *X.XEvent) void {
        const ev: X.XClientMessageEvent = e.xclient;
        var c: *Client = winToClient(z, ev.window) orelse return;

        if (ev.message_type == atoms.net(.WMState)) {
            const fs_atom = atoms.net(.WMFullscreen);
            if (ev.data.l[1] == fs_atom or ev.data.l[2] == fs_atom) {
                c.setFullscreen(switch (ev.data.l[0]) {
                    1 => true, // _NET_WM_STATE_ADD
                    2 => !c.isfullscreen, // _NET_WM_STATE_TOGGLE
                    else => false,
                });
            }
        } else if (ev.message_type == atoms.net(.ActiveWindow)) {
            if (c != z.selmon.sel and c.isurgent) {
                c.setUrgent(z.dpy, true);
            }
        }
    }

    /// (dwm) configurenotify
    fn configureNotify(z: *App, allocator: Allocator, e: *X.XEvent) error{OutOfMemory}!void {
        const ev: X.XConfigureEvent = e.xconfigure;
        if (ev.window != z.root) return;
        const dirty = z.s.w != ev.width or z.s.h != ev.height;
        z.s.w = @intCast(ev.width);
        z.s.h = @intCast(ev.height);

        // TODO: (dwm) updategeom handling sucks, needs to be simplified
        if (updategeom(z) or dirty) {
            z.drw.resize(z.s.w, z.bar_height);
            updateBars(z);
            var m_opt: ?*Monitor = z.mons;
            var c_opt: ?*Client = undefined;
            while (m_opt) |m| : (m_opt = m.next) {
                c_opt = m.clients;
                while (c_opt) |c| : (c_opt = c.next) {
                    if (c.isfullscreen) {
                        c.resize(m.m);
                    }
                }
                X.XMoveResizeWindow(z.dpy, m.barwin, m.w.x, m.w.y, m.w.w, z.bar_height);
            }
            focus(z, allocator, null);
            arrange(z, allocator, null);
        }
    }
    /// (dwm) configurerequest
    fn configureRequest(z: *App, e: *X.XEvent) void {
        const ev = e.xconfigurerequest;
        const vmask = ev.value_mask;

        if (winToClient(z, ev.window)) |c| {
            if (vmask & CW.BorderWidth != 0) {
                c.bw.set(@intCast(ev.border_width));
            } else if (c.is_floating.now or z.selmon.lt.now.arrange == null) {
                const m = c.mon;
                if (vmask & CW.X != 0) {
                    c.pos.prev.x = c.pos.now.x;
                    c.pos.now.x = m.m.x + ev.x;
                }
                if (vmask & CW.Y != 0) {
                    c.pos.prev.y = c.pos.now.y;
                    c.pos.now.y = m.m.y + ev.y;
                }
                if (vmask & CW.Width != 0) {
                    c.pos.prev.w = c.pos.now.w;
                    c.pos.now.w = @intCast(ev.width);
                }
                if (vmask & CW.Height != 0) {
                    c.pos.prev.h = c.pos.now.h;
                    c.pos.now.h = @intCast(ev.height);
                }
                if (c.pos.now.r() > m.m.r() and c.is_floating.now) {
                    // Center in x-direction.
                    c.pos.prev.x = c.pos.now.x;
                    c.pos.now.x = m.m.x + (@divFloor(@as(i32, @intCast(m.m.w)), 2) - @divFloor(c.width(), 2));
                }
                if (c.pos.now.b() > m.m.b() and c.is_floating.now) {
                    // Center in y-direction.
                    c.pos.prev.y = c.pos.now.y;
                    c.pos.now.y = m.m.y + (@divFloor(@as(i32, @intCast(m.m.h)), 2) - @divFloor(c.height(), 2));
                }
                if ((vmask & (CW.X | CW.Y) != 0) and (vmask & (CW.Width | CW.Height)) == 0) {
                    c.configure(z.dpy);
                }
                if (c.isVisible()) {
                    X.XMoveResizeWindow2(z.dpy, c.win, c.pos.now);
                }
            } else {
                c.configure(z.dpy);
            }
        } else {
            var wc = X.XWindowChanges{
                .x = ev.x,
                .y = ev.y,
                .width = ev.width,
                .height = ev.height,
                .border_width = ev.border_width,
                .sibling = ev.above,
                .stack_mode = ev.detail,
            };
            X.XConfigureWindow(z.dpy, ev.window, @intCast(vmask), &wc);
        }
        X.XSync(z.dpy, false);
    }

    /// (dwm) destroynotify
    fn destroyNotify(z: *App, allocator: Allocator, e: *X.XEvent) void {
        const ev: X.XDestroyWindowEvent = e.xdestroywindow;
        if (winToClient(z, ev.window)) |c| unmanage(z, allocator, c, true);
    }

    /// (dwm) enternotify
    fn enterNotify(z: *App, allocator: Allocator, e: *X.XEvent) void {
        const ev: X.XCrossingEvent = e.xcrossing;
        if ((ev.mode != X.NotifyNormal or ev.detail == X.NotifyInferior) and ev.window != z.root) {
            return;
        }
        const c = winToClient(z, ev.window);
        const m = if (c) |client| client.mon else wintomon(z, ev.window);
        if (m != z.selmon) {
            unfocus(z, z.selmon.sel, true);
            z.selmon = m;
        } else if (c == null or c == z.selmon.sel) {
            return;
        }
        focus(z, allocator, c);
    }

    /// (dwm) expose
    fn expose(z: *App, allocator: Allocator, e: *X.XEvent) void {
        const ev: X.XExposeEvent = e.xexpose;
        if (ev.count == 0) {
            drawbar(z, allocator, wintomon(z, ev.window));
        }
    }

    /// (dwm) focusin
    fn focusIn(z: *App, e: *X.XEvent) void {
        const ev: X.XFocusChangeEvent = e.xfocus;
        if (z.selmon.sel) |sel| {
            if (ev.window != sel.win) sel.setFocus();
        }
    }

    /// (dwm) keypress
    fn keyPress(z: *App, allocator: Allocator, e: *X.XEvent) DwmError!void {
        const ev: X.XKeyEvent = e.xkey;
        const keysym = X.XkbKeycodeToKeysym(z.dpy, @intCast(ev.keycode), 0, 0);
        for (cfg.keys) |key| {
            if (keysym == key.sym and z.numlockmask.cleanMask(key.mod) == z.numlockmask.cleanMask(ev.state)) {
                switch (key.lf.func) {
                    .MightError => |f| try f(z, &key.lf.arg),
                    .NoError => |f| f(z, &key.lf.arg),
                    .MightErrorA => |f| try f(z, allocator, &key.lf.arg),
                    .NoErrorA => |f| f(z, allocator, &key.lf.arg),
                }
            }
        }
    }

    /// (dwm) mappingnotify
    fn mappingNotify(z: *App, e: *X.XEvent) void {
        const ev: *X.XMappingEvent = &e.xmapping;
        X.XRefreshKeyboardMapping(ev);
        if (ev.request == X.MappingKeyboard) {
            grabkeys(z);
        }
    }

    /// (dwm) maprequest
    fn mapRequest(z: *App, allocator: Allocator, e: *X.XEvent) error{OutOfMemory}!void {
        const ev: X.XMapRequestEvent = e.xmaprequest;
        var wa: X.XWindowAttributes = undefined;

        if (!X.XGetWindowAttributes(z.dpy, ev.window, &wa)) return;
        if (wa.override_redirect != 0) return;

        if (winToClient(z, ev.window) == null) {
            log.info("Start managing window {d} (mapRequest)", .{ev.window});
            try manage(z, allocator, ev.window, &wa);
        }
    }

    /// (dwm) motionnotify
    fn motionNotify(z: *App, allocator: Allocator, e: *X.XEvent) void {
        const ev: X.XMotionEvent = e.xmotion;
        const static = struct {
            var mon: ?*Monitor = null;
        };
        if (ev.window != z.root) return;
        const rect = Rect{ .x = ev.x_root, .y = ev.y_root, .w = 1, .h = 1 };
        const m_opt = rect.toMonitor(z.selmon);
        if (m_opt) |m| {
            if (static.mon) |mon| {
                if (m != mon) {
                    unfocus(z, z.selmon.sel, true);
                    z.selmon = m;
                    focus(z, allocator, null);
                }
            }
        }
        static.mon = m_opt;
    }

    /// (dwm) propertynotify
    fn propertyNotify(z: *App, allocator: Allocator, e: *X.XEvent) void {
        const ev: X.XPropertyEvent = e.xproperty;
        if (ev.window == z.root and ev.atom == X.XA_WM_NAME) {
            updateStatus(z, allocator);
        } else if (ev.state == X.PropertyDelete) {
            return; // ignore.
        } else if (winToClient(z, ev.window)) |c| {
            switch (ev.atom) {
                X.XA_WM_TRANSIENT_FOR => {
                    const trans_opt = X.XGetTransientForHint(z.dpy, c.win);
                    const b = !c.is_floating.now and trans_opt != null;
                    if (trans_opt) |t| c.is_floating.set(winToClient(z, t) != null);
                    if (b and c.is_floating.now) arrange(z, allocator, c.mon);
                },
                X.XA_WM_NORMAL_HINTS => c.hintsvalid = false,
                X.XA_WM_HINTS => {
                    c.updateWMHints();
                    drawbars(z, allocator);
                },
                else => {},
            }
            if (ev.atom == X.XA_WM_NAME or ev.atom == atoms.net(.WMName)) {
                c.updateTitle();
                if (c == c.mon.sel) {
                    drawbar(z, allocator, c.mon);
                }
            }
            if (ev.atom == atoms.net(.WMWindowType)) {
                c.updateWindowType();
            }
        }
    }

    /// (dwm) unmapnotify
    fn unmapNotify(z: *App, allocator: Allocator, e: *X.XEvent) void {
        const ev: X.XUnmapEvent = e.xunmap;
        if (winToClient(z, ev.window)) |c| {
            if (ev.send_event == 0) {
                unmanage(z, allocator, c, false);
            } else {
                c.setState(.WithdrawnState);
            }
        }
    }
};

/// For debugging: implement an emergency timeout in case we can't back out.
const TIMEOUT: bool = false;

/// (dwm) run
/// main event loop
fn run(z: *App, allocator: Allocator) DwmError!void {
    X.XSync(z.dpy, false);
    var ev: X.XEvent = undefined;
    const start = std.time.timestamp();

    while (z.running and X.XNextEvent(z.dpy, &ev)) {
        if (TIMEOUT and @abs(std.time.timestamp() - start) > 20) @panic("End please");
        try runOne(z, allocator, &ev);
    }
}

inline fn runOne(z: *App, alloc: Allocator, ev: *X.XEvent) DwmError!void {
    switch (ev.type) {
        X.ButtonPress => try R.buttonPress(z, alloc, ev),
        X.ClientMessage => R.clientMessage(z, ev),
        X.ConfigureNotify => try R.configureNotify(z, alloc, ev),
        X.ConfigureRequest => R.configureRequest(z, ev),
        X.DestroyNotify => R.destroyNotify(z, alloc, ev),
        X.EnterNotify => R.enterNotify(z, alloc, ev),
        X.Expose => R.expose(z, alloc, ev),
        X.FocusIn => R.focusIn(z, ev),
        X.KeyPress => try R.keyPress(z, alloc, ev),
        X.MapRequest => try R.mapRequest(z, alloc, ev),
        X.MappingNotify => R.mappingNotify(z, ev),
        X.MotionNotify => R.motionNotify(z, alloc, ev),
        X.PropertyNotify => R.propertyNotify(z, alloc, ev),
        X.UnmapNotify => R.unmapNotify(z, alloc, ev),
        else => {},
    }
}

/// (dwm) scan
fn scan(z: *App, allocator: Allocator) error{OutOfMemory}!void {
    var wa: X.XWindowAttributes = undefined;
    var i: c_uint = undefined;
    var d1: X.Window = undefined;
    var d2: X.Window = undefined;

    // No need to call XFree because null in Zig means NULL in C.
    const wins: []X.Window = X.XQueryTree(z.dpy, z.root, &d1, &d2) orelse return;
    defer X.XFree(wins.ptr);

    // Note: this section down here in important in deciding which window to be
    // `manage`d. We specifically do NOT want to be `manage`-ing the bar
    // window.

    i = 0;
    while (i < wins.len) : (i += 1) {
        const ok = X.XGetWindowAttributes(z.dpy, wins[i], &wa);
        if (!ok or wa.override_redirect != 0) continue;
        if (X.XGetTransientForHint(z.dpy, wins[i]) == null) continue;
        if (wa.map_state == X.IsViewable or getState(z, wins[i]) == X.IconicState) {
            log.info("Start managing window {d} (scan, non-transient)", .{wins[i]});
            try manage(z, allocator, wins[i], &wa);
        }
    }
    i = 0;
    while (i < wins.len) : (i += 1) { // now the transients
        if (!X.XGetWindowAttributes(z.dpy, wins[i], &wa)) continue;
        if (X.XGetTransientForHint(z.dpy, wins[i]) == null) continue;
        const viewable = wa.map_state == X.IsViewable;
        const iconic = getState(z, wins[i]) == X.IconicState;
        if (viewable or iconic) {
            log.info("Start managing window {d} (scan, transient)", .{wins[i]});
            try manage(z, allocator, wins[i], &wa);
        }
    }
}

/// (dwm) sendmon
///
/// Sends a client to a monitor.
fn sendMon(z: *App, allocator: Allocator, c: *Client, m: *Monitor) void {
    if (c.mon == m) return;
    // Leave the previous monitor.
    unfocus(z, c, true);
    c.detach();
    c.detachStack();
    // Enter the new monitor.
    c.mon = m;
    c.tags = m.tags; // Assign tags of target monitor.
    c.attach();
    c.attachStack();
    if (c.isfullscreen) c.resize(m.m);
    focus(z, allocator, null);
    arrange(z, allocator, null);
}

/// (dwm) wintomon
fn wintomon(z: *App, w: X.Window) *Monitor {
    if (w == z.root) {
        if (z.getRootPtr()) |coords| {
            const r = Rect{ .x = coords.x, .y = coords.y, .w = 1, .h = 1 };
            // To guarantee a non-null return of `*Monitor`, we deviate a tad from
            // dwm's behaviour and return `selmon` if nothing is found.
            return r.toMonitor(z.mons) orelse z.selmon;
        }
    }
    var m_opt: ?*Monitor = z.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        if (w == m.barwin) return m;
    }
    if (winToClient(z, w)) |c| return c.mon;
    return z.selmon;
}

/// (dwm) updategeom
fn updategeom(z: *App) bool {
    var dirty = false;
    var mons = z.mons;
    if (mons.m.w != z.s.w or mons.m.h != z.s.h) {
        dirty = true;
        mons.w.w = z.s.w;
        mons.w.h = z.s.h;
        mons.m.w = z.s.w;
        mons.m.h = z.s.h;
        mons.updateBarPosition(z.bar_height);
    }
    if (dirty) {
        z.selmon = mons;
        z.selmon = wintomon(z, z.root);
    }
    return dirty;
}

/// Do not let children turn into zombies when they terminate.
fn setupTerminationHandling() void {
    var sa: C.struct_sigaction = undefined;

    // Initialize and empty a signal set.
    _ = C.sigemptyset(&sa.sa_mask);

    // source: https://man7.org/linux/man-pages/man2/sigaction.2.html
    sa.sa_flags =
        // Do not receive notification when child processes stop or resume.
        C.SA_NOCLDSTOP |
        // Do not transform children into zombies when they terminate.
        //
        // If the SA_NOCLDWAIT flag is set when establishing a handler for
        // SIGCHLD, POSIX.1 leaves it unspecified whether a SIGCHLD signal is
        // generated when a child process terminates.  On Linux, a SIGCHLD
        // signal is generated in this case; on some other implementations, it
        // is not.
        C.SA_NOCLDWAIT |
        // Provide behavior compatible with BSD signal semantics by making
        // certain system calls restartable across signals. This flag is
        // meaningful only when establishing a signal handler.
        C.SA_RESTART;

    // sa_handler specifies the action to be associated with _signum_ and can be
    // one of the following:
    //   *  SIG_DFL for the default action.
    //   *  SIG_IGN to ignore this signal.
    //   *  A pointer to a signal handling function.  This function receives
    //      the signal number as its only argument.
    sa.__sigaction_handler.sa_handler = C.SIG_IGN;

    // The sigaction() system call is used to change the action taken by a
    // process on receipt of a specific signal.
    _ = C.sigaction(C.SIGCHLD, &sa, null);
}

/// Clean up any zombies (inherited from .xinitrc etc).
///
/// pid=-1 means to wait for any child process.
///
/// On success, `waitpid` returns the pid of the child whose state has
/// changed; if WNOHANG was specified and one or more child(ren) specified
/// by pid exist, but have not yet changed state, then 0 is returned. On
/// failure, -1 is returned.
///
/// docs: https://man7.org/linux/man-pages/man2/waitpid.2.html
fn setupClearZombies() void {
    while (std.c.waitpid(-1, null, std.c.W.NOHANG) > 0) {}
}

/// (dwm) unfocus
fn unfocus(z: *App, client: ?*Client, setfocus: bool) void {
    const c = client orelse return;
    log.info("Unfocusing client at: {*}", .{c});
    grabbuttons(z, c, false);
    X.XSetWindowBorder(z.dpy, c.win, z.scheme.get(.Normal).border.pixel);
    if (setfocus) {
        X.XSetInputFocus(z.dpy, z.root, .PointerRoot, X.CurrentTime);
        X.XDeleteProperty(z.dpy, z.root, atoms.net(.ActiveWindow));
    }
}

/// (dwm) focus
///
/// Draws the focus to one particular client. If no `client` is provided (null),
/// then focus any client that is visible.
fn focus(z: *App, allocator: Allocator, client: ?*Client) void {
    log.info("Focusing a client...", .{});

    const target = z.resolveFocus(client) orelse {
        log.info("No focus target found.", .{});
        X.XSetInputFocus(z.dpy, z.root, .PointerRoot, X.CurrentTime);
        X.XDeleteProperty(z.dpy, z.root, atoms.net(.ActiveWindow));
        z.selmon.sel = null;
        drawbars(z, allocator);
        return;
    };

    log.info("Focusing client @ {*}", .{target});

    // If the currently selected client is not the new target, then unfocus it.
    if (z.selmon.sel) |sel| {
        if (sel != target) unfocus(z, sel, false);
    }
    z.selmon = target.mon;
    // if the client (that's about to be focused) is urgent, then put it at
    // ease for it is about to be tended to.
    if (target.isurgent) target.setUrgent(z.dpy, false);
    target.detachStack();
    target.attachStack();
    grabbuttons(z, target, true);
    X.XSetWindowBorder(z.dpy, target.win, z.scheme.get(.Selected).border.pixel);
    target.setFocus();
    z.selmon.sel = target;
    drawbars(z, allocator);
}

/// (dwm) drawbars
fn drawbars(z: *App, allocator: Allocator) void {
    var m_opt: ?*Monitor = z.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        drawbar(z, allocator, m);
    }
}

/// (dwm) grabbuttons
fn grabbuttons(z: *App, c: *Client, focused: bool) void {
    z.numlockmask.update(z.dpy);
    X.XUngrabButton(z.dpy, X.AnyButton, M.AnyModifier, c.win);
    const bmask = EM.ButtonPressMask | EM.ButtonReleaseMask;
    if (!focused) {
        X.XGrabButton(z.dpy, X.AnyButton, M.AnyModifier, c.win, false, bmask, .Sync, .Sync, X.None, X.None);
    }
    for (cfg.buttons) |button| {
        if (button.click == .ClientWin) {
            for (z.numlockmask.modifiers) |modifier| {
                X.XGrabButton(z.dpy, button.button, button.mask | modifier, c.win, false, bmask, .Async, .Sync, X.None, X.None);
            }
        }
    }
}

/// (dwm) grabkeys
fn grabkeys(z: *App) void {
    z.numlockmask.update(z.dpy);

    var start: c_int = undefined; // or, X.KeyCode
    var end: c_int = undefined; // or, X.KeyCode
    var skip: c_int = undefined;

    X.XUngrabKey(z.dpy, X.AnyKey, M.AnyModifier, z.root);
    X.XDisplayKeycodes(z.dpy, &start, &end);
    const syms = X.XGetKeyboardMapping(z.dpy, @intCast(start), end - start + 1, &skip) orelse return;
    defer X.XFree(syms);

    var keycode = start;
    while (keycode < end) : (keycode += 1) {
        for (cfg.keys) |key| {
            // Skip modifier codes, we do that ourselves.
            if (key.sym == syms[@intCast((keycode - start) * skip)]) {
                for (z.numlockmask.modifiers) |mod| {
                    _ = X.XGrabKey(z.dpy, keycode, key.mod | mod, z.root, true, .Async, .Async);
                }
            }
        }
    }
}

/// (dwm) cleanup
/// Cleanup monitors and their clients.
fn cleanupMonitors(z: *App, allocator: Allocator) void {
    var m_opt: ?*Monitor = undefined;

    // Hide all the bars so that we don't use fonts for cleanup.
    m_opt = z.mons;
    while (m_opt) |m| : (m_opt = m.next) m.show_bar = false;

    log.info("Start cleaning up monitors!", .{});
    // View all clients at once. ~0 yields a bitmask of all high bits. I don't
    // fully understand why we do this yet, but I think it helps with clearing
    // out the clients.
    mp.view(z, allocator, &.{ .ui = ~@as(u32, 0) });
    z.selmon.lt.set(&.{ .symbol = "", .arrange = null });

    m_opt = z.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        while (m.stack) |c| {
            unmanage(z, allocator, c, false);
        }
    }
    while (z.mons.next) |m| {
        // Remove `m` from the linked list that is `z.mons`.
        z.mons.next = m.next;
        m.deinit(allocator, z.dpy);
    }
    z.mons.deinit(allocator, z.dpy);
    X.XSync(z.dpy, false);
    X.XSetInputFocus(z.dpy, X.PointerRoot, .PointerRoot, X.CurrentTime);
    X.XDeleteProperty(z.dpy, z.root, atoms.net(.ActiveWindow));
}

/// (dwm) updatebars
fn updateBars(z: *App) void {
    var wa: X.XSetWindowAttributes = .{
        .override_redirect = X.True,
        .background_pixmap = X.ParentRelative,
        .event_mask = EM.ButtonPressMask | EM.ExposureMask,
    };
    const static = struct {
        var name: [NAME.len]u8 = init();
        fn init() [NAME.len]u8 {
            var buf: [NAME.len]u8 = undefined;
            @memcpy(&buf, NAME);
            return buf;
        }
    };
    var ch: X.XClassHint = .{ .res_class = &static.name, .res_name = &static.name };
    var m_opt: ?*Monitor = z.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        if (m.barwin != 0) {
            continue;
        }
        m.barwin = X.XCreateWindow(
            z.dpy,
            z.root,
            m.w.x,
            m.by,
            m.w.w,
            z.bar_height,
            0,
            X.DefaultDepth(z.dpy, z.screen),
            X.CopyFromParent,
            X.DefaultVisual(z.dpy, z.screen),
            CW.OverrideRedirect | CW.BackPixmap | CW.EventMask,
            &wa,
        );
        log.info("Create bar window({d}): (x={d}, y={d}, w={d}, h={d})", .{
            m.barwin,
            m.w.x,
            m.by,
            m.w.w,
            z.bar_height,
        });
        X.XDefineCursor(z.dpy, m.barwin, z.cursors.get(.Normal));
        X.XMapRaised(z.dpy, m.barwin);
        X.XSetClassHint(z.dpy, m.barwin, &ch);
    }
}

/// (dwm) updatestatus
fn updateStatus(z: *App, allocator: Allocator) void {
    if (z.getTextProp(z.root, X.XA_WM_NAME, &z.stext.buffer)) |len| {
        z.stext.len = len;
    } else {
        z.stext.set(NAME ++ "-" ++ VERSION);
    }
    drawbar(z, allocator, z.selmon);
}

/// Layouts.
pub const layouts = struct {
    /// (dwm) monocle
    pub fn monocle(m: *Monitor) void {
        var c_opt = m.clients;
        var n: u32 = 0;
        while (c_opt) |c| : (c_opt = c.next) {
            if (c.isVisible()) n += 1;
        }
        if (n > 0) { // Override layout symbol.
            // TODO: have to make the layout symbol an owned buffer. Right not it
            // can only display const strings.
            // snprintf(m->ltsymbol, sizeof m->ltsymbol, "[%d]", n);
        }
        c_opt = (c_opt orelse return).nextTiled();
        while (c_opt) |c| : (c_opt = c.nextTiled()) {
            var r = m.w;
            r.w = m.w.w - 2 * @as(u32, @intCast(c.bw.now));
            r.h = m.w.h - 2 * @as(u32, @intCast(c.bw.now));
            c.hintAndResize(r, false);
        }
    }

    /// (dwm) tile
    pub fn tile(m: *Monitor) void {
        const n = m.countTiledClients();
        if (n == 0) return;
        const mw: u32 = blk: {
            if (n > m.nmaster) {
                if (m.nmaster == 0) break :blk 0;
                break :blk @intFromFloat(@as(f32, @floatFromInt(m.w.w)) * m.mfact);
            }
            break :blk m.w.w;
        };

        log.info("tile with {d} clients, with mw={d}", .{ n, mw });

        var i: u32 = 0;
        var my: i32 = 0; // master's y
        var ty: i32 = 0; // non-master's y
        var c_opt = if (m.clients) |c| c.nextTiled() else null;
        while (c_opt) |c| : ({
            c_opt = c.nextTiledExclusive();
            i += 1;
        }) {
            log.debug("n={d}, i={d}, ty={d}, my={d}", .{ n, i, ty, my });
            if (i < m.nmaster) {
                const h = @divFloor(m.w.h - @as(u32, @intCast(my)), @min(n, m.nmaster) - i);
                c.hintAndResize(.{
                    .x = m.w.x,
                    .y = m.w.y + my,
                    .w = @intCast(mw - (2 * c.bw.now)),
                    .h = @intCast(h - (2 * c.bw.now)),
                }, false);
                if (my + c.height() < m.w.h) {
                    my += c.height();
                }
            } else {
                const h = @divFloor(m.w.h - @as(u32, @intCast(ty)), n - i);
                c.hintAndResize(.{
                    .x = m.w.x + @as(i32, @intCast(mw)),
                    .y = m.w.y + ty,
                    .w = m.w.w - mw - 2 * c.bw.now,
                    .h = h - 2 * c.bw.now,
                }, false);
                if (ty + c.height() < m.w.h) {
                    ty += c.height();
                }
            }
        }
    }
};

/// Mappable functions. Everything has to have a `arg: *const Arg` parameter.
pub const mp = struct {
    /// (dwm) togglefloating
    pub fn toggleFloating(z: *App, allocator: Allocator, _: *const Arg) void {
        const sel = z.selmon.sel orelse return;
        if (sel.isfullscreen) return; // No support for making fullscreen windows float.
        sel.is_floating.set(!sel.is_floating.now or sel.is_fixed);
        if (sel.is_floating.now) {
            sel.hintAndResize(sel.pos.now, false);
        }
        arrange(z, allocator, z.selmon);
    }

    /// (dwm) spawn
    pub fn spawn(z: *App, arg: *const Arg) void {
        const args = switch (arg.*) {
            .args => |value| value,
            else => unreachable,
        };
        const pid = std.posix.fork() catch {
            @panic("Unable to fork while trying to spawn a child process.");
        };
        if (pid == 0) {
            _ = C.close(X.ConnectionNumber(z.dpy));
            _ = C.setsid();

            var sa: C.struct_sigaction = undefined;
            _ = C.sigemptyset(&sa.sa_mask);
            sa.sa_flags = 0;
            sa.__sigaction_handler.sa_handler = C.SIG_DFL;
            _ = C.sigaction(C.SIGCHLD, &sa, null);
            const err = std.posix.execvpeZ(args[0].?, args, std.c.environ);
            std.debug.print("execvp failed with:\n{any}\n", .{err});
            std.process.exit(1);
        }
    }

    /// (dwm) tag
    pub fn tag(z: *App, allocator: Allocator, arg: *const Arg) void {
        const mask = switch (arg.*) {
            .ui => |v| if (v & cfg.TAGMASK != 0) v else return,
            else => unreachable,
        };
        if (z.selmon.sel) |c| {
            c.tags = mask & cfg.TAGMASK;
            focus(z, allocator, null);
            arrange(z, allocator, z.selmon);
        }
    }

    /// (dwm) tagmon
    pub fn tagMonitor(z: *App, allocator: Allocator, arg: *const Arg) void {
        const direction = switch (arg.*) {
            .d => |v| v,
            else => unreachable,
        };

        const sel = z.selmon.sel orelse return;

        const target = z.getMonitorFromDirection(direction);
        if (target == z.selmon) return;

        sendMon(z, allocator, sel, target);
    }

    /// (dwm) view
    /// Views a certain tag mask.
    pub fn view(z: *App, allocator: Allocator, arg: *const Arg) void {
        const mask = switch (arg.*) {
            .ui => |mask| b: {
                // This mask is expected to only have one high bit.
                if (mask != ~@as(@TypeOf(mask), 0) and @popCount(mask) != 1) {
                    log.err("view() received a mask of {x}", .{mask});
                }
                break :b mask & cfg.TAGMASK;
            },
            else => unreachable,
        };
        log.info("view with bitmask: {b}", .{mask});
        if (mask == z.selmon.tags) {
            return; // nothing to do here.
        } else if (mask != 0) {
            z.selmon.tags = mask;
        }
        focus(z, allocator, null);
        arrange(z, allocator, z.selmon);
    }

    /// (dwm) togglebar
    pub fn toggleBar(z: *App, allocator: Allocator, _: *const Arg) void {
        z.selmon.show_bar = !z.selmon.show_bar;
        z.selmon.updateBarPosition(z.bar_height);
        X.XMoveResizeWindow2(z.dpy, z.selmon.barwin, z.barRect());
        arrange(z, allocator, z.selmon);
    }

    pub fn toggleBarPosition(z: *App, allocator: Allocator, _: *const Arg) void {
        z.selmon.bar_pos = z.selmon.bar_pos.toggle();
        z.selmon.updateBarPosition(z.bar_height);
        X.XMoveResizeWindow2(z.dpy, z.selmon.barwin, z.barRect());
        arrange(z, allocator, z.selmon);
    }

    /// (dwm) toggletag
    pub fn toggleTag(z: *App, allocator: Allocator, arg: *const Arg) void {
        const mask = switch (arg.*) {
            .ui => |v| v,
            else => unreachable,
        };
        const sel = z.selmon.sel orelse return;
        const newtags = sel.tags ^ (mask & cfg.TAGMASK);
        if (newtags != 0) {
            sel.tags = newtags;
            focus(z, allocator, null);
            arrange(z, allocator, z.selmon);
        }
    }

    /// (dwm) toggleview
    pub fn toggleView(z: *App, allocator: Allocator, arg: *const Arg) void {
        const mask = switch (arg.*) {
            .ui => |v| v & cfg.TAGMASK,
            else => unreachable,
        };
        const newtagset = z.selmon.tags ^ mask;
        if (newtagset != 0) {
            z.selmon.tags = newtagset;
            focus(z, allocator, null);
            arrange(z, allocator, z.selmon);
        }
    }

    /// (dwm) quit
    pub fn quit(z: *App, _: *const Arg) void {
        log.info("{s}", .{LINE});
        log.info("quit() called.", .{});
        z.running = false;
    }

    /// (dwm) zoom
    pub fn zoom(z: *App, allocator: Allocator, _: *const Arg) void {
        var c: ?*Client = z.selmon.sel orelse return;
        if (c.?.is_floating.now or z.selmon.lt.now.arrange == null) return;
        const nextTiled: ?*Client = if (z.selmon.clients) |x| x.nextTiled() else null;
        if (c == nextTiled) {
            c = if (c.?.next) |x| x.nextTiled() else null;
            if (c == null) {
                return;
            }
        }
        pop(z, allocator, c.?);
    }

    /// (dwm) focusmon
    pub fn focusMon(z: *App, allocator: Allocator, arg: *const Arg) void {
        const direction = switch (arg.*) {
            .d => |v| v,
            else => unreachable,
        };

        const target = z.getMonitorFromDirection(direction);

        if (target == z.selmon) return;
        unfocus(z, z.selmon.sel, false);
        z.selmon = target;
        focus(z, allocator, null);
    }

    /// (dwm) focusstack
    ///
    /// Focuses the next/previous client in the stack.
    pub fn focusStack(z: *App, allocator: Allocator, arg: *const Arg) void {
        const direction = switch (arg.*) {
            .d => |v| v,
            else => unreachable,
        };
        const sel = z.selmon.sel orelse return;
        if (sel.isfullscreen and cfg.lockfullscreen) return;
        var c_opt: ?*Client = null;
        switch (direction) {
            .Next => {
                c_opt = sel.next;
                // TODO: figure out why this isn't c.snext.
                while (c_opt) |c| : (c_opt = c.next) {
                    if (c.isVisible()) {
                        break;
                    }
                }
                if (c_opt == null) {
                    c_opt = z.selmon.clients;
                    while (c_opt) |c| : (c_opt = c.next) {
                        if (c.isVisible()) {
                            break;
                        }
                    }
                }
            },
            .Prev => {
                var i_opt: ?*Client = null;
                i_opt = z.selmon.clients;
                while (i_opt) |i| : (i_opt = i.next) {
                    if (i == sel) break;
                    if (i.isVisible()) c_opt = i;
                }
                if (c_opt == null) {
                    while (i_opt) |i| : (i_opt = i.next) {
                        if (i.isVisible()) c_opt = i;
                    }
                }
            },
        }
        if (c_opt) |c| {
            focus(z, allocator, c);
            restack(z, allocator, z.selmon);
        }
    }

    /// (dwm) incnmaster
    pub fn incNMaster(z: *App, allocator: Allocator, arg: *const Arg) void {
        const i = switch (arg.*) {
            .i => |v| v,
            else => unreachable,
        };
        z.selmon.nmaster = @intCast(@max(@as(i32, @intCast(z.selmon.nmaster)) + i, 0));
        arrange(z, allocator, z.selmon);
    }

    /// (dwm) killclient
    pub fn killClient(z: *App, _: *const Arg) void {
        const sel = z.selmon.sel orelse return;
        log.info("Trying to kill client {*}", .{sel});
        if (!sel.sendEvent(atoms.wm(.Delete))) {
            log.info("Kill effective", .{});
            X.XGrabServer(z.dpy);
            _ = X.XSetErrorHandler(E.xerrordummy);
            X.XSetCloseDownMode(z.dpy, .DestroyAll);
            X.XKillClient(z.dpy, sel.win);
            X.XSync(z.dpy, false);
            _ = X.XSetErrorHandler(E.xerror);
            X.XUngrabServer(z.dpy);
        } else {
            log.info("Kill ineffective", .{});
        }
    }

    /// (dwm) movemouse
    pub fn moveMouse(z: *App, allocator: Allocator, _: *const Arg) DwmError!void {
        var c = z.selmon.sel orelse return;
        if (c.isfullscreen) return; // No support moving fullscreen windows by mouse.
        restack(z, allocator, z.selmon);

        // Old client x and y coordinates.
        const ocx = c.pos.now.x;
        const ocy = c.pos.now.y;

        const grab_ok = X.XGrabPointer(z.dpy, z.root, false, MOUSEMASK, .Async, //
            .Async, X.None, z.cursors.get(.Move), X.CurrentTime);
        if (!grab_ok) return;
        const coords = z.getRootPtr() orelse return;
        const x = coords.x;
        const y = coords.y;
        var ev: X.XEvent = undefined;
        var lasttime: X.Time = 0;
        while (true) {
            X.XMaskEvent(z.dpy, MOUSEMASK | EM.ExposureMask | EM.SubstructureRedirectMask, &ev);
            switch (ev.type) {
                X.Expose | X.MapRequest | X.ConfigureRequest => try runOne(z, allocator, &ev),
                X.MotionNotify => {
                    if (ev.xmotion.time - lasttime <= @divFloor(1000, cfg.refreshrate)) {
                        continue;
                    }
                    lasttime = ev.xmotion.time;
                    var nx = ocx + (ev.xmotion.x - x);
                    var ny = ocy + (ev.xmotion.y - y);
                    if (@abs(z.selmon.w.x - nx) < cfg.snap) {
                        nx = z.selmon.w.x;
                    } else if (@abs(z.selmon.w.r() - (nx + c.width())) < cfg.snap) {
                        nx = z.selmon.w.r() - c.width();
                    }
                    if (@abs(z.selmon.w.y - ny) < cfg.snap) {
                        ny = z.selmon.w.y;
                    } else if (@abs(z.selmon.w.b() - (ny + c.height())) < cfg.snap) {
                        ny = z.selmon.w.b() - c.height();
                    }
                    if (!c.is_floating.now and
                        z.selmon.lt.now.arrange != null and
                        (@abs(nx - c.pos.now.x) > cfg.snap or
                            @abs(ny - c.pos.now.y) > cfg.snap))
                    {
                        toggleFloating(z, allocator, undefined);
                    }
                    if (z.selmon.lt.now.arrange != null or c.is_floating.now) {
                        var r = c.pos.now;
                        r.x = nx;
                        r.y = ny;
                        c.hintAndResize(r, true);
                    }
                },
                X.ButtonRelease => break,
                else => {},
            }
        }
        X.XUngrabPointer(z.dpy, X.CurrentTime);
        const m_opt = c.pos.now.toMonitor(z.mons);
        if (m_opt != z.selmon) {
            if (m_opt) |m| {
                sendMon(z, allocator, c, m);
                z.selmon = m;
                focus(z, allocator, null);
            }
        }
    }

    /// (dwm) setlayout
    pub fn setLayout(z: *App, allocator: Allocator, arg: *const Arg) void {
        // TODO: check all other instances of tagged access of args. Make sure to
        // use a switch statement before indexing.
        const lt = switch (arg.*) {
            .l => |lt| lt,
            else => unreachable,
        };
        z.selmon.lt.now = lt;
        if (z.selmon.sel) |_| {
            arrange(z, allocator, z.selmon);
        } else {
            drawbar(z, allocator, z.selmon);
        }
    }

    /// (dwm) setmfact
    pub fn setMFact(z: *App, allocator: Allocator, arg: *const Arg) void {
        if (z.selmon.lt.now.arrange == null) return;
        const f: f32 = switch (arg.*) {
            .f => |v| v,
            else => unreachable,
        };
        if (0.05 <= f and f <= 0.95) {
            z.selmon.mfact = f;
            arrange(z, allocator, z.selmon);
        }
    }

    /// (dwm) resizemouse
    pub fn resizeMouse(z: *App, allocator: Allocator, _: *const Arg) DwmError!void {
        var c = z.selmon.sel orelse return;
        if (c.isfullscreen) return; // No support moving fullscreen windows by mouse.
        restack(z, allocator, z.selmon);

        // Old client x and y coordinates.
        const ocx = c.pos.now.x;
        const ocy = c.pos.now.y;

        const grab_ok = X.XGrabPointer(
            z.dpy,
            z.root,
            false,
            MOUSEMASK,
            .Async,
            .Async,
            X.None,
            z.cursors.get(.Resize),
            X.CurrentTime,
        );
        if (!grab_ok) return;
        if (c.is_floating.now) {
            X.XWarpPointer(z.dpy, X.None, c.win, .zero, //
                @intCast(c.pos.now.w + c.bw.now - 1), //
                @intCast(c.pos.now.h + c.bw.now - 1));
        } else {
            X.XWarpPointer(z.dpy, X.None, z.selmon.barwin, .zero, //
                @intFromFloat(z.selmon.mfact * @as(f32, @floatFromInt(z.selmon.m.w))), //
                @intCast(@divFloor(z.selmon.m.h, 2)));
        }
        var ev: X.XEvent = undefined;
        var lasttime: X.Time = 0;
        while (true) {
            X.XMaskEvent(z.dpy, MOUSEMASK | EM.ExposureMask | EM.SubstructureRedirectMask, &ev);
            switch (ev.type) {
                X.Expose | X.MapRequest | X.ConfigureRequest => try runOne(z, allocator, &ev),
                X.MotionNotify => {
                    if (ev.xmotion.time - lasttime <= @divFloor(1000, cfg.refreshrate)) {
                        continue;
                    }
                    lasttime = ev.xmotion.time;
                    const nw: i32 = @max(@as(i32, @intCast(ev.xmotion.x)) - ocx - 2 * @as(i32, @intCast(c.bw.now)) + 1, 1);
                    const nh: i32 = @max(@as(i32, @intCast(ev.xmotion.y)) - ocy - 2 * @as(i32, @intCast(c.bw.now)) + 1, 1);
                    if (!c.is_floating.now) {
                        const f = @as(f32, @floatFromInt(ev.xmotion.x)) /
                            @as(f32, @floatFromInt(z.selmon.m.w));
                        if (0.05 <= f and f <= 0.95) {
                            z.selmon.mfact = f;
                            arrange(z, allocator, z.selmon);
                        }
                        // toggleFloating(undefined);
                    } else if (c.mon.w.x + nw >= z.selmon.w.l() and
                        c.mon.w.x + nw <= z.selmon.w.r() and
                        c.mon.w.y + nh >= z.selmon.w.t() and
                        c.mon.w.y + nh <= z.selmon.w.b())
                    {
                        if (!c.is_floating.now and
                            z.selmon.lt.now.arrange != null and
                            (@abs(nw - @as(i32, @intCast(c.pos.now.w))) > cfg.snap or
                                @abs(nh - @as(i32, @intCast(c.pos.now.h))) > cfg.snap))
                        {
                            toggleFloating(z, allocator, undefined);
                        }
                    }
                    if (z.selmon.lt.now.arrange == null or c.is_floating.now) {
                        var r = c.pos.now;
                        r.w = @intCast(nw);
                        r.h = @intCast(nh);
                        c.hintAndResize(r, true);
                    }
                },
                X.ButtonRelease => break,
                else => {},
            }
        }
        if (c.is_floating.now) {
            X.XWarpPointer(z.dpy, X.None, c.win, .zero, //
                @intCast(c.pos.now.w + c.bw.now - 1), //
                @intCast(c.pos.now.h + c.bw.now - 1));
        }
        X.XUngrabPointer(z.dpy, X.CurrentTime);
        while (X.XCheckMaskEvent(z.dpy, EM.EnterWindowMask, &ev)) {}
        const m_opt = c.pos.now.toMonitor(z.mons);
        if (m_opt != z.selmon) {
            if (m_opt) |m| {
                sendMon(z, allocator, c, m);
                z.selmon = m;
                focus(z, allocator, null);
            }
        }
    }
};

/// (dwm) pop
pub fn pop(z: *App, allocator: Allocator, c: *Client) void {
    c.detach();
    c.attach();
    focus(z, allocator, c);
    arrange(z, allocator, c.mon);
}

/// (dwm) drawbar
fn drawbar(z: *App, allocator: Allocator, m: *Monitor) void {
    if (!m.show_bar) return;

    var tw: u32 = 0;
    const boxs = @divTrunc(z.drw.fonts.height, 9);
    const boxw = @divTrunc(z.drw.fonts.height, 6) + 2;

    const occ = m.getOccupiedBitmask();
    const urg = m.getUrgentBitmask();

    // draw status text first so it can be overdrawn by tags later
    if (m == z.selmon) { // status text is only drawn on selected monitor
        z.drw.setScheme(z.scheme.get(.Normal));
        tw = z.TEXTW(allocator, z.stext.get());
        _ = z.drw.drawText(allocator, .{
            .x = @as(c_int, @intCast(m.w.w)) - @as(c_int, @intCast(tw)),
            .y = 0,
            .w = tw,
            .h = z.bar_height,
        }, 0, z.stext.get(), 0);
    }

    var x: i32 = 0;
    var w: u32 = 0;
    for (0..cfg.tags.len) |i| {
        w = z.TEXTW(allocator, cfg.tags[i].text);
        const current_tag = @as(u32, 1) << @intCast(i);
        const selected = m.tags & current_tag != 0;
        z.drw.setScheme(z.scheme.get(if (selected) .Selected else .Normal));
        _ = z.drw.drawText(
            allocator,
            .{ .x = x, .y = 0, .w = w, .h = z.bar_height },
            z.lrpad / 2,
            cfg.tags[i].text,
            urg & current_tag,
        );
        if ((occ & current_tag) != 0) {
            z.drw.drawRect(
                .{ .x = x + boxs, .y = boxs, .w = @intCast(boxw), .h = @intCast(boxw) },
                filled: {
                    const client = z.selmon.sel orelse break :filled false;
                    break :filled m == z.selmon and (client.tags & current_tag) != 0;
                },
                (urg & current_tag) != 0,
            );
        }
        x += @intCast(w);
    }

    w = z.TEXTW(allocator, m.lt.now.symbol);
    z.drw.setScheme(z.scheme.get(.Normal));
    x = z.drw.drawText(
        allocator,
        .{ .x = x, .y = 0, .w = w, .h = z.bar_height },
        z.lrpad / 2,
        m.lt.now.symbol,
        0,
    );

    // TODO: what if tw > m.ww?
    w = m.w.w - tw - @as(u32, @intCast(x));
    if (w > z.bar_height) {
        if (m.sel) |c| {
            const name = c.name.get();
            const r = Rect{ .x = x, .y = 0, .w = w, .h = z.bar_height };
            z.drw.setScheme(z.scheme.get(if (m == z.selmon) .Bar else .Normal));
            _ = z.drw.drawText(allocator, r, z.lrpad / 2, name, 0);
        } else {
            z.drw.setScheme(z.scheme.get(.Normal));
            z.drw.drawRect(.{ .x = x, .y = 0, .w = w, .h = z.bar_height }, true, true);
        }
    }
    z.drw.map(m.barwin, .{ .x = 0, .y = 0, .w = m.w.w, .h = z.bar_height });
}

/// Returns true if we should terminate the process immediately after this
/// function ends.
fn handleCliArgs(buffer: []u8) error{WriteFailed}!bool {
    var stdout_writer = std.fs.File.stdout().writer(buffer);
    var stdout = &stdout_writer.interface;
    const argv = std.os.argv;
    // If the only flag given is "-v", then print the version.
    if (argv.len == 2 and mem.eql(u8, mem.span(argv[1]), "-v")) {
        try stdout.print("{s}-{s}\n", .{ NAME, VERSION });
        try stdout.flush();
        return true;
    }
    // If there are any CLI args at all, print the super-minimal help text,
    // which is to either run the binary with no flags, or run it with the "-v"
    // flag.
    else if (argv.len != 1) {
        try stdout.print("usage: {s} [-v]\n", .{NAME});
        try stdout.flush();
        return true;
    }
    // Otherwise, we continue execution.
    return false;
}

/// TODO: rearrange the setup calls such that the `Drw` struct gets initialized
/// cleanly.
/// TODO: Move both setup() and cleanup() into main() so that we can rely on
/// `defer` calls to clean up resources.
pub fn main() !void {
    defer log.info("This is the final message from " ++ NAME ++ "!", .{});
    log.info("{s}", .{LINE});
    log.info("Started execution of {s}", .{NAME});
    log.info("{s}", .{LINE});

    { // Handle the CLI args, if any.
        var buffer: [64]u8 = undefined;
        if (try handleCliArgs(&buffer)) return;
    }

    // Initialize the global allocator.
    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .init;
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    if (C.setlocale(C.LC_CTYPE, "") == null or !X.XSupportsLocale()) {
        std.debug.print("warning: no locale support\n", .{});
    }

    const dpy = X.XOpenDisplay(null) orelse {
        return std.debug.print(NAME ++ ": cannot open X display\n", .{});
    };
    defer X.XCloseDisplay(dpy);
    const screen = X.DefaultScreen(dpy);

    checkOtherWM(dpy);
    setupTerminationHandling();
    setupClearZombies();

    // Initialize the X Display and Screen.
    var z: App = .{ .dpy = dpy, .screen = X.DefaultScreen(dpy) };

    // Initialize colors.
    for (std.enums.values(SchemeState)) |ss| {
        const color = try ColorScheme.init(allocator, dpy, screen, cfg.colors.get(ss));
        z.scheme.set(ss, color);
    }
    defer for (std.enums.values(SchemeState)) |ss| z.scheme.get(ss).deinit(allocator, dpy, screen);

    // Initialize dimensions, and establish first window and monitor.
    z.s = .{
        .w = @intCast(X.DisplayWidth(dpy, screen)),
        .h = @intCast(X.DisplayHeight(dpy, screen)),
    };
    z.root = X.RootWindow(dpy, screen);
    z.selmon = try Monitor.init(allocator);
    z.mons = z.selmon;
    _ = updategeom(&z);
    defer cleanupMonitors(&z, allocator);

    // Initialize atoms.
    atoms.initializeAtomsForEnum(dpy, atoms.WM, &atoms.__WM);
    atoms.initializeAtomsForEnum(dpy, atoms.Net, &atoms.__NET);

    // Supporting window for _NET_SUPPORTING_WM_CHECK. For more information,
    // see the documentation for the .WMCheck enum.
    const checkWin = X.XCreateSimpleWindow(dpy, z.root, .{ .x = 0, .y = 0, .w = 1, .h = 1 }, 0, 0, 0);
    defer X.XDestroyWindow(dpy, checkWin);
    {
        const utf8string = X.XInternAtom(dpy, "UTF8_STRING", false) orelse {
            return std.debug.print("XInternAtom failed on \"UTF8_STRING\"\n", .{});
        };
        X.XChangeProperty(dpy, z.root, atoms.net(.WMCheck), X.XA_WINDOW, 32, .Replace, @ptrCast(&checkWin), 1);
        X.XChangeProperty(dpy, checkWin, atoms.net(.WMCheck), X.XA_WINDOW, 32, .Replace, @ptrCast(&checkWin), 1);
        X.XChangeProperty(dpy, checkWin, atoms.net(.WMName), utf8string, 8, .Replace, NAME.ptr, NAME.len);
    }

    // Initialize all the fonts specified in the config.
    const fonts = try Font.initMany(allocator, dpy, screen, &cfg.fonts) orelse {
        @panic("Not a single font was valid.");
    };
    defer fonts.free(allocator);
    z.lrpad = @intCast(fonts.height);

    z.drw = try .init(.{
        .dpy = dpy,
        .screen = screen,
        .root = z.root,
        .width = z.s.w,
        .height = z.s.h,
        .fonts = fonts,
        .colors = &cfg.colors,
    });
    defer z.drw.deinit();

    // Initialize cursors.
    z.cursors.set(.Normal, X.XCreateFontCursor(dpy, .Left_ptr));
    z.cursors.set(.Resize, X.XCreateFontCursor(dpy, .Sizing));
    z.cursors.set(.Move, X.XCreateFontCursor(dpy, .Fleur));
    defer for (z.cursors.values) |cursor| X.XFreeCursor(dpy, cursor);

    // Initialize bars.
    updateBars(&z);
    updateStatus(&z, allocator);

    // EWMH support per view.
    // https://specifications.freedesktop.org/wm/1.5/
    const nets = atoms.net_array();
    X.XChangeProperty(dpy, z.root, atoms.net(.Supported), X.XA_ATOM, 32, .Replace, @ptrCast(&nets), @intCast(nets.len));
    X.XDeleteProperty(dpy, z.root, atoms.net(.ClientList));

    { // Select events.
        var wa: X.XSetWindowAttributes = .{
            .cursor = z.cursors.get(.Normal),
            .event_mask = EM.PropertyChangeMask | EM.StructureNotifyMask |
                EM.SubstructureRedirectMask | EM.SubstructureNotifyMask |
                EM.ButtonPressMask | EM.PointerMotionMask |
                EM.EnterWindowMask | EM.LeaveWindowMask,
        };
        X.XChangeWindowAttributes(dpy, z.root, CW.EventMask | CW.Cursor, &wa);
        X.XSelectInput(dpy, z.root, wa.event_mask);
    }

    grabkeys(&z);
    defer X.XUngrabKey(dpy, X.AnyKey, M.AnyModifier, z.root);
    focus(&z, allocator, null);

    // End of setup ============================================================

    try scan(&z, allocator);
    log.info("{s}", .{LINE});
    log.info("Starting event loop", .{});
    log.info("{s}", .{LINE});
    try run(&z, allocator);
}

test {
    _ = @import("tests.zig");
}
