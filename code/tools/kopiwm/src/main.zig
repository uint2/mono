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
const SchemeState = @import("enums.zig").SchemeState;
const MOUSEMASK = @import("config.zig").MOUSEMASK;
const DwmError = @import("errors.zig").DwmError;
const HandlerFn = @import("enums.zig").HandlerFn;
const atoms = @import("atoms.zig");
const X = @import("x11.zig");
const M = @import("x11.zig").masks;
const EM = @import("x11.zig").eventMask;
const CW = @import("x11.zig").CW;
const E = @import("errors.zig");
const NumLockMask = @import("numlockmask.zig").NumLockMask;

const NAME = @import("build_opts").name;
const VERSION = @import("build_opts").version;

const LINE = "----------------------------------------------------------------------";

/// C standard library.
const C = @cImport({
    @cInclude("locale.h");
    @cInclude("signal.h");
    @cInclude("unistd.h");
});

var z: App = .init();
var numlockmask: NumLockMask = .empty;

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

/// (dwm) dirtomon
/// TODO: See if we can guarantee a non-null pointer.
fn directionToMonitor(direction: Direction) ?*Monitor {
    var m_opt: ?*Monitor = null;
    switch (direction) {
        .Next => {
            m_opt = if (z.selmon.next) |t| t else z.mons;
        },
        .Prev => {
            m_opt = z.mons;
            if (z.selmon == z.mons) {
                while (m_opt) |m| : (m_opt = m.next) {}
            } else {
                while (m_opt) |m| : (m_opt = m.next) {
                    if (m.next == z.selmon) {
                        break;
                    }
                }
            }
        },
    }
    return m_opt;
}

/// (dwm) focusmon
pub fn focusMon(allocator: Allocator, arg: *const Arg) void {
    // Skip base case where there are no monitors to change focus to.
    // TODO: see if we can guarantee that `z.mons` is non-null.
    const mons = z.mons orelse return;
    if (mons.next == null) return;
    const m_opt = directionToMonitor(arg.d);
    if (m_opt == z.selmon) return;
    if (m_opt) |m| {
        unfocus(z.selmon.sel, false);
        z.selmon = m;
        focus(allocator, null);
    }
}

/// (dwm) focusstack
pub fn focusStack(allocator: Allocator, arg: *const Arg) void {
    const sel = z.selmon.sel orelse return;
    if (sel.isfullscreen and cfg.lockfullscreen) return;
    var c_opt: ?*Client = null;
    switch (arg.d) {
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
        focus(allocator, c);
        restack(allocator, z.selmon);
    }
}

/// (dwm) INTERSECT
fn intersect(x: i32, y: i32, w: i32, h: i32, m: *Monitor) i32 {
    return @max(0, @min(x + w, m.wx + @as(i32, @intCast(m.ww))) - @max(x, m.wx)) *
        @max(0, @min(y + h, m.wy + @as(i32, @intCast(m.wh))) - @max(y, m.wy));
}

/// (dwm) wintoclient
/// Searches all the monitors and all of their clients for one that matches
/// the window search query. Returns the first hit.
fn winToClient(w: X.Window) ?*Client {
    var m_opt = z.mons;
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
fn getState(w: X.Window) @typeInfo(X.WindowState).@"enum".tag_type {
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
fn manage(allocator: Allocator, w: X.Window, wa: *X.XWindowAttributes) error{OutOfMemory}!void {
    const c = try allocator.create(Client);
    c.* = .init(&z, w, z.selmon, wa);
    const transient_window = X.XGetTransientForHint(z.dpy, w);

    log.info("Created client {*}", .{c});

    c.updateTitle();
    blk: {
        if (transient_window) |_| {
            // This seems to make very little sense if there is a bijection between
            // clients and windows.
            if (winToClient(w)) |other_client| {
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

    grabbuttons(c, false);

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
        unfocus(c.mon.sel, false);
    }
    c.mon.sel = c;
    arrange(allocator, c.mon);
    X.XMapWindow(z.dpy, c.win);
    focus(allocator, null);
}

/// (dwm) unmanage
/// Destroys a client and removes it from the monitor that owns it.
fn unmanage(allocator: Allocator, c: *Client, destroyed: bool) void {
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
    focus(allocator, null);
    updateClientList();
    arrange(allocator, m);
}

/// (dwm) updateclientlist
/// Updates the ClientList property in the X server.
fn updateClientList() void {
    var m_opt = z.mons;
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
    m.layout_symbol = m.lt.now.symbol;
    if (m.lt.now.arrange) |f| {
        log.info("Arranging monitor {*} with algo \"{s}\"", .{ m, m.lt.now.symbol });
        f(m);
    }
}

/// (dwm) restack
fn restack(allocator: Allocator, m: *Monitor) void {
    drawbar(allocator, m);

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
fn arrange(allocator: Allocator, monitor: ?*Monitor) void {
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
        restack(allocator, m);
    } else {
        m_opt = monitor;
        while (m_opt) |m| : (m_opt = m.next) {
            arrangeMon(m);
        }
    }
}

/// (dwm) buttonpress
fn buttonPress(allocator: Allocator, e: *X.XEvent) DwmError!void {
    const ev: X.XButtonPressedEvent = e.xbutton;
    var click: Clk = .RootWin;
    var arg: Arg = undefined;

    // Focus monitor if necessary.
    const m = wintomon(ev.window);
    if (m != z.selmon) {
        if (z.selmon.sel) |c| unfocus(c, true);
        z.selmon = m;
        focus(allocator, null);
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
        } else if (ev.x < x + z.TEXTW(allocator, z.selmon.layout_symbol)) {
            click = .LtSymbol;
        } else if (ev.x > z.selmon.w.w - z.TEXTW(allocator, z.stext.get()) + z.lrpad - 2) {
            click = .StatusText;
        } else {
            click = .WinTitle;
        }
    }
    // Locate the click, and populate the `click` variable.
    // This block searches for the click location in the client.
    if (winToClient(ev.window)) |c| {
        focus(allocator, c);
        restack(allocator, z.selmon);
        X.XAllowEvents(z.dpy, .ReplayPointer, X.CurrentTime);
        click = .ClientWin;
    }

    // Search the `buttons` map for a hit.
    for (cfg.buttons) |*button| {
        if (button.click != click or button.button != ev.button) continue;
        if (numlockmask.cleanMask(button.mask) == numlockmask.cleanMask(ev.state)) {
            const arg2 = switch (click) {
                .TagBar => &arg,
                else => &button.lf.arg,
            };
            switch (button.lf.func) {
                .MightError => |f| try f(arg2),
                .NoError => |f| f(arg2),
                .MightErrorA => |f| try f(allocator, arg2),
                .NoErrorA => |f| f(allocator, arg2),
            }
        }
    }
}

/// (dwm) clientmessage
fn clientMessage(e: *X.XEvent) void {
    const ev: X.XClientMessageEvent = e.xclient;
    var c: *Client = winToClient(ev.window) orelse return;

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

/// (dwm) configurerequest
fn configureRequest(e: *X.XEvent) void {
    const ev = e.xconfigurerequest;
    const vmask = ev.value_mask;

    if (winToClient(ev.window)) |c| {
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

/// (dwm) configurenotify
fn configureNotify(allocator: Allocator, e: *X.XEvent) error{OutOfMemory}!void {
    const ev: X.XConfigureEvent = e.xconfigure;
    if (ev.window != z.root) return;
    const dirty = z.s.w != ev.width or z.s.h != ev.height;
    z.s.w = @intCast(ev.width);
    z.s.h = @intCast(ev.height);

    var selmon: ?*Monitor = z.selmon;
    // TODO: (dwm) updategeom handling sucks, needs to be simplified
    if ((try updategeom(allocator, &selmon)) or dirty) {
        z.drw.resize(z.s.w, z.bar_height);
        updateBars();
        var m_opt = z.mons;
        var c_opt: ?*Client = null;
        while (m_opt) |m| : (m_opt = m.next) {
            c_opt = m.clients;
            while (c_opt) |c| : (c_opt = c.next) {
                if (c.isfullscreen) {
                    c.resize(m.m);
                }
            }
            X.XMoveResizeWindow(z.dpy, m.barwin, m.w.x, m.w.y, m.w.w, z.bar_height);
        }
        focus(allocator, null);
        arrange(allocator, null);
    }
}

/// (dwm) destroynotify
fn destroyNotify(allocator: Allocator, e: *X.XEvent) void {
    const ev: X.XDestroyWindowEvent = e.xdestroywindow;
    if (winToClient(ev.window)) |c| unmanage(allocator, c, true);
}

/// (dwm) enternotify
fn enterNotify(allocator: Allocator, e: *X.XEvent) void {
    const ev: X.XCrossingEvent = e.xcrossing;
    if ((ev.mode != X.NotifyNormal or ev.detail == X.NotifyInferior) and ev.window != z.root) {
        return;
    }
    const c = winToClient(ev.window);
    const m = if (c) |client| client.mon else wintomon(ev.window);
    if (m != z.selmon) {
        unfocus(z.selmon.sel, true);
        z.selmon = m;
    } else if (c == null or c == z.selmon.sel) {
        return;
    }
    focus(allocator, c);
}

/// (dwm) expose
fn expose(allocator: Allocator, e: *X.XEvent) void {
    const ev: X.XExposeEvent = e.xexpose;
    if (ev.count == 0) {
        drawbar(allocator, wintomon(ev.window));
    }
}

/// (dwm) focusin
fn focusIn(e: *X.XEvent) void {
    const ev: X.XFocusChangeEvent = e.xfocus;
    if (z.selmon.sel) |sel| {
        if (ev.window != sel.win) sel.setFocus();
    }
}

/// (dwm) keypress
fn keyPress(allocator: Allocator, e: *X.XEvent) DwmError!void {
    const ev: X.XKeyEvent = e.xkey;
    const keysym = X.XkbKeycodeToKeysym(z.dpy, @intCast(ev.keycode), 0, 0);
    for (cfg.keys) |key| {
        if (keysym == key.sym and numlockmask.cleanMask(key.mod) == numlockmask.cleanMask(ev.state)) {
            switch (key.lf.func) {
                .MightError => |f| try f(&key.lf.arg),
                .NoError => |f| f(&key.lf.arg),
                .MightErrorA => |f| try f(allocator, &key.lf.arg),
                .NoErrorA => |f| f(allocator, &key.lf.arg),
            }
        }
    }
}

/// (dwm) mappingnotify
fn mappingNotify(e: *X.XEvent) void {
    const ev: *X.XMappingEvent = &e.xmapping;
    X.XRefreshKeyboardMapping(ev);
    if (ev.request == X.MappingKeyboard) {
        grabkeys();
    }
}

/// (dwm) maprequest
fn mapRequest(allocator: Allocator, e: *X.XEvent) error{OutOfMemory}!void {
    const ev: X.XMapRequestEvent = e.xmaprequest;
    var wa: X.XWindowAttributes = undefined;

    if (!X.XGetWindowAttributes(z.dpy, ev.window, &wa)) return;
    if (wa.override_redirect != 0) return;

    if (winToClient(ev.window) == null) {
        log.info("Start managing window {d} (mapRequest)", .{ev.window});
        try manage(allocator, ev.window, &wa);
    }
}

/// (dwm) motionnotify
fn motionNotify(allocator: Allocator, e: *X.XEvent) void {
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
                unfocus(z.selmon.sel, true);
                z.selmon = m;
                focus(allocator, null);
            }
        }
    }
    static.mon = m_opt;
}

/// (dwm) propertynotify
fn propertyNotify(allocator: Allocator, e: *X.XEvent) void {
    const ev: X.XPropertyEvent = e.xproperty;
    if (ev.window == z.root and ev.atom == X.XA_WM_NAME) {
        updateStatus(allocator);
    } else if (ev.state == X.PropertyDelete) {
        return; // ignore.
    } else if (winToClient(ev.window)) |c| {
        switch (ev.atom) {
            X.XA_WM_TRANSIENT_FOR => {
                const trans_opt = X.XGetTransientForHint(z.dpy, c.win);
                const b = !c.is_floating.now and trans_opt != null;
                if (trans_opt) |t| c.is_floating.set(winToClient(t) != null);
                if (b and c.is_floating.now) arrange(allocator, c.mon);
            },
            X.XA_WM_NORMAL_HINTS => c.hintsvalid = false,
            X.XA_WM_HINTS => {
                c.updateWMHints();
                drawbars(allocator);
            },
            else => {},
        }
        if (ev.atom == X.XA_WM_NAME or ev.atom == atoms.net(.WMName)) {
            c.updateTitle();
            if (c == c.mon.sel) {
                drawbar(allocator, c.mon);
            }
        }
        if (ev.atom == atoms.net(.WMWindowType)) {
            c.updateWindowType();
        }
    }
}

/// (dwm) unmapnotify
fn unmapNotify(allocator: Allocator, e: *X.XEvent) void {
    const ev: X.XUnmapEvent = e.xunmap;
    if (winToClient(ev.window)) |c| {
        if (ev.send_event == 0) {
            unmanage(allocator, c, false);
        } else {
            c.setState(.WithdrawnState);
        }
    }
}

/// For debugging: implement an emergency timeout in case we can't back out.
const TIMEOUT: bool = false;

/// (dwm) run
/// main event loop
fn run(allocator: Allocator) DwmError!void {
    X.XSync(z.dpy, false);
    var ev: X.XEvent = undefined;
    const start = std.time.timestamp();

    while (z.running and X.XNextEvent(z.dpy, &ev)) {
        if (TIMEOUT and @abs(std.time.timestamp() - start) > 20) @panic("End please");
        try runOne(allocator, &ev);
    }
}

inline fn runOne(allocator: Allocator, ev: *X.XEvent) DwmError!void {
    if (handler[@intCast(ev.type)]) |handler_fn| {
        switch (handler_fn) {
            .NoAllocE => |f| try f(ev),
            .AllocE => |f| try f(allocator, ev),
            .NoAlloc => |f| f(ev),
            .Alloc => |f| f(allocator, ev),
        }
    }
}

const handler: [X.LASTEvent]?HandlerFn = createHandler();
fn createHandler() [X.LASTEvent]?HandlerFn {
    var ret: [X.LASTEvent]?HandlerFn = undefined;
    var i: c_int = 0;
    while (i < ret.len) : (i += 1) {
        ret[@intCast(i)] = switch (i) {
            // zig fmt: off
            X.ButtonPress      => .{ .AllocE   = buttonPress },
            X.ClientMessage    => .{ .NoAlloc  = clientMessage },
            X.ConfigureNotify  => .{ .AllocE   = configureNotify },
            X.ConfigureRequest => .{ .NoAlloc  = configureRequest },
            X.DestroyNotify    => .{ .Alloc    = destroyNotify },
            X.EnterNotify      => .{ .Alloc    = enterNotify },
            X.Expose           => .{ .Alloc    = expose },
            X.FocusIn          => .{ .NoAlloc  = focusIn },
            X.KeyPress         => .{ .AllocE   = keyPress },
            X.MapRequest       => .{ .AllocE   = mapRequest },
            X.MappingNotify    => .{ .NoAlloc  = mappingNotify },
            X.MotionNotify     => .{ .Alloc    = motionNotify },
            X.PropertyNotify   => .{ .Alloc    = propertyNotify },
            X.UnmapNotify      => .{ .Alloc    = unmapNotify },
            // zig fmt: on
            else => null,
        };
    }
    return ret;
}

/// (dwm) scan
fn scan(allocator: Allocator) error{OutOfMemory}!void {
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
        if (wa.map_state == X.IsViewable or getState(wins[i]) == X.IconicState) {
            log.info("Start managing window {d} (scan, non-transient)", .{wins[i]});
            try manage(allocator, wins[i], &wa);
        }
    }
    i = 0;
    while (i < wins.len) : (i += 1) { // now the transients
        if (!X.XGetWindowAttributes(z.dpy, wins[i], &wa)) continue;
        if (X.XGetTransientForHint(z.dpy, wins[i]) == null) continue;
        const viewable = wa.map_state == X.IsViewable;
        const iconic = getState(wins[i]) == X.IconicState;
        if (viewable or iconic) {
            log.info("Start managing window {d} (scan, transient)", .{wins[i]});
            try manage(allocator, wins[i], &wa);
        }
    }
}

/// (dwm) incnmaster
pub fn incNMaster(allocator: Allocator, arg: *const Arg) void {
    const i = switch (arg.*) {
        .i => |v| v,
        else => return,
    };
    z.selmon.nmaster = @intCast(@max(@as(i32, @intCast(z.selmon.nmaster)) + i, 0));
    arrange(allocator, z.selmon);
}

/// (dwm) killclient
pub fn killClient(_: *const Arg) void {
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

/// (dwm) movemouse
pub fn moveMouse(allocator: Allocator, _: *const Arg) DwmError!void {
    var c = z.selmon.sel orelse return;
    if (c.isfullscreen) return; // No support moving fullscreen windows by mouse.
    restack(allocator, z.selmon);

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
            X.Expose | X.MapRequest | X.ConfigureRequest => try runOne(allocator, &ev),
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
                    toggleFloating(allocator, undefined);
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
            sendMon(allocator, c, m);
            z.selmon = m;
            focus(allocator, null);
        }
    }
}

/// (dwm) setlayout
pub fn setLayout(allocator: Allocator, arg: *const Arg) void {
    // TODO: check all other instances of tagged access of args. Make sure to
    // use a switch statement before indexing.
    const lt = switch (arg.*) {
        .l => |lt| lt,
        else => return,
    };
    z.selmon.lt.now = lt;
    z.selmon.layout_symbol = lt.symbol;
    if (z.selmon.sel) |_| {
        arrange(allocator, z.selmon);
    } else {
        drawbar(allocator, z.selmon);
    }
}

/// (dwm) setmfact
pub fn setMFact(allocator: Allocator, arg: *const Arg) void {
    if (z.selmon.lt.now.arrange == null) return;
    const f: f32 = switch (arg.*) {
        .f => |v| v,
        else => return,
    };
    if (0.05 <= f and f <= 0.95) {
        z.selmon.mfact = f;
        arrange(allocator, z.selmon);
    }
}

/// (dwm) resizemouse
pub fn resizeMouse(allocator: Allocator, _: *const Arg) DwmError!void {
    var c = z.selmon.sel orelse return;
    if (c.isfullscreen) return; // No support moving fullscreen windows by mouse.
    restack(allocator, z.selmon);

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
            X.Expose | X.MapRequest | X.ConfigureRequest => try runOne(allocator, &ev),
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
                        arrange(allocator, z.selmon);
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
                        toggleFloating(allocator, undefined);
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
            sendMon(allocator, c, m);
            z.selmon = m;
            focus(allocator, null);
        }
    }
}

/// (dwm) sendmon
fn sendMon(allocator: Allocator, c: *Client, m: *Monitor) void {
    if (c.mon == m) return;
    unfocus(c, true);
    c.detach();
    c.detachStack();
    c.mon = m;
    c.tags = m.tags; // Assign tags of target monitor.
    c.attach();
    c.attachStack();
    if (c.isfullscreen) {
        c.resize(m.w);
    }
    focus(allocator, null);
    arrange(allocator, null);
}

/// (dwm) togglefloating
pub fn toggleFloating(allocator: Allocator, _: *const Arg) void {
    const sel = z.selmon.sel orelse return;
    if (sel.isfullscreen) return; // No support for making fullscreen windows float.
    sel.is_floating.set(!sel.is_floating.now or sel.is_fixed);
    if (sel.is_floating.now) {
        sel.hintAndResize(sel.pos.now, false);
    }
    arrange(allocator, z.selmon);
}

/// (dwm) wintomon
fn wintomon(w: X.Window) *Monitor {
    if (w == z.root) {
        if (z.getRootPtr()) |coords| {
            const r = Rect{ .x = @intCast(coords.x), .y = @intCast(coords.y), .w = 1, .h = 1 };
            // To guarantee a non-null return of `*Monitor`, we deviate a tad from
            // dwm's behaviour and return `selmon` if nothing is found.
            return r.toMonitor(z.mons) orelse (z.mons orelse unreachable);
        }
    }
    var m_opt = z.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        if (w == m.barwin) return m;
    }
    if (winToClient(w)) |c| return c.mon;
    return z.selmon;
}

/// (dwm) updategeom
fn updategeom(allocator: Allocator, selmon: *?*Monitor) error{OutOfMemory}!bool {
    var dirty = false;
    var mons: *Monitor = undefined;
    {
        // default monitor setup
        mons = z.mons orelse m: {
            z.mons = try Monitor.init(allocator);
            break :m z.mons.?;
        };
        if (mons.m.w != z.s.w or mons.m.h != z.s.h) {
            dirty = true;
            mons.w.w = z.s.w;
            mons.w.h = z.s.h;
            mons.m.w = z.s.w;
            mons.m.h = z.s.h;
            mons.updateBarPosition(z.bar_height);
        }
    }
    if (dirty) {
        selmon.* = mons;
        selmon.* = wintomon(z.root);
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
fn unfocus(client: ?*Client, setfocus: bool) void {
    const c = client orelse return;
    log.info("Unfocusing client at: {*}", .{c});
    grabbuttons(c, false);
    X.XSetWindowBorder(z.dpy, c.win, z.scheme.get(.Normal).border.pixel);
    if (setfocus) {
        X.XSetInputFocus(z.dpy, z.root, .PointerRoot, X.CurrentTime);
        X.XDeleteProperty(z.dpy, z.root, atoms.net(.ActiveWindow));
    }
}

/// (dwm) focus
fn focus(allocator: Allocator, client: ?*Client) void {
    if (client) |c| {
        log.info("focus({*})", .{c});
    } else log.info("focus(null)", .{});

    var c_opt = client;
    if (if (c_opt) |c| !c.isVisible() else true) {
        // If `client` is null or it's invisible, then push the pointer forward
        // until c_opt points to the first visible client.
        c_opt = z.selmon.stack;
        while (c_opt) |c| : (c_opt = c.snext) {
            if (c.isVisible()) break;
        }
    }
    // If the currently selected client in the selected monitor is not `c_opt`,
    // then unfocus it.
    if (z.selmon.sel) |sel| {
        if (sel != c_opt) {
            unfocus(sel, false);
        }
    }
    if (c_opt) |c| {
        z.selmon = c.mon;
        // if the client (that's about to be focused) is urgent, then put it at
        // ease for it is about to be tended to.
        if (c.isurgent) c.setUrgent(z.dpy, false);
        c.detachStack();
        c.attachStack();
        grabbuttons(c, true);
        X.XSetWindowBorder(z.dpy, c.win, z.scheme.get(.Selected).border.pixel);
        c.setFocus();
    } else {
        X.XSetInputFocus(z.dpy, z.root, .PointerRoot, X.CurrentTime);
        X.XDeleteProperty(z.dpy, z.root, atoms.net(.ActiveWindow));
    }
    z.selmon.sel = c_opt;
    drawbars(allocator);
}

/// (dwm) drawbars
fn drawbars(allocator: Allocator) void {
    var m_opt: ?*Monitor = z.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        drawbar(allocator, m);
    }
}

/// (dwm) grabbuttons
fn grabbuttons(c: *Client, focused: bool) void {
    numlockmask.update(z.dpy);
    X.XUngrabButton(z.dpy, X.AnyButton, M.AnyModifier, c.win);
    const bmask = EM.ButtonPressMask | EM.ButtonReleaseMask;
    if (!focused) {
        X.XGrabButton(z.dpy, X.AnyButton, M.AnyModifier, c.win, false, bmask, .Sync, .Sync, X.None, X.None);
    }
    for (cfg.buttons) |button| {
        if (button.click == .ClientWin) {
            for (numlockmask.modifiers) |modifier| {
                X.XGrabButton(z.dpy, button.button, button.mask | modifier, c.win, false, bmask, .Async, .Sync, X.None, X.None);
            }
        }
    }
}

/// (dwm) grabkeys
fn grabkeys() void {
    numlockmask.update(z.dpy);

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
                for (numlockmask.modifiers) |mod| {
                    _ = X.XGrabKey(z.dpy, keycode, key.mod | mod, z.root, true, .Async, .Async);
                }
            }
        }
    }
}

/// (dwm) cleanup
// Continue to build this up as we go.
fn cleanup(allocator: Allocator, wmcheckwin: X.Window) void {
    // View all clients at once. ~0 yields a bitmask of all high bits. I don't
    // fully understand why we do this yet, but I think it helps with clearing
    // out the clients.
    view(allocator, &.{ .ui = ~@as(u32, 0) });
    z.selmon.lt.set(&.{ .symbol = "", .arrange = null });

    var m_opt = z.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        while (m.stack) |c| {
            unmanage(allocator, c, false);
        }
    }
    X.XUngrabKey(z.dpy, X.AnyKey, M.AnyModifier, z.root);
    while (z.mons) |mon| {
        cleanupmon(allocator, mon);
    }
    for (z.cursors.values) |cursor| {
        X.XFreeCursor(z.dpy, cursor);
    }
    for (std.enums.values(SchemeState)) |ss| {
        z.drw.scmFree(allocator, z.scheme.get(ss));
    }
    X.XDestroyWindow(z.dpy, wmcheckwin);
    z.drw.deinit(allocator);
    X.XSync(z.dpy, false);
    X.XSetInputFocus(z.dpy, X.PointerRoot, .PointerRoot, X.CurrentTime);
    X.XDeleteProperty(z.dpy, z.root, atoms.net(.ActiveWindow));
}

/// (dwm) cleanupmon
fn cleanupmon(allocator: Allocator, mon: *Monitor) void {
    // First, remove `mon` from the linked list that is `z.mons`.
    if (mon == z.mons) {
        z.mons = z.mons.?.next;
    } else {
        var m_opt = z.mons;
        while (m_opt) |m| : (m_opt = m.next) {
            if (m.next == mon) {
                m.next = mon.next;
                break;
            }
        }
    }
    X.XUnmapWindow(z.dpy, mon.barwin);
    X.XDestroyWindow(z.dpy, mon.barwin);
    log.warn("Deallocate monitor: {*}", .{mon});
    allocator.destroy(mon);
}

/// (dwm) updatebars
fn updateBars() void {
    var wa: X.XSetWindowAttributes = .{
        .override_redirect = X.True,
        .background_pixmap = X.ParentRelative,
        .event_mask = EM.ButtonPressMask | EM.ExposureMask,
    };
    var ch = z.classHint();
    var m_opt = z.mons;
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
fn updateStatus(allocator: Allocator) void {
    if (z.getTextProp(z.root, X.XA_WM_NAME, &z.stext.buffer)) |len| {
        z.stext.len = len;
    } else {
        z.stext.set(NAME ++ "-" ++ VERSION);
    }
    drawbar(allocator, z.selmon);
}

/// (dwm) spawn
pub fn spawn(arg: *const Arg) void {
    const args = switch (arg.*) {
        .args => |value| value,
        else => return,
    };
    const pid = std.posix.fork() catch unreachable;
    if (pid == 0) {
        _ = C.close(X.ConnectionNumber(z.dpy));
        _ = C.setsid();

        var sa: C.struct_sigaction = undefined;
        _ = C.sigemptyset(&sa.sa_mask);
        sa.sa_flags = 0;
        sa.__sigaction_handler.sa_handler = C.SIG_DFL;
        _ = C.sigaction(C.SIGCHLD, &sa, null);
        std.posix.execvpeZ(args[0].?, args, std.c.environ) catch {
            @panic("execvp failed.");
        };
    }
}

/// (dwm) tag
pub fn tag(allocator: Allocator, arg: *const Arg) void {
    const mask = switch (arg.*) {
        .ui => |v| if (v & cfg.TAGMASK != 0) v else return,
        else => return,
    };
    if (z.selmon.sel) |c| {
        c.tags = mask & cfg.TAGMASK;
        focus(allocator, null);
        arrange(allocator, z.selmon);
    }
}

/// (dwm) tagmon
pub fn tagMonitor(allocator: Allocator, arg: *const Arg) void {
    const direction = switch (arg.*) {
        .d => |v| v,
        else => return,
    };

    const sel = z.selmon.sel orelse return;
    const mons = z.mons orelse return;
    if (mons.next == null) return;

    if (directionToMonitor(direction)) |m| {
        sendMon(allocator, sel, m);
    }
}

/// (dwm) view
/// Views a certain tag mask.
pub fn view(allocator: Allocator, arg: *const Arg) void {
    const mask = switch (arg.*) {
        .ui => |mask| b: {
            // This mask is expected to only have one high bit.
            if (mask != ~@as(@TypeOf(mask), 0) and @popCount(mask) != 1) {
                log.err("view() received a mask of {x}", .{mask});
            }
            break :b mask & cfg.TAGMASK;
        },
        else => return,
    };
    log.info("view with bitmask: {b}", .{mask});
    if (mask == z.selmon.tags) {
        return; // nothing to do here.
    } else if (mask != 0) {
        z.selmon.tags = mask;
    }
    focus(allocator, null);
    arrange(allocator, z.selmon);
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

/// (dwm) togglebar
pub fn toggleBar(allocator: Allocator, _: *const Arg) void {
    z.selmon.show_bar = !z.selmon.show_bar;
    z.selmon.updateBarPosition(z.bar_height);
    X.XMoveResizeWindow2(z.dpy, z.selmon.barwin, z.barRect());
    arrange(allocator, z.selmon);
}

pub fn toggleBarPosition(allocator: Allocator, _: *const Arg) void {
    z.selmon.bar_pos = z.selmon.bar_pos.toggle();
    z.selmon.updateBarPosition(z.bar_height);
    X.XMoveResizeWindow2(z.dpy, z.selmon.barwin, z.barRect());
    arrange(allocator, z.selmon);
}

/// (dwm) toggletag
pub fn toggleTag(allocator: Allocator, arg: *const Arg) void {
    const mask = switch (arg.*) {
        .ui => |v| v,
        else => return,
    };
    const sel = z.selmon.sel orelse return;
    const newtags = sel.tags ^ (mask & cfg.TAGMASK);
    if (newtags != 0) {
        sel.tags = newtags;
        focus(allocator, null);
        arrange(allocator, z.selmon);
    }
}

/// (dwm) toggleview
pub fn toggleView(allocator: Allocator, arg: *const Arg) void {
    const mask = switch (arg.*) {
        .ui => |v| v & cfg.TAGMASK,
        else => return,
    };
    const newtagset = z.selmon.tags ^ mask;
    if (newtagset != 0) {
        z.selmon.tags = newtagset;
        focus(allocator, null);
        arrange(allocator, z.selmon);
    }
}

/// (dwm) pop
pub fn pop(allocator: Allocator, c: *Client) void {
    c.detach();
    c.attach();
    focus(allocator, c);
    arrange(allocator, c.mon);
}

/// (dwm) quit
pub fn quit(_: *const Arg) void {
    log.info("{s}", .{LINE});
    log.info("quit() called.", .{});
    z.running = false;
}

/// (dwm) zoom
pub fn zoom(allocator: Allocator, _: *const Arg) void {
    var c: ?*Client = z.selmon.sel orelse return;
    if (c.?.is_floating.now or z.selmon.lt.now.arrange == null) return;
    const nextTiled: ?*Client = if (z.selmon.clients) |x| x.nextTiled() else null;
    if (c == nextTiled) {
        c = if (c.?.next) |x| x.nextTiled() else null;
        if (c == null) {
            return;
        }
    }
    pop(allocator, c.?);
}

/// (dwm) drawbar
fn drawbar(allocator: Allocator, m: *Monitor) void {
    if (!m.show_bar) return;

    var tw: u32 = 0;
    const boxs = z.drw.fonts.h / 9;
    const boxw = z.drw.fonts.h / 6 + 2;
    var occ: u32 = 0; // it's a bitmask.
    var urg: u32 = 0; // it's a bitmask.

    // draw status first so it can be overdrawn by tags later
    if (m == z.selmon) { // status is only drawn on selected monitor
        z.drw.setScheme(z.scheme.get(.Normal));
        tw = z.TEXTW(allocator, z.stext.get());
        _ = z.drw.drawText(allocator, .{
            .x = @as(i32, @intCast(m.w.w)) - @as(i32, @intCast(tw)),
            .y = 0,
            .w = tw,
            .h = z.bar_height,
        }, 0, z.stext.get(), 0);
    }

    var c_opt = m.clients;
    while (c_opt) |c| : (c_opt = c.next) {
        occ |= c.tags;
        if (c.isurgent) urg |= c.tags;
    }

    var x: i32 = 0;
    var w: u32 = 0;
    for (0..cfg.tags.len) |i| {
        w = z.TEXTW(allocator, cfg.tags[i].text);
        const tag_mask = @as(u32, 1) << @intCast(i);
        const selected = m.tags & tag_mask != 0;
        z.drw.setScheme(z.scheme.get(if (selected) .Selected else .Normal));
        _ = z.drw.drawText(
            allocator,
            .{ .x = x, .y = 0, .w = w, .h = z.bar_height },
            z.lrpad / 2,
            cfg.tags[i].text,
            urg & tag_mask,
        );
        if ((occ & tag_mask) != 0) {
            z.drw.drawRect(
                .{ .x = x + boxs, .y = boxs, .w = boxw, .h = boxw },
                filled: {
                    const client = z.selmon.sel orelse break :filled false;
                    break :filled m == z.selmon and (client.tags & tag_mask) != 0;
                },
                (urg & tag_mask) != 0,
            );
        }
        x += @intCast(w);
    }

    w = z.TEXTW(allocator, m.layout_symbol);
    z.drw.setScheme(z.scheme.get(.Normal));
    x = z.drw.drawText(
        allocator,
        .{ .x = x, .y = 0, .w = w, .h = z.bar_height },
        z.lrpad / 2,
        m.layout_symbol,
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

    checkOtherWM(dpy);
    setupTerminationHandling();
    setupClearZombies();

    z.dpy = dpy;
    z.screen = X.DefaultScreen(dpy);
    z.s = .{
        .w = @intCast(X.DisplayWidth(dpy, z.screen)),
        .h = @intCast(X.DisplayHeight(dpy, z.screen)),
    };
    z.root = X.RootWindow(dpy, z.screen);

    { // Initialize selmon, and make sure it's non-null.
        var selmon: ?*Monitor = null;
        _ = try updategeom(allocator, &selmon);
        z.selmon = selmon orelse {
            return std.debug.print(NAME ++ ": could not find the first selected monitor\n", .{});
        };
    }

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

    z.drw = try .init(allocator, .{
        .dpy = dpy,
        .screen = z.screen,
        .root = z.root,
        .width = z.s.w,
        .height = z.s.h,
        .fonts = &cfg.fonts,
        .colors = &cfg.colors,
    });
    z.lrpad = z.drw.fonts.h;

    // Initialize appearance.
    for (std.enums.values(SchemeState)) |ss|
        z.scheme.set(ss, try z.drw.scmCreate(allocator, cfg.colors.get(ss)));

    // Initialize cursors.
    z.cursors.set(.Normal, X.XCreateFontCursor(dpy, .Left_ptr));
    z.cursors.set(.Resize, X.XCreateFontCursor(dpy, .Sizing));
    z.cursors.set(.Move, X.XCreateFontCursor(dpy, .Fleur));

    // Initialize bars.
    updateBars();
    updateStatus(allocator);

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

    grabkeys();
    focus(allocator, null);

    // End of setup ============================================================

    try scan(allocator);
    log.info("{s}", .{LINE});
    log.info("Starting event loop", .{});
    log.info("{s}", .{LINE});
    try run(allocator);
}

const X_TUTORIAL_SOURCE: []const u8 = @embedFile("x11.zig");

test "All inline functions for docs" {
    var iter = std.mem.splitScalar(u8, X_TUTORIAL_SOURCE, '\n');
    while (iter.next()) |line| {
        // We don't want any non-inlined functions because they really are just
        // macros of X11 library functions that contain docs.
        const contains_pub_fn = std.mem.containsAtLeast(u8, line, 1, "pub fn");
        try std.testing.expect(!contains_pub_fn);
    }
}

test "All inline functions have sources" {
    var iter = std.mem.splitScalar(u8, X_TUTORIAL_SOURCE, '\n');
    const n = 2;
    var prev: [n]?[]const u8 = undefined;
    while (iter.next()) |line| {
        if (std.mem.startsWith(u8, line, "pub inline fn")) {
            log.info("line: {s}", .{line});
            try std.testing.expectStringStartsWith(prev[1].?, "///");
            try std.testing.expectStringStartsWith(prev[0].?, "/// source: https://");
        }
        @memmove(prev[1..n], prev[0 .. n - 1]);
        prev[0] = line;
    }
}

// test "All struct docs have sources" {
//     var iter = std.mem.splitScalar(u8, X_TUTORIAL_SOURCE, '\n');
//     const n = 2;
//     var prev: [n]?[]const u8 = undefined;
//     while (iter.next()) |line| {
//         const select = std.mem.startsWith(u8, line, "pub const X") and
//             std.mem.containsAtLeast(u8, line, 1, "= X.");
//         if (select) {
//             log.info("line: {s}", .{line});
//             try std.testing.expectStringStartsWith(prev[1].?, "///");
//             try std.testing.expectStringStartsWith(prev[0].?, "/// source: https://x.org/");
//         }
//         @memmove(prev[1..n], prev[0 .. n - 1]);
//         prev[0] = line;
//     }
// }

test "Don't use weird quotes" {
    const q = @import("quotes.zig");
    try std.testing.expect(!std.mem.containsAtLeastScalar(u8, X_TUTORIAL_SOURCE, 1, q.Q_APOSTROPHE));
    try std.testing.expect(!std.mem.containsAtLeastScalar(u8, X_TUTORIAL_SOURCE, 1, q.Q_RIGHT_SINGLE_QUOTATION_MARK));
    try std.testing.expect(!std.mem.containsAtLeastScalar(u8, X_TUTORIAL_SOURCE, 1, q.Q_LEFT_SINGLE_QUOTATION_MARK));
    try std.testing.expect(!std.mem.containsAtLeastScalar(u8, X_TUTORIAL_SOURCE, 1, q.Q_RIGHT_DOUBLE_QUOTATION_MARK));
    try std.testing.expect(!std.mem.containsAtLeastScalar(u8, X_TUTORIAL_SOURCE, 1, q.Q_LEFT_DOUBLE_QUOTATION_MARK));
}
