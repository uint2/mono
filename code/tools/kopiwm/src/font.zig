const std = @import("std");
const log = std.log;
const mem = std.mem;
const Allocator = mem.Allocator;

const X = @import("x11.zig");

const FCE = error{FontCreateError};

/// This represents a linked list of fonts.
pub const Font = struct {
    const Self = @This();

    dpy: *X.Display,
    /// Standardized height of the font as computed at initialization.
    height: c_int,
    xfont: *X.XftFont,
    next: ?*Self,

    pub fn deinit(self: *Self, allocator: Allocator) void {
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
    pub fn free(self: *Self, allocator: Allocator) void {
        if (self.next) |next| next.free(allocator);
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
    pub fn fromName(self: *Self, dpy: *X.Display, screen: c_int, font_name: []const u8) FCE!void {
        if (font_name.len == 0) {
            std.debug.print("No font specified.", .{});
            return error.FontCreateError;
        }

        // Using the pattern found at font->xfont->pattern does not yield the
        // same substitution results as using the pattern returned by
        // FcNameParse; using the latter results in the desired fallback
        // behaviour whereas the former just results in missing-character
        // rectangles being drawn, at least with some fonts.
        const xfont = X.XftFontOpenName(dpy, screen, font_name) orelse {
            std.debug.print("error, cannot load font from name: '{s}'\n", .{font_name});
            return error.FontCreateError;
        };
        xfont.pattern = X.FcNameParse(font_name) orelse {
            std.debug.print("error, cannot parse font name to pattern: '{s}'\n", .{font_name});
            X.XftFontClose(dpy, xfont);
            return error.FontCreateError;
        };

        self.xfont = xfont;
        self.height = xfont.ascent + xfont.descent;
        self.dpy = dpy;
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
        var it = std.mem.reverseIterator(fonts);
        while (it.next()) |fontName| {
            var cur = try allocator.create(Font);
            try cur.fromName(dpy, screen, fontName);
            cur.next = ret;
            ret = cur;
        }
        return ret;
    }
};
