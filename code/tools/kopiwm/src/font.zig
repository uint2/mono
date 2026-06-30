const std = @import("std");
const log = std.log;
const mem = std.mem;
const Allocator = mem.Allocator;

const X = @import("x11.zig");

const FCE = error{FontCreateError};

/// The `Self` type must have a member `.next: *Self`. That's all.
/// Note that LSP may not work on all methods in here because we're receiving
/// `Self` as a parameter, instead of using the usual `const Self = @This();`.
pub fn LinkedList(Self: type) type {
    return struct {
        /// Gets the last element in the linked list.
        pub inline fn back(self: *Self) *Self {
            var ptr: *Self = self;
            while (self.next) |next| : (ptr = next) {}
            return ptr;
        }

        /// Appends a value to the end of the list.
        pub fn pushBack(self: *Self, value: *Self) void {
            self.back().next = value;
        }
    };
}

/// A linked list of fonts (including itself).
pub const Font = struct {
    const Self = @This();

    const C = LinkedList(Self);
    pub const back = C.back;
    pub const pushBack = C.pushBack;

    /// The connection to the X server.
    dpy: *X.Display,

    /// Standardized height of the font as computed at initialization.
    height: c_int = 0,

    /// X's font struct.
    xfont: *X.XftFont,

    /// The next font in the linked list.
    next: ?*Self = null,

    /// (dwm) xfont_create
    pub fn initFromName(
        allocator: Allocator,
        dpy: *X.Display,
        screen: c_int,
        fontName: []const u8,
    ) error{ OutOfMemory, FontCreateError }!*Self {
        if (fontName.len == 0) {
            std.debug.print("No font specified.", .{});
            return error.FontCreateError;
        }
        var self = try allocator.create(Self);
        self.dpy = dpy;
        // Using the pattern found at font->xfont->pattern does not yield the
        // same substitution results as using the pattern returned by
        // FcNameParse; using the latter results in the desired fallback
        // behaviour whereas the former just results in missing-character
        // rectangles being drawn, at least with some fonts.
        self.xfont = X.XftFontOpenName(dpy, screen, fontName) orelse {
            std.debug.print("error, cannot load font from name: '{s}'\n", .{fontName});
            return error.FontCreateError;
        };
        self.xfont.pattern = X.FcNameParse(fontName) orelse {
            std.debug.print("error, cannot parse font name to pattern: '{s}'\n", .{fontName});
            X.XftFontClose(dpy, self.xfont);
            return error.FontCreateError;
        };
        self.height = self.xfont.ascent + self.xfont.descent;
        return self;
    }

    /// (dwm) xfont_create
    pub fn initFromPattern(
        allocator: Allocator,
        dpy: *X.Display,
        font_pattern: *X.FcPattern,
    ) error{ OutOfMemory, FontCreateError }!*Self {
        var self = try allocator.create(Self);
        self.dpy = dpy;
        self.xfont = X.XftFontOpenPattern(dpy, font_pattern) orelse {
            std.debug.print("error, cannot load font from pattern\n", .{});
            return error.FontCreateError;
        };
        self.height = self.xfont.ascent + self.xfont.descent;
        return self;
    }

    pub fn deinit(self: *Self, allocator: Allocator) void {
        log.warn("Trying to deallocate font: {*}", .{self});
        if (self.xfont.pattern) |pattern| {
            X.FcPatternDestroy(pattern);
        }
        X.XftFontClose(self.dpy, self.xfont);
        log.warn("Deallocate font: {*}", .{self});
        allocator.destroy(self);
    }

    /// Frees the entire linked list of fonts.
    ///
    /// (dwm) drw_fontset_free
    pub fn freeAll(self: *Self, allocator: Allocator) void {
        if (self.next) |next| next.freeAll(allocator);
        self.deinit(allocator);
    }

    /// (dwm) drw_font_getexts
    ///
    /// Gets the extents of an utf-8 encoded string. It is on the user to ensure
    /// that `utf8.str` is indeed utf-8 encoded.
    pub fn getExtents(self: *const Self, utf8str: []const u8, w: ?*u32, h: ?*u32) void {
        if (utf8str.len == 0) {
            if (w) |w_ptr| w_ptr.* = 0;
            if (h) |h_ptr| h_ptr.* = 0;
            return;
        }
        var ext: X.XGlyphInfo = undefined;
        X.XftTextExtentsUtf8(self.dpy, self.xfont, utf8str, &ext);
        if (w) |w_ptr| w_ptr.* = @intCast(ext.xOff);
        if (h) |h_ptr| h_ptr.* = @intCast(self.height); // Standardized height.
    }

    /// (dwm) xfont_create
    pub fn fromPattern(self: *Self, dpy: *X.Display, font_pattern: *X.FcPattern) FCE!void {
        const xfont = X.XftFontOpenPattern(dpy, font_pattern) orelse {
            std.debug.print("error, cannot load font from pattern\n", .{});
            return error.FontCreateError;
        };

        self.xfont = xfont;
        self.height = xfont.ascent + xfont.descent;
        self.dpy = dpy;
    }

    /// (dwm) drw_fontset_create
    /// Builds the linked list of fonts such that the first font provided in
    /// the `fonts` slice is at the head of the linked list.
    pub fn initMany(
        allocator: Allocator,
        dpy: *X.Display,
        screen: c_int,
        fonts: []const []const u8,
    ) error{ OutOfMemory, FontCreateError }!?*Self {
        if (fonts.len == 0) return null;
        var ret: ?*Font = null;
        var cur: *Font = undefined;
        var it = std.mem.reverseIterator(fonts);
        while (it.next()) |fontName| {
            cur = try Font.initFromName(allocator, dpy, screen, fontName);
            cur.next = ret;
            ret = cur;
        }
        return ret;
    }

    /// Find the first font that supports the UTF-8 codepoint requested.
    pub fn getFontThatHasChar(self: *Self, utf8Codepoint: u21) ?*Font {
        var f_opt: ?*Font = self;
        while (f_opt) |f| : (f_opt = f.next) {
            if (f.xftCharExists(self.dpy, utf8Codepoint)) return f;
        }
        return null;
    }

    pub inline fn xftCharExists(self: *const Self, dpy: *X.Display, codepoint: u21) bool {
        return X.XftCharExists(dpy, self.xfont, @intCast(codepoint));
    }
};
