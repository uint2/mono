const std = @import("std");
const Direction = @import("lazy_fn.zig").Direction;
const log = std.log;
const build_opts = @import("build_opts");
const NumLockMask = @import("numlockmask.zig").NumLockMask;
const X = @import("x11.zig");
const Rect = @import("rect.zig").Rect;
const SchemeState = @import("color_scheme.zig").SchemeState;
const CursorState = @import("enums.zig").CursorState;
const Size = @import("enums.zig").Size;
const Coordinates = @import("enums.zig").Coordinates;
const fstr = @import("fstr.zig").fstr;
const Client = @import("client.zig").Client;
const Allocator = std.mem.Allocator;
const EnumArray = @import("enum_array.zig").EnumArray;
const cfg = @import("config.zig");

const Drw = @import("drw.zig").Drw;
const ColorScheme = @import("color_scheme.zig").ColorScheme;
const Monitor = @import("monitor.zig").Monitor;

const Self = @This();

// Note to new Zig learners: if we try to deference this, we get "error:
// cannot dereference undefined value."
dpy: *X.Display = undefined,

screen: c_int = 0,

/// Screen size.
/// Apparently dwm updates this in `void configurenotify(XEvent *)`, and
/// that's probably how multipe monitors are supported.
s: Size = .zero,

drw: Drw = undefined,

/// Left-right padding.
lrpad: u32 = 0,

bar_height: u32 = cfg.bar_height,

/// Linked list of monitors.
mons: *Monitor = undefined,

/// Selected monitor.
selmon: *Monitor = undefined,

root: X.Window = 0,

cursors: EnumArray(CursorState, X.Cursor) = .empty,
scheme: EnumArray(SchemeState, *ColorScheme) = .empty,

/// Status bar text.
stext: fstr(256) = .empty,

numlockmask: NumLockMask = .empty,

running: bool = true,

pub fn init(allocator: Allocator, dpy: *X.Display, screen: c_int) error{OutOfMemory}!Self {
    const m0 = try Monitor.init(allocator);
    return Self{
        .dpy = dpy,
        .screen = screen,
        .selmon = m0,
        .mons = m0,
    };
}

/// (dwm) TEXTW
pub fn TEXTW(self: *Self, allocator: Allocator, text: []const u8) u32 {
    return self.drw.fontSetGetWidth(allocator, text) + self.lrpad;
}

/// (dwm) getrootptr
pub fn getRootPtr(self: *const Self) ?Coordinates(c_int) {
    // XQueryPointer returns the root window the pointer is logically on and
    // the pointer coordinates relative to the root window's origin.
    const res = X.XQueryPointer(self.dpy, self.root);
    return if (res.win_pos) |_| res.root_pos else null;
}

/// Gets the property of a window in text form, and writes it to `buffer`.
/// Returns the number of valid bytes written to the buffer.
/// (dwm) gettextprop
pub fn getTextProp(self: *const Self, w: X.Window, atom: X.Atom, buffer: []u8) ?usize {
    if (buffer.len == 0) return null;
    const text_property = X.XGetTextProperty(self.dpy, w, atom) orelse return null;
    if (text_property.nitems == 0) {
        return null;
    }
    var l: ?usize = null;
    if (text_property.encoding == X.XA_STRING) {
        const value: []const u8 = std.mem.span(text_property.value);
        l = @min(value.len, buffer.len);
        @memcpy(buffer[0..l.?], value[0..l.?]);
    } else {
        if (X.XmbTextPropertyToTextList(self.dpy, &text_property)) |list| {
            const value: []const u8 = std.mem.span(list[0]);
            l = @min(value.len, buffer.len);
            @memcpy(buffer[0..l.?], value[0..l.?]);
            X.XFreeStringList(list.ptr);
        }
    }
    X.XFree(text_property.value);
    return l;
}

/// Gets the Rect for the status bar (window).
pub fn barRect(self: *const Self) Rect {
    return .{
        .x = self.selmon.w.x,
        .y = self.selmon.by,
        .w = self.selmon.w.w,
        .h = self.bar_height,
    };
}

/// Tries to find the best client to focus, based on the suggestion.
/// * If the suggested client is visible and non-null, then all's good.
/// * If it's invisible, check that client's parent monitor for the first
///   visible client.
/// * Otherwise, check the selected monitor for the first visible client.
/// * Give up and return null.
pub fn resolveFocus(self: *const Self, suggested: ?*Client) ?*Client {
    if (suggested) |suggestedClient| {
        if (suggestedClient.isVisible()) {
            return suggestedClient;
        } else if (suggestedClient.mon.firstVisibleClient()) |client| {
            return client;
        } else if (self.selmon == suggestedClient.mon) {
            return null;
        }
    }
    return self.selmon.firstVisibleClient();
}

/// (dwm) dirtomon
///
/// If there are multiple monitors, then go to the next/prev one. Otherwise,
/// do not move (i.e. stay on selmon).
pub fn getMonitorFromDirection(self: *const Self, direction: Direction) *Monitor {
    switch (direction) {
        .Next => return if (self.selmon.next) |t| t else self.selmon,
        .Prev => {
            var m = self.mons;
            if (self.selmon == self.mons) {
                // Send pointer to the end of the linked list.
                while (m.next) |next| : (m = next) {}
                return m;
            } else {
                // Advance pointer to just before the selected one.
                while (m.next) |next| : (m = next) if (next == self.selmon) return m;
                // The monitor list is corrupted because we couldn't retrieve
                // selmon from traversing the linked list from the start. So
                // selmon is dangling.
                @panic("Corrupted monitor linked list.");
            }
        },
    }
}

/// (dwm) wintoclient
/// Searches all the monitors and all of their clients for one that matches
/// the window search query. Returns the first hit.
pub fn winToClient(self: *const Self, w: X.Window) ?*Client {
    var m_opt: ?*Monitor = self.mons;
    var c_opt: ?*Client = null;
    while (m_opt) |m| : (m_opt = m.next) {
        c_opt = m.clients;
        while (c_opt) |c| : (c_opt = c.next) {
            if (c.win == w) return c;
        }
    }
    return null;
}
