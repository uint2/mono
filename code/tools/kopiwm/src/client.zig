const std = @import("std");
const log = std.log;
const mem = std.mem;
const Monitor = @import("monitor.zig").Monitor;
const App = @import("app.zig");
const fstr = @import("fstr.zig").fstr;
const Rect = @import("rect.zig").Rect;
const toggle = @import("toggle.zig").toggle;
const cfg = @import("config.zig");
const Size = @import("enums.zig").Size;
const X = @import("x11.zig");
const M = @import("x11.zig").masks;
const EM = @import("x11.zig").eventMask;
const CW = @import("x11.zig").CW;
const atoms = @import("atoms.zig");

const ClientSizes = struct {
    base: ?Size = null,
    /// Incremental size when resizing.
    inc: ?Size = null,
    max: ?Size = null,
    min: ?Size = null,
    /// Maximum aspect ratio (width / height).
    maxa: ?f32 = null,
    /// Minimum aspect ratio (height / width).
    /// Note that this is the reciprocal of the conventional notion of the
    /// aspect ratio because of how we'll be using it.
    mina: ?f32 = null,

    const init: @This() = .{};
};

pub const Client = struct {
    const Self = @This();
    app: *const App,

    name: fstr(256) = .empty,
    /// Position, current and previous.
    pos: toggle(Rect),
    sz: ClientSizes = .init,
    hintsvalid: bool = false,
    /// Border width.
    borderWidth: toggle(c_uint),
    /// Bitmask of active tags.
    tags: u32 = 0,
    is_fixed: bool = false,
    is_floating: toggle(bool) = .init(false),
    isurgent: bool = false,
    neverfocus: bool = false,
    isfullscreen: bool = false,
    /// Next client in the linked list of clients.
    next: ?*Self = null,
    /// Next client in the stacking order. That is, the order in which windows
    /// appear visually. If window A covers window B, or is laid on top of it,
    /// then A is before B in the stacking order.
    snext: ?*Self = null,
    /// The parent monitor to this client.
    mon: *Monitor,
    win: X.Window,

    pub fn init(
        app: *const App,
        window: X.Window,
        monitor: *Monitor,
        wa: *X.XWindowAttributes,
    ) Self {
        return Self{
            .app = app,
            .win = window,
            .mon = monitor,
            .pos = .init(.fromX(X.XWindowAttributes, wa)),
            .borderWidth = .init(@intCast(wa.border_width)),
            .is_floating = .init(false),
        };
    }

    /// (dwm) updatetitle
    pub fn updateTitle(self: *Self) void {
        const z = self.app;
        if (z.getTextProp(self.win, atoms.net(.WMName), &self.name.buffer)) |len| {
            self.name.len = len;
        } else if (z.getTextProp(self.win, X.XA_WM_NAME, &self.name.buffer)) |len| {
            self.name.len = len;
        } else {
            self.name.set("broken");
        }
    }

    /// (dwm) ISVISIBLE
    pub inline fn isVisible(self: *Self) bool {
        return self.tags & self.mon.tags != 0;
    }

    /// (dwm) seturgent
    /// Sets the client's urgent state to `urgent`.
    pub fn setUrgent(self: *Self, dpy: *X.Display, urgent: bool) void {
        self.isurgent = urgent;
        var wmh = X.XGetWMHints(dpy, self.win) orelse return;
        if (urgent) wmh.flags |= M.XUrgencyHint else wmh.flags &= ~M.XUrgencyHint;
        X.XSetWMHints(dpy, self.win, wmh);
        X.XFree(wmh);
    }

    /// Gets a pointer to the node in the linked list `self.mon.stack` that
    /// points to `self`.
    fn getStackPtr(self: *Self) *?*Self {
        var c_opt: *?*Self = &self.mon.stack;
        while (c_opt.*) |c| : (c_opt = &c.snext) {
            if (c == self) return c_opt;
        }
        @panic("Invalid state: Client pointer not found in owning stack.");
    }

    /// Gets a pointer to the node in the linked list `self.mon.clients` that
    /// points to `self`.
    fn getPtr(self: *Self) *?*Self {
        var c_opt: *?*Self = &self.mon.clients;
        while (c_opt.*) |c| : (c_opt = &c.next) {
            if (c == self) return c_opt;
        }
        @panic("Invalid state: Client pointer not found in owning list.");
    }

    /// (dwm) attach
    /// Puts `self` at the front of the Monitor's (self.mon) linked list.
    pub fn attach(self: *Self) void {
        self.next = self.mon.clients;
        self.mon.clients = self;
    }

    /// (dwm) attachstack
    /// Puts `self` at the front of the Monitor's (self.mon) linked list, but
    /// for the stack list.
    pub fn attachStack(self: *Self) void {
        self.snext = self.mon.stack;
        self.mon.stack = self;
    }

    /// (dwm) detach
    pub fn detach(self: *Self) void {
        self.getPtr().* = self.next;
    }

    /// (dwm) detachstack
    pub fn detachStack(self: *Self) void {
        self.getStackPtr().* = self.snext;

        if (self == self.mon.sel) {
            var c_opt = self.mon.stack;
            while (c_opt) |c| : (c_opt = c.snext) {
                if (c.isVisible()) {
                    break;
                }
            }
            self.mon.sel = c_opt;
        }
    }

    /// (dwm) setfocus
    pub fn setFocus(self: *Self) void {
        const z = self.app;
        if (!self.neverfocus) {
            X.XSetInputFocus(z.dpy, self.win, .PointerRoot, X.CurrentTime);
        }
        X.XChangeProperty(
            z.dpy,
            z.root,
            atoms.net(.ActiveWindow),
            X.XA_WINDOW,
            32,
            .Replace,
            @ptrCast(&self.win),
            1,
        );
        _ = self.sendEvent(atoms.wm(.TakeFocus));
    }

    /// (dwm) sendevent
    /// Returns true upon successful execution.
    pub fn sendEvent(self: *Self, proto: X.Atom) bool {
        const z = self.app;
        var exists = false;

        if (X.XGetWMProtocols(z.dpy, self.win)) |protocols| {
            defer X.XFree(protocols.ptr);
            for (protocols) |protocol| {
                exists = protocol == proto;
                if (exists) break;
            }
        }

        if (exists) {
            var ev = X.XEvent{
                .xclient = .{
                    .type = X.ClientMessage,
                    .window = self.win,
                    .message_type = atoms.wm(.Protocols),
                    .format = 32,
                },
            };
            ev.xclient.data.l[0] = @intCast(proto);
            ev.xclient.data.l[1] = X.CurrentTime;
            X.XSendEvent(z.dpy, self.win, false, EM.NoEventMask, &ev);
        }
        return exists;
    }

    /// (dwm) WIDTH
    pub inline fn width(self: *const Self) i32 {
        return @intCast(self.pos.now.w + 2 * self.borderWidth.now);
    }

    /// (dwm) HEIGHT
    pub inline fn height(self: *const Self) i32 {
        return @intCast(self.pos.now.h + 2 * self.borderWidth.now);
    }

    /// (dwm) configure
    pub fn configure(self: *const Self, dpy: *X.Display) void {
        var xconf = self.pos.now.toX(X.XConfigureEvent);
        xconf.type = X.ConfigureNotify;
        xconf.display = dpy;
        xconf.event = self.win;
        xconf.window = self.win;
        xconf.border_width = @intCast(self.borderWidth.now);
        xconf.above = X.None;
        xconf.override_redirect = X.False;
        var event = X.XEvent{ .xconfigure = xconf };
        X.XSendEvent(dpy, self.win, false, EM.StructureNotifyMask, &event);
    }

    /// (dwm) getatomprop
    fn getAtomProp(self: *Self, dpy: *X.Display, prop: X.Atom) ?X.Atom {
        // var da: X.Atom = undefined; // dummy atom.
        // var atom: X.Atom = undefined;
        // var format: c_int = undefined;
        // var nitems: c_ulong = undefined;
        // var dl: c_ulong = undefined; // dummy long.
        // var property: ?[*]u8 = undefined;

        const data = X.XGetWindowProperty(dpy, self.win, prop, 0, @sizeOf(X.Atom), false, X.XA_ATOM) orelse return null;
        defer data.deinit();
        if (data.value.len() == 0) return null;
        return switch (data.value) {
            .Fmt8 => |v| @as([*]X.Atom, @ptrCast(@alignCast(v)))[0],
            .Fmt16 => |v| @as([*]X.Atom, @ptrCast(@alignCast(v)))[0],
            .Fmt32 => |v| @as([*]X.Atom, @ptrCast(@alignCast(v)))[0],
        };
    }

    /// (dwm) setfullscreen
    pub fn setFullscreen(self: *Self, fullscreen: bool) void {
        const z = self.app;
        if (fullscreen and !self.isfullscreen) {
            X.XChangeProperty(
                z.dpy,
                self.win,
                atoms.net(.WMState),
                X.XA_ATOM,
                32,
                .Replace,
                @ptrCast(&atoms.net(.WMFullscreen)),
                1,
            );
            self.isfullscreen = true;
            self.borderWidth.set(0);
            self.is_floating.set(true);
            self.resize(self.mon.m);
            X.XRaiseWindow(self.app.dpy, self.win);
        } else if (!fullscreen and self.isfullscreen) {
            X.XChangeProperty(
                z.dpy,
                self.win,
                atoms.net(.WMState),
                X.XA_ATOM,
                32,
                .Replace,
                null,
                0,
            );
            self.isfullscreen = false;
            self.is_floating.revert();
            self.borderWidth.revert();
            self.pos.revert();
            self.resize(self.pos.now);
            // arrange(self.mon);
        }
    }

    /// (dwm) updatewindowtype
    pub fn updateWindowType(self: *Self) void {
        const z = self.app;
        if (self.getAtomProp(z.dpy, atoms.net(.WMState)) == atoms.net(.WMFullscreen)) {
            self.setFullscreen(true);
        }
        if (self.getAtomProp(z.dpy, atoms.net(.WMWindowType)) == atoms.net(.WMWindowTypeDialog)) {
            self.is_floating.set(true);
        }
    }

    /// (dwm) resizeclient
    /// Resize the X window, and also update its border width.
    pub fn resize(self: *Self, rect: Rect) void {
        const z = self.app;
        var wc = rect.toX(X.XWindowChanges);
        wc.border_width = @intCast(self.borderWidth.now);
        const flags = CW.X | CW.Y | CW.Width | CW.Height | CW.BorderWidth;
        X.XConfigureWindow(z.dpy, self.win, flags, &wc);
        self.pos.set(rect);
        self.configure(z.dpy);
        X.XSync(z.dpy, false);
    }

    /// (dwm) resize
    pub fn hintAndResize(self: *Self, target: Rect, interact: bool) void {
        var t = target;
        if (self.applySizeHints(&t, interact)) self.resize(t);
    }

    /// (dwm) applysizehints
    /// Called during client window resize operations. `rect` is the originally
    /// suggested resize target. After applying size hints, `rect` will be
    /// updated to be a more correct resize target. Returns true if the final
    /// value of `rect` differs from the client's current state.
    pub fn applySizeHints(self: *Self, rect: *Rect, interact: bool) bool {
        const c: *const Self = self;
        const m: *Monitor = self.mon;

        // Set minimum possible.
        rect.w = @max(1, rect.w);
        rect.h = @max(1, rect.h);

        const bw: i32 = @intCast(c.borderWidth.now);
        if (interact) {
            if (rect.x > c.app.s.w) {
                // left-most point is beyond the limits of the current monitor.
                rect.x = @as(i32, @intCast(c.app.s.w)) - c.width();
            }
            if (rect.y > c.app.s.h) {
                // top-most point is beyond the limits of the current monitor.
                rect.y = @as(i32, @intCast(c.app.s.h)) - c.height();
            }
            if (rect.r() + 2 * bw < 0) {
                rect.x = 0;
            }
            if (rect.b() + 2 * bw < 0) {
                rect.y = 0;
            }
        } else {
            if (rect.x >= m.w.r()) rect.x = m.w.r() - c.width();
            if (rect.y >= m.w.b()) rect.y = m.w.b() - c.height();
            if (rect.r() + 2 * bw <= m.w.x) rect.x = m.w.x;
            if (rect.b() + 2 * bw <= m.w.y) rect.y = m.w.y;
        }

        if (rect.h < c.app.bar_height) rect.h = c.app.bar_height;
        if (rect.w < c.app.bar_height) rect.w = c.app.bar_height;

        if (cfg.resizehints or c.is_floating.now or m.lt.now.arrange == null) {
            if (!c.hintsvalid) {
                self.updateSizeHints();
            }
            // dwm says: "see last two sentences in ICCCM 4.1.2.3".
            // Here is the entire last paragraph:
            // > The min_aspect and max_aspect fields are fractions with the
            // > numerator first and the denominator second, and they allow a
            // > client to specify the range of aspect ratios it prefers. Window
            // > managers that honor aspect ratios should take into account the
            // > base size in determining the preferred window size. If a base
            // > size is provided along with the aspect ratio fields, the base
            // > size should be subtracted from the window size prior to checking
            // > that the aspect ratio falls in range. If a base size is not
            // > provided, nothing should be subtracted from the window size.
            // > (The minimum size is not to be used in place of the base size
            // > for this purpose.)
            const baseismin = b: {
                const base = &(c.sz.base orelse break :b false);
                const min = &(c.sz.min orelse break :b false);
                break :b base.eq(min);
            };

            if (!baseismin) { // temporarily remove base dimensions
                if (c.sz.base) |*base| {
                    rect.w -= base.w;
                    rect.h -= base.h;
                }
            }

            { // adjust for aspect limits
                const w: f32 = @floatFromInt(rect.w);
                const h: f32 = @floatFromInt(rect.h);
                // If the aspect ratio is too large (very wide), then we reduce
                // the width to fix the ratio, and if the aspect ratio is too
                // small (very narrow), we reduce the height to make fix the
                // ratio. Both cases, we're making the window smaller.
                if (c.sz.mina) |mina| {
                    if (mina < h / w) {
                        rect.h = @intFromFloat(@as(f32, @floatFromInt(rect.w)) * mina + 0.5);
                    }
                }
                if (c.sz.maxa) |maxa| {
                    if (maxa < w / h) {
                        rect.w = @intFromFloat(@as(f32, @floatFromInt(rect.h)) * maxa + 0.5);
                    }
                }
            }
            if (baseismin) { // Increment calculation requires this.
                if (c.sz.base) |*base| {
                    rect.w -= base.w;
                    rect.h -= base.h;
                }
            }
            // Adjust for increment value.
            if (c.sz.inc) |inc| {
                rect.w -= rect.w % inc.w;
                rect.h -= rect.h % inc.h;
            }
            // Restore base dimensions.
            if (c.sz.base) |base| {
                rect.w += base.w;
                rect.h += base.h;
            }
            if (c.sz.min) |min| {
                rect.w = @max(rect.w, min.w);
                rect.h = @max(rect.h, min.h);
            }
            if (c.sz.max) |max| {
                rect.w = @min(rect.w, max.w);
                rect.h = @min(rect.h, max.h);
            }
        }
        return !c.pos.now.eq(rect);
    }

    /// (dwm) updatewmhints
    pub fn updateWMHints(self: *Self) void {
        const z = self.app;
        const wmh = X.XGetWMHints(z.dpy, self.win) orelse return;
        defer X.XFree(wmh);
        const wmh_urg = wmh.flags & M.XUrgencyHint != 0;
        if (self == z.selmon.sel and wmh_urg) {
            wmh.flags &= ~M.XUrgencyHint;
            X.XSetWMHints(z.dpy, self.win, wmh);
        } else {
            self.isurgent = wmh_urg;
        }
        if (wmh.flags & X.masks.InputHint != 0) {
            self.neverfocus = wmh.input == 0;
        } else {
            self.neverfocus = false;
        }
    }

    /// (dwm) updatesizehints
    pub fn updateSizeHints(self: *Self) void {
        const sz: *ClientSizes = &self.sz;

        const hint = X.XGetWMNormalHints(self.app.dpy, self.win) orelse hint: {
            // Size is uninitialized, ensure that size.flags aren't used.
            break :hint X.XSizeHints{ .flags = M.PSize };
        };

        // [base]
        if ((hint.flags & M.PBaseSize) != 0) {
            sz.base = .{ .w = @intCast(hint.base_width), .h = @intCast(hint.base_height) };
        } else if ((hint.flags & M.PMinSize) != 0) {
            sz.base = .{ .w = @intCast(hint.min_width), .h = @intCast(hint.min_height) };
        } else sz.base = null;

        // [inc]
        if ((hint.flags & M.PResizeInc) != 0) {
            sz.inc = .{ .w = @intCast(hint.width_inc), .h = @intCast(hint.height_inc) };
        } else sz.inc = null;

        // [max]
        if ((hint.flags & M.PMaxSize) != 0) {
            sz.max = .{ .w = @intCast(hint.max_width), .h = @intCast(hint.max_height) };
        } else sz.max = null;

        // [min]
        if ((hint.flags & M.PMinSize) != 0) {
            sz.min = .{ .w = @intCast(hint.min_width), .h = @intCast(hint.min_height) };
        } else if ((hint.flags & M.PBaseSize) != 0) {
            sz.min = .{ .w = @intCast(hint.base_width), .h = @intCast(hint.base_height) };
        } else sz.min = null;

        if ((hint.flags & X.masks.PAspect) != 0) {
            if (hint.min_aspect.y > 0) {
                sz.mina = @as(f32, @floatFromInt(hint.min_aspect.y)) / @as(f32, @floatFromInt(hint.min_aspect.x));
            }
            if (hint.max_aspect.y > 0) {
                sz.maxa = @as(f32, @floatFromInt(hint.max_aspect.x)) / @as(f32, @floatFromInt(hint.max_aspect.y));
            }
        } else {
            sz.mina = null;
            sz.maxa = null;
        }
        self.is_fixed = isfixed: {
            const max = sz.max orelse break :isfixed false;
            const min = sz.min orelse break :isfixed false;
            break :isfixed max.eq(&min);
        };
        self.hintsvalid = true;
    }

    /// (dwm) applyrules
    pub fn applyRules(self: *Self) void {
        // Rule matching.
        self.is_floating.set(false);
        self.tags = 0;
        const class_hint_opt = X.XGetClassHint(self.app.dpy, self.win);

        const class: []const u8 = if (class_hint_opt) |c| mem.span(c.res_class) else "<broken>";
        const instance: []const u8 = if (class_hint_opt) |c| mem.span(c.res_name) else "<broken>";

        for (cfg.rules) |rule| {
            var match = if (rule.title) |s| self.name.contains(s) else true;
            if (rule.class) |s| match &= mem.containsAtLeast(u8, class, 1, s);
            if (rule.instance) |s| match &= mem.containsAtLeast(u8, instance, 1, s);
            if (!match) continue;
            // Matched the rule!
            self.is_floating.set(rule.is_floating);
            self.tags |= rule.tags;
        }

        if (class_hint_opt) |class_hint| {
            X.XFree(class_hint.res_class);
            X.XFree(class_hint.res_name);
        }
        if (self.tags & cfg.TAGMASK == 0) {
            self.tags = self.mon.tags;
        } else {
            self.tags &= cfg.TAGMASK;
        }
    }

    /// (dwm) setclientstate
    pub fn setState(self: *Self, state: X.WindowState) void {
        const data: [2]c_int = .{ @intFromEnum(state), X.None };
        const z = self.app;
        X.XChangeProperty(
            z.dpy,
            self.win,
            atoms.wm(.State),
            atoms.wm(.State),
            32,
            .Replace,
            @ptrCast(&data),
            2,
        );
    }

    /// (dwm) showhide
    /// Refreshes the show-hide state of the entire linked list of Clients in
    /// the stack.
    pub fn showHide(c: *Self) void {
        log.info("showHide called on {*} ({s})", .{ c, if (c.isVisible()) "show" else "hide" });
        if (c.isVisible()) {
            // Show clients top-down.
            X.XMoveWindow(c.app.dpy, c.win, c.pos.now.x, c.pos.now.y);
            const should_resize = r: {
                if (c.isfullscreen) break :r false;
                if (c.mon.lt.now.arrange) |_| break :r true;
                break :r c.is_floating.now;
            };
            if (should_resize) c.hintAndResize(c.pos.now, false);
            if (c.snext) |next| next.showHide();
        } else {
            // Hide clients bottom up.
            if (c.snext) |next| next.showHide();
            X.XMoveWindow(c.app.dpy, c.win, c.width() * -2, c.pos.now.y);
        }
    }

    pub inline fn isTiled(self: *Self) bool {
        return !self.is_floating.now and self.isVisible();
    }

    /// (dwm) nexttiled
    /// Get the next element (possibly itself) in the linked list (given by
    /// `self.next`) that is tiled. Could be the current element, could also be
    /// null.
    pub fn nextTiled(self: *Self) ?*Self {
        var c_opt: ?*Self = self;
        while (c_opt) |c| : (c_opt = c.next) if (c.isTiled()) return c;
        return null;
    }

    /// Get the next element (NOT itself) in the linked list (given by
    /// `self.next`) that is tiled.
    pub fn nextTiledExclusive(self: *Self) ?*Self {
        return if (self.next) |c| c.nextTiled() else null;
    }
};
