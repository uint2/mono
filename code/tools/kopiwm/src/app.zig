const std = @import("std");
const log = std.log;
const atoms = @import("atoms.zig");
const Direction = @import("lazy_fn.zig").Direction;
const build_opts = @import("build_opts");
const NumLockMask = @import("numlockmask.zig").NumLockMask;
const X = @import("x11.zig");
const EM = @import("x11.zig").eventMask;
const CW = @import("x11.zig").CW;
const M = @import("x11.zig").masks;
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
const Clk = @import("enums.zig").Clk;

const NAME = @import("build_opts").name;
const VERSION = @import("build_opts").version;

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
pub fn TEXTW(self: *Self, allocator: Allocator, text: []const u8) error{OutOfMemory}!u32 {
    return try self.drw.fontSetGetWidth(allocator, text) + self.lrpad;
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

/// (dwm) arrange
pub fn arrangeAllMonitors(self: *Self) void {
    var m_opt: ?*Monitor = self.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        if (m.stack) |c| c.showHide(self);
    }
    m_opt = self.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        m.startArrange(self);
    }
}

/// (dwm) drawbars
///
/// Updates all the status bars across all monitors.
pub fn drawbars(self: *Self, allocator: Allocator) error{OutOfMemory}!void {
    var m_opt: ?*Monitor = self.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        try m.drawbar(allocator, self);
    }
}

/// (dwm) wintomon
///
/// Obtains the monitor that a window is on. Falls back to the selected monitor.
pub fn windowToMonitor(self: *Self, w: X.Window) *Monitor {
    if (w == self.root) {
        if (self.getRootPtr()) |coords| {
            const r = Rect{ .x = coords.x, .y = coords.y, .w = 1, .h = 1 };
            // To guarantee a non-null return of `*Monitor`, we deviate a tad from
            // dwm's behaviour and return `selmon` if nothing is found.
            return r.toMonitor(self.mons) orelse self.selmon;
        }
    }
    var m_opt: ?*Monitor = self.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        if (w == m.barwin) return m;
    }
    if (self.winToClient(w)) |c| return c.mon;
    return self.selmon;
}

/// (dwm) focus
///
/// Draws the focus to one particular client. If no `client` is provided (null),
/// then focus any client that is visible.
pub fn resolveClientAndFocus(self: *Self, allocator: Allocator, client: ?*Client) error{OutOfMemory}!void {
    log.info("Focusing a client...", .{});

    const target = self.resolveFocus(client) orelse {
        log.info("No focus target found.", .{});
        X.XSetInputFocus(self.dpy, self.root, .PointerRoot, X.CurrentTime);
        X.XDeleteProperty(self.dpy, self.root, atoms.net(.ActiveWindow));
        self.selmon.sel = null;
        try self.drawbars(allocator);
        return;
    };

    log.info("Focusing client @ {*}", .{target});
    target.focus(self);
    try self.drawbars(allocator);
}

/// (dwm) updateclientlist
/// Updates the ClientList property in the X server.
pub fn updateClientList(self: *const Self) void {
    var m_opt: ?*Monitor = self.mons;
    var c_opt: ?*Client = undefined;
    // Delete the existing list.
    X.XDeleteProperty(self.dpy, self.root, atoms.net(.ClientList));
    // Rebuild the list.
    while (m_opt) |m| : (m_opt = m.next) {
        c_opt = m.clients;
        while (c_opt) |c| : (c_opt = c.next) {
            X.XChangeProperty(
                self.dpy,
                self.root,
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

/// (dwm) updatestatus
pub fn updateStatus(self: *Self, allocator: Allocator) error{OutOfMemory}!void {
    if (self.getTextProp(self.root, X.XA_WM_NAME, &self.stext.buffer)) |len| {
        self.stext.len = len;
    } else {
        self.stext.set(NAME ++ "-" ++ VERSION);
    }
    try self.selmon.drawbar(allocator, self);
}

/// (dwm) updatebars
pub fn updateBars(self: *Self) void {
    var wa: X.XSetWindowAttributes = .{
        .override_redirect = X.True,
        .background_pixmap = X.ParentRelative,
        .event_mask = EM.ButtonPressMask | EM.ExposureMask,
    };
    const static = struct {
        var name: [NAME.len]u8 = staticInit();
        fn staticInit() [NAME.len]u8 {
            var buf: [NAME.len]u8 = undefined;
            @memcpy(&buf, NAME);
            return buf;
        }
    };
    var ch: X.XClassHint = .{ .res_class = &static.name, .res_name = &static.name };
    var m_opt: ?*Monitor = self.mons;
    while (m_opt) |m| : (m_opt = m.next) {
        if (m.barwin != 0) {
            continue;
        }
        m.barwin = X.XCreateWindow(
            self.dpy,
            self.root,
            m.w.x,
            m.by,
            m.w.w,
            self.bar_height,
            0,
            X.DefaultDepth(self.dpy, self.screen),
            X.CopyFromParent,
            X.DefaultVisual(self.dpy, self.screen),
            CW.OverrideRedirect | CW.BackPixmap | CW.EventMask,
            &wa,
        );
        log.info("Create bar window({d}): (x={d}, y={d}, w={d}, h={d})", .{
            m.barwin,
            m.w.x,
            m.by,
            m.w.w,
            self.bar_height,
        });
        X.XDefineCursor(self.dpy, m.barwin, self.cursors.get(.Normal));
        X.XMapRaised(self.dpy, m.barwin);
        X.XSetClassHint(self.dpy, m.barwin, &ch);
    }
}

/// (dwm) grabkeys
pub fn grabkeys(self: *Self) void {
    self.numlockmask.update(self.dpy);

    var start: c_int = undefined; // or, X.KeyCode
    var end: c_int = undefined; // or, X.KeyCode
    var skip: c_int = undefined;

    X.XUngrabKey(self.dpy, X.AnyKey, M.AnyModifier, self.root);
    X.XDisplayKeycodes(self.dpy, &start, &end);
    const syms = X.XGetKeyboardMapping(self.dpy, @intCast(start), end - start + 1, &skip) orelse return;
    defer X.XFree(syms);

    var keycode = start;
    while (keycode < end) : (keycode += 1) {
        for (cfg.keys) |key| {
            // Skip modifier codes, we do that ourselves.
            if (key.sym == syms[@intCast((keycode - start) * skip)]) {
                for (self.numlockmask.modifiers) |mod| {
                    _ = X.XGrabKey(self.dpy, keycode, key.mod | mod, self.root, true, .Async, .Async);
                }
            }
        }
    }
}

/// (dwm) updategeom
pub fn updategeom(self: *Self) bool {
    var dirty = false;
    var mons = self.mons;
    if (mons.m.w != self.s.w or mons.m.h != self.s.h) {
        dirty = true;
        mons.w.w = self.s.w;
        mons.w.h = self.s.h;
        mons.m.w = self.s.w;
        mons.m.h = self.s.h;
        mons.updateBarPosition(self.bar_height);
    }
    if (dirty) {
        self.selmon = mons;
        self.selmon = self.windowToMonitor(self.root);
    }
    return dirty;
}

pub fn resolveClick(self: *Self, allocator: Allocator, ev: *const X.XButtonPressedEvent) error{OutOfMemory}!struct {
    /// Location of the click.
    loc: Clk,
    /// The tag that was clicked, as a bitmask.
    tagMask: u32,
} {
    var click: Clk = .RootWin;
    var tagMask: u32 = 0;

    // Focus monitor if necessary.
    const m = self.windowToMonitor(ev.window);
    if (m != self.selmon) {
        if (self.selmon.sel) |c| c.unfocus(self, true);
        self.selmon = m;
        try self.resolveClientAndFocus(allocator, null);
    }

    // This block searches for the click location in the bar window. That is
    // the status bar area.
    if (ev.window == self.selmon.barwin) {
        var i: usize = 0;
        var x: u32 = 0;
        while (true) {
            x += try self.TEXTW(allocator, cfg.tags[i].text);
            if (ev.x >= x) {
                i += 1;
                if (i < cfg.tags.len) continue;
            }
            break;
        }
        if (i < cfg.tags.len) {
            click = .TagBar;
            tagMask = @as(u32, 1) << @intCast(i);
        } else if (ev.x < x + try self.TEXTW(allocator, self.selmon.lt.now.symbol)) {
            click = .LtSymbol;
        } else if (ev.x > self.selmon.w.w - try self.TEXTW(allocator, self.stext.get()) + self.lrpad - 2) {
            click = .StatusText;
        } else {
            click = .WinTitle;
        }
    }
    // This block searches for the click location in the client.
    else if (self.winToClient(ev.window)) |c| {
        try self.resolveClientAndFocus(allocator, c);
        try self.selmon.restack(allocator, self);
        X.XAllowEvents(self.dpy, .ReplayPointer, X.CurrentTime);
        click = .ClientWin;
    }
    return .{ .loc = click, .tagMask = tagMask };
}
