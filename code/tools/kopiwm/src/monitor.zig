const std = @import("std");
const mem = std.mem;
const log = std.log;

const cfg = @import("config.zig");
const toggle = @import("toggle.zig").toggle;
const lt = @import("layout.zig");
const Layout = lt.Layout;
const Client = @import("client.zig").Client;
const BarPosition = @import("enums.zig").BarPosition;

const X = @import("x11.zig");
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

    /// Count the number of clients that are tiled.
    pub fn countTiledClients(self: *Self) u32 {
        var c = self.clients orelse return 0;
        var n: u32 = 0;
        while (c.nextTiled()) |nt| {
            // We found the next tiled client (i.e. `nt`), and so we add one to
            // the count.
            n += 1;
            // But we cannot use `nt` again because the next tiled client of
            // `nt` would be itself, resulting in an infinite loop.
            c = nt.next orelse break;
        }
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
};
