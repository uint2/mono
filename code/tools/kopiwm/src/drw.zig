const std = @import("std");
const log = std.log;
const X = @import("x11.zig");
const Rect = @import("rect.zig").Rect;
const EnumArray = @import("enum_array.zig").EnumArray;
const SchemeState = @import("enums.zig").SchemeState;
const mem = std.mem;
const Allocator = mem.Allocator;

// TODO: change this to Font when all is said and done.
/// This represents a linked list of fonts.
pub const Font = struct {
    const Self = @This();

    dpy: *X.Display,
    /// Standardized height of the font as computed at initialization.
    height: u16,
    xfont: *X.XftFont,
    pattern: ?*X.FcPattern,
    next: ?*Font,

    /// (dwm) drw_font_getexts
    ///
    /// Gets the extents of an utf-8 encoded string. It is on the user to ensure
    /// that `utf8str` is indeed utf-8 encoded.
    pub fn getExtents(self: *Self, utf8str: []const u8, w: ?*u32, h: ?*u32) void {
        if (utf8str.len == 0) {
            if (w) |w_ptr| w_ptr.* = 0;
            if (h) |h_ptr| h_ptr.* = 0;
            return;
        }
        var ext: X.XGlyphInfo = undefined;
        X.XftTextExtentsUtf8(self.dpy, self.xfont, utf8str, &ext);
        if (w) |w_ptr| w_ptr.* = @intCast(ext.xOff);
        if (h) |h_ptr| h_ptr.* = self.height; // Standardized height.
    }
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

pub const ColorScheme = Scheme(X.XftColor);

/// (dwm) drw_fontset_create
/// Builds the list of fonts such that the first font provided in the
/// `fonts` slice is at the head of the linked list.
pub fn fontsetCreate(
    allocator: Allocator,
    dpy: *X.Display,
    screen: c_int,
    fonts: []const []const u8,
) error{ OutOfMemory, FontCreateError }!?*Font {
    if (fonts.len == 0) return null;
    var ret: ?*Font = null;
    var it = std.mem.reverseIterator(fonts);
    while (it.next()) |font| {
        const cur = try xfontCreate(allocator, dpy, screen, font, null);
        cur.next = ret;
        ret = cur;
    }
    return ret;
}

/// (dwm) xfont_create
fn xfontCreate(
    allocator: Allocator,
    dpy: *X.Display,
    screen: c_int,
    fontname: []const u8,
    font_pattern: ?*X.FcPattern,
) error{ OutOfMemory, FontCreateError }!*Font {
    var xfont: ?*X.XftFont = null;
    var pattern: ?*X.FcPattern = null;

    if (fontname.len > 0) {
        // Using the pattern found at font->xfont->pattern does not yield the
        // same substitution results as using the pattern returned by
        // FcNameParse; using the latter results in the desired fallback
        // behaviour whereas the former just results in missing-character
        // rectangles being drawn, at least with some fonts.
        xfont = X.XftFontOpenName(dpy, screen, fontname) orelse {
            std.debug.print("error, cannot load font from name: '{s}'\n", .{fontname});
            return error.FontCreateError;
        };
        pattern = X.FcNameParse(fontname) orelse {
            std.debug.print("error, cannot parse font name to pattern: '{s}'\n", .{fontname});
            X.XftFontClose(dpy, xfont.?);
            return error.FontCreateError;
        };
    } else if (font_pattern) |fp| {
        xfont = X.XftFontOpenPattern(dpy, fp) orelse {
            std.debug.print("error, cannot load font from pattern\n", .{});
            return error.FontCreateError;
        };
    } else {
        std.debug.print("No font specified.", .{});
        return error.FontCreateError;
    }

    var font = try allocator.create(Font);
    font.xfont = xfont orelse unreachable;
    font.pattern = pattern;
    font.height = @intCast(xfont.?.ascent);
    font.height += @intCast(xfont.?.descent);
    font.dpy = dpy;

    return font;
}

/// (dwm) xfont_free
fn xfontFree(allocator: Allocator, font: *Font) void {
    if (font.pattern) |pattern| {
        X.FcPatternDestroy(pattern);
    }
    X.XftFontClose(font.dpy, font.xfont);
    log.warn("Deallocate font: {*}", .{font});
    allocator.destroy(font);
}

pub const DrwInitParams = struct {
    dpy: *X.Display,
    screen: c_int,
    /// Root Window.
    root: X.Window,
    width: c_uint,
    height: c_uint,
    fonts: []const []const u8,
    colors: *const EnumArray(SchemeState, Scheme([]const u8)),
};

/// TODO: use an array for fonts instead of linked list.
pub const Drw = struct {
    const Self = @This();

    dpy: *X.Display,
    screen: c_int,
    root: X.Window,
    drawable: X.Drawable,
    gc: X.GC,

    /// Width.
    w: c_uint,
    /// Height.
    h: c_uint,
    /// The current state of scheme.
    scheme: ?*ColorScheme = null,
    /// A linked list of fonts.
    fonts: *Font,

    pub fn init(allocator: Allocator, p: DrwInitParams) error{ OutOfMemory, FontCreateError }!Self {
        const depth = X.DefaultDepth(p.dpy, p.screen);
        const fonts = try fontsetCreate(allocator, p.dpy, p.screen, p.fonts) orelse {
            // Empty linked list. No fonts loaded.
            std.debug.print("no fonts could be loaded.\n", .{});
            return error.FontCreateError;
        };
        const drw: Self = .{
            .dpy = p.dpy,
            .screen = p.screen,
            .root = p.root,
            .drawable = X.XCreatePixmap(p.dpy, p.root, p.width, p.height, @intCast(depth)),
            .gc = X.XCreateGC(p.dpy, p.root, 0, undefined),
            .w = p.width,
            .h = p.height,
            .fonts = fonts,
        };
        X.XSetLineAttributes(p.dpy, drw.gc, 1, .Solid, .Butt, .Miter);
        return drw;
    }

    /// (dwm) drw_free
    pub fn deinit(self: *Self, allocator: Allocator) void {
        X.XFreePixmap(self.dpy, self.drawable);
        X.XFreeGC(self.dpy, self.gc);
        fontsetFree(allocator, self.fonts);
    }

    /// (dwm) drw_resize
    /// Resize drawing area.
    pub fn resize(self: *Self, w: u32, h: u32) void {
        self.w = w;
        self.h = h;
        if (self.drawable != 0) {
            X.XFreePixmap(self.dpy, self.drawable);
        }
        self.drawable = X.XCreatePixmap(
            self.dpy,
            self.root,
            w,
            h,
            @intCast(X.DefaultDepth(self.dpy, self.screen)),
        );
    }

    /// (dwm) drw_fontset_free
    pub fn fontsetFree(allocator: Allocator, set: ?*Font) void {
        if (set) |f| {
            fontsetFree(allocator, f.next);
            xfontFree(allocator, f);
        }
    }

    /// (dwm) drw_clr_create
    pub fn clrCreate(self: *Self, dest: *X.XftColor, color_name: []const u8) void {
        const result = X.XftColorAllocName(
            self.dpy,
            X.DefaultVisual(self.dpy, self.screen),
            X.DefaultColormap(self.dpy, self.screen),
            color_name,
            dest,
        );
        if (!result) {
            std.debug.print("error, cannot allocate color '{s}'\n", .{color_name});
            std.process.exit(1);
        }
        dest.pixel |= 0xff << 24;
    }

    /// (dwm) drw_clr_free
    pub fn clrFree(self: *Self, c: *X.XftColor) void {
        X.XftColorFree(
            self.dpy,
            X.DefaultVisual(self.dpy, self.screen),
            X.DefaultColormap(self.dpy, self.screen),
            c,
        );
    }

    /// (dwm) drw_scm_create
    pub fn scmCreate(
        self: *Self,
        allocator: Allocator,
        scheme: Scheme([]const u8),
    ) error{OutOfMemory}!*ColorScheme {
        var ret = try allocator.create(ColorScheme);
        self.clrCreate(&ret.fg, scheme.fg);
        self.clrCreate(&ret.bg, scheme.bg);
        self.clrCreate(&ret.border, scheme.border);
        return ret;
    }

    /// (dwm) drw_scm_free
    pub fn scmFree(self: *Self, allocator: Allocator, scheme: *ColorScheme) void {
        self.clrFree(&scheme.fg);
        self.clrFree(&scheme.bg);
        self.clrFree(&scheme.border);
        log.warn("Deallocate color scheme: {*}", .{scheme});
        allocator.destroy(scheme);
    }

    /// (dwm) drw_setscheme
    pub fn setScheme(self: *Self, scheme: *ColorScheme) void {
        self.scheme = scheme;
    }

    /// (dwm) drw_setfontset
    pub fn setFontSet(self: *Self, set: *Font) void {
        self.fonts = set;
    }

    /// (dwm) drw_rect
    pub fn drawRect(self: *Self, rect: Rect, filled: bool, invert: bool) void {
        const scheme = self.scheme orelse return;
        const color = if (invert) scheme.bg.pixel else scheme.fg.pixel;
        X.XSetForeground(self.dpy, self.gc, color);
        if (filled) {
            X.XFillRectangle(self.dpy, self.drawable, self.gc, rect);
        } else {
            X.XDrawRectangle(self.dpy, self.drawable, self.gc, .{
                .x = rect.x,
                .y = rect.y,
                .w = rect.w - 1,
                .h = rect.h - 1,
            });
        }
    }

    /// (dwm) drw_text
    /// Question: Is `invert` a bitmask? or a boolean? or a numerical value?
    /// Because based on dwm's source code all three cases kinda doesn't fit.
    ///
    /// This draws the text onto the abstract drawable first, not directly to
    /// any screen, and so the coordinates will (later) be made relative
    /// to whichever screen the drawable gets mapped to.
    pub fn drawText(
        self: *Self,
        allocator: Allocator,
        rect: Rect,
        lpad: u32,
        text_to_draw: []const u8,
        invert: u32,
    ) i32 {
        const INVALID = "�";
        var text: []const u8 = text_to_draw;
        var x = rect.x;
        const y = rect.y;
        var w = rect.w;
        const h = rect.h;
        var usedfont = self.fonts;

        if (text.len == 0) return 0;

        const render: bool = w != 0 and h != 0;

        if (render and (self.scheme == null or w == 0)) return 0;

        const state = struct {
            var ellipsis_width: ?u32 = null;
            var invalid_width: ?u32 = null;
            var nomatches: [128]usize = std.mem.zeroes([128]usize);
        };

        const invert_ = invert != 0; // just the boolean version of `invert`.

        var d: ?*X.XftDraw = null;
        if (!render) {
            // When NOT rendering, treat `invert` as a different kind of value
            // altogether.
            w = ~@as(c_uint, 0);
        } else {
            const color = if (invert_) &self.scheme.?.fg else &self.scheme.?.bg;
            X.XSetForeground(self.dpy, self.gc, color.pixel);
            X.XFillRectangle(self.dpy, self.drawable, self.gc, .{ .x = x, .y = y, .w = w, .h = h });
            if (w < lpad) {
                return x + @as(i32, @intCast(w));
            }
            d = X.XftDrawCreate(
                self.dpy,
                self.drawable,
                X.DefaultVisual(self.dpy, self.screen),
                X.DefaultColormap(self.dpy, self.screen),
            );
            x += @intCast(lpad);
            w -= lpad;
        }
        defer if (d) |draw| X.XftDrawDestroy(draw);

        if (state.ellipsis_width == null and render) {
            state.ellipsis_width = self.fontSetGetWidth(allocator, "...");
        }
        if (state.invalid_width == null and render) {
            state.invalid_width = self.fontSetGetWidth(allocator, INVALID);
        }

        const utf8 = struct {
            var codepoint: u21 = 0;
            var charlen: u3 = 0;
        };

        var nextfont: ?*Font = null;
        var utf8err: bool = undefined;
        var ellipsis_x: i32 = 0;
        var ellipsis_len: u32 = undefined;
        var ellipsis_w: u32 = 0;
        var overflow: bool = false;
        var utf8str: []const u8 = undefined;
        var ty: i32 = 0;
        var charexists = false;
        var match_opt: ?*X.FcPattern = null;
        var result: X.XftResult = undefined;
        // The number of bytes that the next UTF-8 char uses.
        var ew: u32 = undefined;
        var utf8strlen: u32 = undefined;

        // Main loop for printing text to completion. Breaks only when text runs
        // out or if there is overflow.
        while (true) {
            utf8err = false;
            ellipsis_len = 0;
            utf8strlen = 0;
            ew = 0;
            utf8str = text;
            utf8.codepoint = 0;
            utf8.charlen = 0;
            while (text.len > 0) {
                utf8.charlen = std.unicode.utf8ByteSequenceLength(text[0]) catch unreachable;
                utf8.codepoint = switch (utf8.charlen) {
                    1 => @intCast(text[0]),
                    2 => std.unicode.utf8Decode2(text[0..2].*) catch unreachable,
                    3 => std.unicode.utf8Decode3(text[0..3].*) catch unreachable,
                    4 => std.unicode.utf8Decode4(text[0..4].*) catch unreachable,
                    else => unreachable,
                };
                var curfont_opt: ?*Font = self.fonts;
                charexists = false;
                var tmpw: u32 = undefined;
                while (curfont_opt) |curfont| : (curfont_opt = curfont.next) {
                    charexists |= X.XftCharExists(self.dpy, curfont.xfont, @intCast(utf8.codepoint));
                    if (!charexists) {
                        continue;
                    }
                    curfont.getExtents(text[0..utf8.charlen], &tmpw, null);

                    if (ew + (state.ellipsis_width orelse 0) <= w) {
                        // keep track where the ellipsis still fits
                        ellipsis_x = x + @as(i32, @intCast(ew));
                        ellipsis_w = w - ew;
                        ellipsis_len = utf8strlen;
                    }

                    if (ew + tmpw > w) {
                        overflow = true;
                        // called from drw_fontset_getwidth_clamp():
                        // it wants the width AFTER the overflow
                        if (!render) {
                            x += @intCast(tmpw);
                        } else {
                            utf8strlen = ellipsis_len;
                        }
                    } else if (curfont == usedfont) {
                        text = text[utf8.charlen..];
                        utf8strlen += if (utf8err) 0 else utf8.charlen;
                        ew += if (utf8err) 0 else tmpw;
                    } else {
                        nextfont = curfont;
                    }
                    break;
                }

                if (overflow or !charexists or nextfont != null or utf8err) {
                    break;
                } else {
                    charexists = false;
                }
            }

            if (utf8strlen > 0) {
                if (render) {
                    ty = y + @divTrunc(@as(i32, @intCast(h - usedfont.height)), 2) + usedfont.xfont.ascent;
                    const color = if (invert_) &self.scheme.?.bg else &self.scheme.?.fg;
                    if (d) |drw| {
                        X.XftDrawStringUtf8(drw, color, usedfont.xfont, x, ty, utf8str, @intCast(utf8strlen));
                    }
                }
                x += @intCast(ew);
                w -= ew;
            }

            if (utf8err and (!render or (state.invalid_width orelse w) < w)) {
                if (render) {
                    _ = self.drawText(allocator, .{ .x = x, .y = y, .w = w, .h = h }, 0, INVALID, invert);
                }
                x += @intCast(state.invalid_width orelse 0);
                w -= state.invalid_width orelse 0;
            }

            if (render and overflow) {
                _ = self.drawText(allocator, .{ .x = ellipsis_x, .y = y, .w = ellipsis_w, .h = h }, 0, "...", invert);
            }

            if (text.len == 0 or overflow) {
                break;
            } else if (nextfont) |f| {
                charexists = false;
                usedfont = f;
            } else {
                // TODO: break all this out into a separate function and call it
                // something related to fallback.

                // Regardless of whether or not a fallback font is found, the
                // character must be drawn.
                charexists = true;

                var hash: usize = @intCast(utf8.codepoint);
                hash = ((hash >> 16) ^ hash) *% 0x21F0AAAD;
                hash = ((hash >> 15) ^ hash) *% 0xD35A2D97;
                const l = state.nomatches.len;
                const h0 = ((hash >> 15) ^ hash) % l;
                const h1 = (hash >> 17) % l;
                // avoid expensive XftFontMatch call when we know we won't find
                // a match
                if (state.nomatches[h0] == utf8.codepoint or state.nomatches[h1] == utf8.codepoint) {
                    usedfont = self.fonts;
                    continue;
                }

                const fccharset = X.FcCharSetCreate() orelse unreachable;
                _ = X.FcCharSetAddChar(fccharset, @intCast(utf8.codepoint));

                const self_fonts_pattern = self.fonts.pattern orelse {
                    // Refer to the comment in xfont_create for more information.
                    @panic("the first font in the cache must be loaded from a font string.");
                };

                const fcpattern = X.FcPatternDuplicate(self_fonts_pattern) orelse unreachable;
                _ = X.FcPatternAddCharSet(fcpattern, X.FC_CHARSET, fccharset);
                _ = X.FcPatternAddBool(fcpattern, X.FC_SCALABLE, true);

                _ = X.FcConfigSubstitute(null, fcpattern, .Pattern);
                X.FcDefaultSubstitute(fcpattern);
                match_opt = X.XftFontMatch(self.dpy, self.screen, fcpattern, &result);

                X.FcCharSetDestroy(fccharset);
                X.FcPatternDestroy(fcpattern);

                if (match_opt) |match| {
                    const j = if (state.nomatches[h0] > 0) h1 else h0;
                    usedfont = xfontCreate(allocator, self.dpy, self.screen, "", match) catch {
                        state.nomatches[j] = utf8.codepoint;
                        continue;
                    };
                    if (X.XftCharExists(self.dpy, usedfont.xfont, @intCast(utf8.codepoint))) {
                        var curfont: *Font = self.fonts;
                        while (curfont.next) |next| : (curfont = next) {}
                        curfont.next = usedfont;
                    } else {
                        state.nomatches[j] = utf8.codepoint;
                        xfontFree(allocator, usedfont);
                    }
                }
            }
        }
        return x + if (render) @as(i32, @intCast(w)) else 0;
    }

    /// (dwm) drw_fontset_getwidth
    pub fn fontSetGetWidth(self: *Self, allocator: Allocator, text: []const u8) u32 {
        if (text.len == 0) return 0;
        return @intCast(self.drawText(allocator, .zero, 0, text, 0));
    }

    /// (dwm) drw_map
    pub fn map(self: *Self, w: X.Window, r: Rect) void {
        X.XCopyArea(self.dpy, self.drawable, w, self.gc, r, r.toCoordinates());
        X.XSync(self.dpy, false);
    }
};
