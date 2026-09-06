const std = @import("std");
const log = std.log;
const Allocator = std.mem.Allocator;
const X = @import("x11.zig");

/// Represents a possible which one might be in that warrants a unique color scheme.
pub const SchemeState = enum {
    Normal,
    Selected,
    Bar,
};

pub fn Scheme(comptime T: type) type {
    return struct {
        const Self = @This();
        /// Foreground color.
        fg: T,
        /// Background color.
        bg: T,
        /// Border color.
        border: T,
    };
}

fn clrCreate(dpy: *X.Display, screen: c_int, dest: *X.XftColor, color_name: []const u8) void {
    const result = X.XftColorAllocName(
        dpy,
        X.DefaultVisual(dpy, screen),
        X.DefaultColormap(dpy, screen),
        color_name,
        dest,
    );
    if (!result) {
        std.debug.print("error, cannot allocate color '{s}'\n", .{color_name});
        std.process.exit(1);
    }
    // Force maximum opacity.
    dest.pixel |= 0xff << 24;
}

pub const ColorScheme = struct {
    const Self = @This();

    /// Foreground color.
    fg: X.XftColor,
    /// Background color.
    bg: X.XftColor,
    /// Border color.
    border: X.XftColor,

    pub fn init(
        allocator: Allocator,
        dpy: *X.Display,
        screen: c_int,
        scheme: Scheme([]const u8),
    ) error{OutOfMemory}!*Self {
        var ret = try allocator.create(Self);
        clrCreate(dpy, screen, &ret.fg, scheme.fg);
        clrCreate(dpy, screen, &ret.bg, scheme.bg);
        clrCreate(dpy, screen, &ret.border, scheme.border);
        return ret;
    }

    pub fn deinit(self: *Self, allocator: Allocator, dpy: *X.Display, screen: c_int) void {
        const visual = X.DefaultVisual(dpy, screen);
        const cmap = X.DefaultColormap(dpy, screen);
        X.XftColorFree(dpy, visual, cmap, &self.fg);
        X.XftColorFree(dpy, visual, cmap, &self.bg);
        X.XftColorFree(dpy, visual, cmap, &self.border);
        log.warn("Deallocate color scheme: {*}", .{self});
        allocator.destroy(self);
    }
};
