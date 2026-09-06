const X = @import("x11.zig");
const DwmError = @import("errors.zig").DwmError;
const Allocator = @import("std").mem.Allocator;

/// (dwm) Clk* enums.
pub const Clk = enum {
    /// User clicked on one of the tags in the tags list (traditionally located
    /// at the top-left) in the bar window.
    TagBar,
    /// User clicked the layout symbol (traditionally located to the left of the
    /// tags) in the bar window.
    LtSymbol,
    /// User clicked the status text (traditionally located at top-right) in the
    /// bar window.
    StatusText,
    /// User clicked the window title in the bar window.
    WinTitle,
    /// User clicked on a client window.
    ClientWin,
    /// The base case: User clicked on none of the above.
    RootWin,
};

/// (dwm) Cur* enums.
/// The different possible states of the mouse cursor.
pub const CursorState = enum {
    Normal,
    Resize,
    Move,
};

pub const BarPosition = enum {
    const Self = @This();

    top,
    bottom,

    pub fn toggle(self: *const Self) Self {
        return switch (self.*) {
            .top => .bottom,
            .bottom => .top,
        };
    }
};

pub const Rule = struct {
    class: ?[]const u8,
    instance: ?[]const u8 = null,
    title: ?[]const u8 = null,
    /// Active tags bitmask.
    tags: u32,
    is_floating: bool,
};

pub const Size = struct {
    const Self = @This();
    /// Width.
    w: u32,
    /// Height.
    h: u32,
    pub const zero: Self = .{ .w = 0, .h = 0 };
    pub inline fn eq(lhs: *const Self, rhs: *const Self) bool {
        return lhs.w == rhs.w and lhs.h == rhs.h;
    }
};

pub const HandlerFnTag = enum { NoAllocE, AllocE, NoAlloc, Alloc };
pub const HandlerFn = union(HandlerFnTag) {
    NoAllocE: *const fn (*X.XEvent) DwmError!void,
    AllocE: *const fn (Allocator, *X.XEvent) DwmError!void,
    NoAlloc: *const fn (*X.XEvent) void,
    Alloc: *const fn (Allocator, *X.XEvent) void,
};
