const std = @import("std");
const mem = std.mem;
const log = std.log;

const App = @import("app.zig");
const cfg = @import("config.zig");
const toggle = @import("toggle.zig").toggle;
const lt = @import("layout.zig");
const Layout = lt.Layout;
const Client = @import("client.zig").Client;
const BarPosition = @import("enums.zig").BarPosition;

const X = @import("x11.zig");
const CW = @import("x11.zig").CW;
const EM = @import("x11.zig").eventMask;
const Allocator = std.mem.Allocator;
const Rect = @import("rect.zig").Rect;

pub const Monitor = struct {
    const Self = @This();
    /// Master window factor.
    mfact: f32 = cfg.mfact,
    /// Number of master windows.
    nmaster: u32 = cfg.nmaster,
    /// Status bar's y-coordinate.
    by: c_int = undefined,
    /// The Rect that every pixel on the monitor lives in.
    m: Rect = .zero,
    /// The Rect that windows live in. This is simply the monitor's Rect minus
    /// the status bar's Rect.
    w: Rect = .zero,
    /// The bitmask of visible tags. Initialize with the first tag visible.
    tags: u32 = cfg.tagMask(0),
    /// false means hide bar.
    show_bar: bool = cfg.show_bar,
    bar_pos: BarPosition = cfg.bar_pos,
    /// Linked list of clients.
    clients: ?*Client = null,
    /// Selected client
    sel: ?*Client = null,
    /// Clients ordered by stacking order. That is, the order in which windows
    /// appear visually. If window A covers window B, or is laid on top of it,
    /// then A is before B in the stacking order.
    stack: ?*Client = null,

    next: ?*Self = null,
    barwin: X.Window = 0,
    lt: toggle(*const Layout),

    /// (dwm) createmon
    ///
    /// As of this initialization, there are no ties to anything X-related yet.
    /// So this can be called even without a Display ready.
    pub fn init(allocator: Allocator) error{OutOfMemory}!*Self {
        const m = try allocator.create(Self);
        m.* = .{
            .lt = .init(&cfg.layouts[0]),
        };
        std.log.info("Initialized a monitor!", .{});
        return m;
    }

    pub fn deinit(self: *Self, allocator: Allocator, dpy: *X.Display) void {
        X.XUnmapWindow(dpy, self.barwin);
        X.XDestroyWindow(dpy, self.barwin);
        log.warn("Deallocate monitor: {*}", .{self});
        allocator.destroy(self);
    }

    /// Checks if the currently selected client.
    pub fn tagMaskIsActive(self: *Self, mask: u32) bool {
        const sel = self.sel orelse return false;
        return (sel.tags & mask) != 0;
    }

    /// Count the number of clients (including this one) that are tiled.
    pub fn countTiledClients(self: *Self) u32 {
        var c = self.clients orelse return 0;
        var n: u32 = if (c.isTiled()) 1 else 0;
        while (c.nextTiledExclusive()) |next| : (c = next) n += 1;
        return n;
    }

    /// (dwm) updatebarpos
    pub fn updateBarPosition(self: *Monitor, bar_height: c_uint) void {
        if (!self.show_bar) {
            // If the bar is not shown, then the dimensions of the windows
            // display area simply become the entire monitor.
            self.w = self.m;
            // Send the bar out of the screen.
            self.by = self.m.y - 2 * @as(c_int, @intCast(bar_height));
            return;
        }

        // Otherwise, the height of the display area is shortened by precisely
        // the bar height.
        self.w.h = self.m.h - bar_height;

        switch (self.bar_pos) {
            .top => {
                self.by = self.m.y;
                self.w.y = self.m.y + @as(c_int, @intCast(bar_height));
            },
            .bottom => {
                self.by = self.m.b() - @as(c_int, @intCast(bar_height));
                self.w.y = self.m.y;
            },
        }
    }

    /// Gets the occupancy bitmask. A high bit indicates that there exists a
    /// client at that tag.
    pub fn getOccupiedBitmask(self: *const Self) u32 {
        var mask: u32 = 0;
        var c_opt = self.clients;
        while (c_opt) |c| : (c_opt = c.next) {
            mask |= c.tags;
        }
        return mask;
    }

    /// Gets the urgent bitmask. A high bit indicates that there exists a client
    /// flagged as urgent at that tag.
    pub fn getUrgentBitmask(self: *const Self) u32 {
        var mask: u32 = 0;
        var c_opt = self.clients;
        while (c_opt) |c| : (c_opt = c.next) {
            if (c.isurgent) mask |= c.tags;
        }
        return mask;
    }

    /// Gets the first visible client based on stacking order.
    pub fn firstVisibleClient(self: *const Self) ?*Client {
        var c_opt = self.stack;
        while (c_opt) |c| : (c_opt = c.snext) {
            if (c.isVisible()) return c;
        }
        return null;
    }

    /// (dwm) arrangemon
    pub fn arrange(self: *Monitor, allocator: Allocator, z: *App) error{OutOfMemory}!void {
        if (self.stack) |c| c.showHide(z);
        self.startArrange(z);
        try self.restack(allocator, z);
    }

    /// (dwm) arrangemon
    ///
    /// Calls the arrange function stored inside of the layout member.
    pub fn startArrange(self: *Self, z: *const App) void {
        (self.lt.now.arrange orelse return)(z, self);
    }

    /// (dwm) drawbar
    pub fn drawbar(self: *Self, allocator: Allocator, z: *App) error{OutOfMemory}!void {
        if (!self.show_bar) return;

        var tw: u32 = 0;
        const boxs = @divTrunc(z.drw.fonts.height, 9);
        const boxw = @divTrunc(z.drw.fonts.height, 6) + 2;

        const occ = self.getOccupiedBitmask();
        const urg = self.getUrgentBitmask();

        // draw status text first so it can be overdrawn by tags later
        if (self == z.selmon) { // status text is only drawn on selected monitor
            z.drw.setScheme(z.scheme.get(.Normal));
            tw = try z.TEXTW(allocator, z.stext.get());
            _ = try z.drw.drawText(allocator, .{
                .x = @as(c_int, @intCast(self.w.w)) - @as(c_int, @intCast(tw)),
                .y = 0,
                .w = tw,
                .h = z.bar_height,
            }, 0, z.stext.get(), 0);
        }

        var x: i32 = 0;
        var w: u32 = 0;
        for (0..cfg.tags.len) |i| {
            w = try z.TEXTW(allocator, cfg.tags[i].text);
            const current_tag = @as(u32, 1) << @intCast(i);
            const selected = self.tags & current_tag != 0;
            z.drw.setScheme(z.scheme.get(if (selected) .Selected else .Normal));
            _ = try z.drw.drawText(
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
                        break :filled self == z.selmon and (client.tags & current_tag) != 0;
                    },
                    (urg & current_tag) != 0,
                );
            }
            x += @intCast(w);
        }

        w = try z.TEXTW(allocator, self.lt.now.symbol);
        z.drw.setScheme(z.scheme.get(.Normal));
        x = try z.drw.drawText(
            allocator,
            .{ .x = x, .y = 0, .w = w, .h = z.bar_height },
            z.lrpad / 2,
            self.lt.now.symbol,
            0,
        );

        // TODO: what if tw > self.ww?
        w = self.w.w - tw - @as(u32, @intCast(x));
        if (w > z.bar_height) {
            if (self.sel) |c| {
                const name = c.name.get();
                const r = Rect{ .x = x, .y = 0, .w = w, .h = z.bar_height };
                z.drw.setScheme(z.scheme.get(if (self == z.selmon) .Bar else .Normal));
                _ = try z.drw.drawText(allocator, r, z.lrpad / 2, name, 0);
            } else {
                z.drw.setScheme(z.scheme.get(.Normal));
                z.drw.drawRect(.{ .x = x, .y = 0, .w = w, .h = z.bar_height }, true, true);
            }
        }
        z.drw.map(self.barwin, .{ .x = 0, .y = 0, .w = self.w.w, .h = z.bar_height });
    }

    /// (dwm) restack
    ///
    /// Puts the selected client at the top of the stacking order, so that no other
    /// window obscures it. And then also syncs our stacking order with X's.
    pub fn restack(self: *Monitor, allocator: Allocator, z: *App) error{OutOfMemory}!void {
        try self.drawbar(allocator, z);

        const has_arrange = self.lt.now.arrange != null;

        const sel = self.sel orelse return;
        if (sel.is_floating.now or !has_arrange) {
            X.XRaiseWindow(z.dpy, sel.win);
        }
        if (has_arrange) {
            var wc = X.XWindowChanges{ .sibling = self.barwin, .stack_mode = X.Below };
            var c_opt = self.stack;
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
};
