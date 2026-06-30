const std = @import("std");
const log = std.log;
const unicode = std.unicode;
const X = @import("x11.zig");
const Rect = @import("rect.zig").Rect;
const EnumArray = @import("enum_array.zig").EnumArray;
const SchemeState = @import("color_scheme.zig").SchemeState;
const mem = std.mem;
const Allocator = mem.Allocator;
const Font = @import("font.zig").Font;
const ColorScheme = @import("color_scheme.zig").ColorScheme;
const Scheme = @import("color_scheme.zig").Scheme;

/// A simple hashset implementation specifically to store unicode codepoints
/// (and hence u21).
const SimpleHashSet = struct {
    const Self = @This();
    const N: usize = 128;

    data: [N]u21,

    const init: Self = .{ .data = std.mem.zeroes([N]u21) };

    fn hashedIndex(codepoint: u21) usize {
        var hash: usize = @intCast(codepoint);
        hash = ((hash >> 16) ^ hash) *% 0x21F0AAAD;
        hash = ((hash >> 15) ^ hash) *% 0xD35A2D97;
        return hash;
    }

    /// Note that this is approximate but good enough.
    pub fn contains(self: *Self, codepoint: u21) bool {
        const hash = hashedIndex(codepoint);
        const h0 = ((hash >> 15) ^ hash) % N;
        const h1 = (hash >> 17) % N;
        return self.data[h0] == codepoint or self.data[h1] == codepoint;
    }

    pub fn insert(self: *Self, codepoint: u21) void {
        const hash = hashedIndex(codepoint);
        const h0 = ((hash >> 15) ^ hash) % N;
        const h1 = (hash >> 17) % N;
        self.data[if (self.data[h0] > 0) h1 else h0] = codepoint;
    }
};

pub const DrwInitParams = struct {
    dpy: *X.Display,
    screen: c_int,
    /// Root Window.
    root: X.Window,
    width: c_uint,
    height: c_uint,
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

    pub fn init(p: DrwInitParams) error{ OutOfMemory, FontCreateError }!Self {
        const depth = X.DefaultDepth(p.dpy, p.screen);
        const drw: Self = .{
            .dpy = p.dpy,
            .screen = p.screen,
            .root = p.root,
            .drawable = X.XCreatePixmap(p.dpy, p.root, p.width, p.height, @intCast(depth)),
            .gc = X.XCreateGC(p.dpy, p.root, 0, undefined),
            .w = p.width,
            .h = p.height,
        };
        X.XSetLineAttributes(p.dpy, drw.gc, 1, .Solid, .Butt, .Miter);
        return drw;
    }

    /// (dwm) drw_free
    pub fn deinit(self: *Self) void {
        X.XFreePixmap(self.dpy, self.drawable);
        X.XFreeGC(self.dpy, self.gc);
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

    /// (dwm) drw_setscheme
    pub fn setScheme(self: *Self, scheme: *ColorScheme) void {
        self.scheme = scheme;
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
        fonts: *Font,
        rect: Rect,
        /// Left padding.
        lpad: u32,
        text_to_draw: []const u8,
        invert: u32,
    ) error{OutOfMemory}!i32 {
        const INVALID = "�";
        var text: []const u8 = text_to_draw;
        var x = rect.x;
        var w = rect.w;

        if (text.len == 0) return 0;

        const render: bool = w != 0 and rect.h != 0;

        if (render and (self.scheme == null or w == 0)) return 0;

        const state = struct {
            var ellipsis_width: ?u32 = null;
            var invalid_width: ?u32 = null;
            var nomatches: [128]usize = std.mem.zeroes([128]usize);
            var nomatch: SimpleHashSet = .init;
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
            X.XFillRectangle(self.dpy, self.drawable, self.gc, rect);
            if (w < lpad) {
                return x + @as(c_int, @intCast(w));
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
            state.ellipsis_width = try self.fontSetGetWidth(allocator, fonts, "...");
        }
        if (state.invalid_width == null and render) {
            state.invalid_width = try self.fontSetGetWidth(allocator, fonts, INVALID);
        }

        const utf8 = struct {
            var codepoint: u21 = 0;
            var charlen: u3 = 0;
            var str: []const u8 = undefined;
            /// Then number of UTF-8 characters in `str`.
            var strlen: u32 = 0;
        };

        var nextfont: ?*Font = null;
        var utf8err: bool = undefined;
        var ellipsis_x: i32 = 0;
        var ellipsis_len: u32 = undefined;
        var ellipsis_w: u32 = 0;
        var overflow: bool = false;
        var ty: i32 = 0;
        var charexists = false;
        var result: X.XftResult = undefined;
        // The number of bytes that the next UTF-8 char uses.
        var ew: u32 = undefined;
        var usedfont = fonts;

        // Main loop for printing text to completion. Breaks only when text runs
        // out or if there is overflow.
        while (true) {
            utf8err = false;
            ellipsis_len = 0;
            utf8.strlen = 0;
            ew = 0;
            utf8.str = text;
            utf8.codepoint = 0;
            utf8.charlen = 0;

            while (text.len > 0) {
                utf8.charlen = unicode.utf8ByteSequenceLength(text[0]) catch unreachable;
                utf8.codepoint = switch (utf8.charlen) {
                    1 => @intCast(text[0]),
                    2 => unicode.utf8Decode2(text[0..2].*) catch unreachable,
                    3 => unicode.utf8Decode3(text[0..3].*) catch unreachable,
                    4 => unicode.utf8Decode4(text[0..4].*) catch unreachable,
                    else => unreachable,
                };
                charexists = false;
                var tmpw: u32 = undefined;
                if (fonts.getFontThatHasChar(utf8.codepoint)) |font| {
                    charexists = true;
                    font.getExtents(text[0..utf8.charlen], &tmpw, null);

                    if (ew + (state.ellipsis_width orelse 0) <= w) {
                        // keep track where the ellipsis still fits
                        ellipsis_x = x + @as(i32, @intCast(ew));
                        ellipsis_w = w - ew;
                        ellipsis_len = utf8.strlen;
                    }

                    if (ew + tmpw > w) {
                        overflow = true;
                        // called from drw_fontset_getwidth_clamp():
                        // it wants the width AFTER the overflow
                        if (!render) {
                            x += @intCast(tmpw);
                        } else {
                            utf8.strlen = ellipsis_len;
                        }
                    } else if (font == usedfont) {
                        text = text[utf8.charlen..];
                        utf8.strlen += if (utf8err) 0 else utf8.charlen;
                        ew += if (utf8err) 0 else tmpw;
                    } else {
                        nextfont = font;
                    }
                }

                if (overflow or !charexists or nextfont != null or utf8err) {
                    break;
                } else charexists = false;
            }

            if (utf8.strlen > 0) {
                if (render) {
                    ty = rect.y + @divTrunc(@as(c_int, @intCast(rect.h)) - usedfont.height, 2) + usedfont.xfont.ascent;
                    const color = if (invert_) &self.scheme.?.bg else &self.scheme.?.fg;
                    if (d) |drw| {
                        X.XftDrawStringUtf8(drw, color, usedfont.xfont, x, ty, utf8.str, @intCast(utf8.strlen));
                    }
                }
                x += @intCast(ew);
                w -= ew;
            }

            if (utf8err and (!render or (state.invalid_width orelse w) < w)) {
                if (render) {
                    _ = try self.drawText(allocator, fonts, .{ .x = x, .y = rect.y, .w = w, .h = rect.h }, 0, INVALID, invert);
                }
                x += @intCast(state.invalid_width orelse 0);
                w -= state.invalid_width orelse 0;
            }

            if (render and overflow) {
                _ = try self.drawText(allocator, fonts, .{ .x = ellipsis_x, .y = rect.y, .w = ellipsis_w, .h = rect.h }, 0, "...", invert);
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

                // avoid expensive XftFontMatch call when we know we won't find
                // a match
                if (state.nomatch.contains(utf8.codepoint)) {
                    usedfont = fonts;
                    continue;
                }

                const fccharset = X.FcCharSetCreate() orelse unreachable;
                _ = X.FcCharSetAddChar(fccharset, @intCast(utf8.codepoint));

                const self_fonts_pattern = fonts.xfont.pattern orelse {
                    // Refer to the comment in Font.fromName for more information.
                    @panic("the first font in the cache must be loaded from a font string.");
                };

                const fcpattern = X.FcPatternDuplicate(self_fonts_pattern) orelse unreachable;
                _ = X.FcPatternAddCharSet(fcpattern, X.FC_CHARSET, fccharset);
                _ = X.FcPatternAddBool(fcpattern, X.FC_SCALABLE, true);

                _ = X.FcConfigSubstitute(null, fcpattern, .Pattern);
                X.FcDefaultSubstitute(fcpattern);

                defer X.FcCharSetDestroy(fccharset);
                defer X.FcPatternDestroy(fcpattern);

                const fontMatch = X.XftFontMatch(self.dpy, self.screen, fcpattern, &result) orelse continue;

                usedfont = Font.initFromPattern(allocator, self.dpy, fontMatch) catch |err| switch (err) {
                    error.OutOfMemory => return error.OutOfMemory,
                    error.FontCreateError => {
                        state.nomatch.insert(utf8.codepoint);
                        continue;
                    },
                };
                if (usedfont.xftCharExists(self.dpy, utf8.codepoint)) {
                    fonts.pushBack(usedfont);
                } else {
                    state.nomatch.insert(utf8.codepoint);
                    usedfont.deinit(allocator);
                }
            }
        }
        return x + if (render) @as(i32, @intCast(w)) else 0;
    }

    /// (dwm) drw_fontset_getwidth
    pub fn fontSetGetWidth(
        self: *Self,
        allocator: Allocator,
        fonts: *Font,
        text: []const u8,
    ) error{OutOfMemory}!u32 {
        if (text.len == 0) return 0;
        return @intCast(try self.drawText(allocator, fonts, .zero, 0, text, 0));
    }

    /// (dwm) drw_map
    pub fn map(self: *Self, w: X.Window, r: Rect) void {
        X.XCopyArea(self.dpy, self.drawable, w, self.gc, r, r.toCoordinates());
        X.XSync(self.dpy, false);
    }
};
