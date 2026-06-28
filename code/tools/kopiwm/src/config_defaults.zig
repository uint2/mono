const std = @import("std");
const k = @import("x11.zig").keys;
const m = @import("x11.zig").masks;
const Layout = @import("layout.zig").Layout;
const SchemeState = @import("color_scheme.zig").SchemeState;
const Scheme = @import("drw.zig").Scheme;
const EnumArray = @import("enum_array.zig").EnumArray;
const Key = @import("enums.zig").Key;
const Button = @import("enums.zig").Button;
const M = @import("main.zig");
const cfg = @import("config.zig");

pub const layouts = [_]Layout{
    .{ .symbol = "[]=", .arrange = M.layouts.tile },
    .empty,
    .{ .symbol = "[M]", .arrange = M.layouts.monocle },
};

const col_gray1: []const u8 = "#222222";
const col_gray2: []const u8 = "#444444";
const col_gray3: []const u8 = "#bbbbbb";
const col_gray4: []const u8 = "#eeeeee";
const col_accent_400: []const u8 = "#d8b4fe";
const col_accent_900: []const u8 = "#581c87";

fn initColors() EnumArray(SchemeState, Scheme([]const u8)) {
    var c: EnumArray(SchemeState, Scheme([]const u8)) = undefined;
    // zig fmt: off
    c.set(.Normal,   .{ .fg = col_gray3, .bg = col_gray1,      .border = col_gray2      });
    c.set(.Selected, .{ .fg = col_gray1, .bg = col_accent_400, .border = col_accent_900 });
    c.set(.Bar,      .{ .fg = col_gray3, .bg = col_gray2,      .border = col_gray2      });
    // zig fmt: on
    return c;
}

pub const colors = initColors();

const ShiftMask = m.ShiftMask;
const ControlMask = m.ControlMask;

const MODKEY = m.Mod4Mask;
const launchcmd: [*:null]const ?[*:0]const u8 = &.{ "rofi", "-show", "run", "-matching", "fuzzy", "-sort", "-sorting-method", "fzf" };
const termcmd: [*:null]const ?[*:0]const u8 = &.{"xterm"};

// zig fmt: off
pub const base_keys = [_]Key{
    .init(MODKEY,            k.XK_p,      .f(M.mp.spawn,          .{ .args = launchcmd  })),
    .init(MODKEY|ShiftMask,  k.XK_Return, .f(M.mp.spawn,          .{ .args = termcmd    })),
    .init(MODKEY,            k.XK_b,      .a(M.mp.toggleBar,      undefined              )),
    .init(MODKEY,            k.XK_j,      .a(M.mp.focusStack,     .{ .d = .Next         })),
    .init(MODKEY,            k.XK_k,      .a(M.mp.focusStack,     .{ .d = .Prev         })),
    .init(MODKEY,            k.XK_i,      .a(M.mp.incNMaster,     .{ .i =  1            })),
    .init(MODKEY,            k.XK_d,      .a(M.mp.incNMaster,     .{ .i = -1            })),
    .init(MODKEY,            k.XK_h,      .a(M.mp.setMFact,       .{ .f =  0.05         })),
    .init(MODKEY,            k.XK_l,      .a(M.mp.incNMaster,     .{ .f = -0.05         })),
    .init(MODKEY,            k.XK_Return, .a(M.mp.zoom,           undefined              )),
    .init(MODKEY,            k.XK_Tab,    .a(M.mp.view,           undefined              )),
    .init(MODKEY|ShiftMask,  k.XK_c,      .f(M.mp.killClient,     undefined              )),
    .init(MODKEY,            k.XK_t,      .a(M.mp.setLayout,      .{ .l = &layouts[0]   })),
    .init(MODKEY,            k.XK_f,      .a(M.mp.setLayout,      .{ .l = &layouts[1]   })),
    .init(MODKEY,            k.XK_m,      .a(M.mp.setLayout,      .{ .l = &layouts[2]   })),
    .init(MODKEY,            k.XK_space,  .a(M.mp.setLayout,      .{ .l = &.empty       })),
    .init(MODKEY|ShiftMask,  k.XK_space,  .a(M.mp.toggleFloating, undefined              )),
    .init(MODKEY,            k.XK_0,      .a(M.mp.view,           .{ .ui = ~@as(u32, 0) })),
    .init(MODKEY|ShiftMask,  k.XK_0,      .a(M.mp.tag,            .{ .ui = ~@as(u32, 0) })),
    .init(MODKEY,            k.XK_comma,  .a(M.mp.focusMon,       .{ .d = .Prev         })),
    .init(MODKEY,            k.XK_period, .a(M.mp.focusMon,       .{ .d = .Next         })),
    .init(MODKEY|ShiftMask,  k.XK_comma,  .a(M.mp.tagMonitor,     .{ .d = .Prev         })),
    .init(MODKEY|ShiftMask,  k.XK_period, .a(M.mp.tagMonitor,     .{ .d = .Next         })),
    .init(MODKEY|ShiftMask,  k.XK_q,      .f(M.mp.quit,           undefined              )),
};
// zig fmt: on

/// A template of what's to be mapped for each tag available.
// zig fmt: off
pub const tag_keys = [_]Key{
    .init(MODKEY,                       0, .a(M.mp.view,       .{ .ui = 0 })),
    .init(MODKEY|ControlMask,           0, .a(M.mp.toggleView, .{ .ui = 0 })),
    .init(MODKEY|ShiftMask,             0, .a(M.mp.tag,        .{ .ui = 0 })),
    .init(MODKEY|ShiftMask|ControlMask, 0, .a(M.mp.toggleTag,  .{ .ui = 0 })),
};
// zig fmt: on

pub const keys = cfg.initKeys(&base_keys);

// zig fmt: off
pub const buttons = [_]Button{
.init(.LtSymbol,     0,        k.Button1,   .a( M.mp.setLayout,        .{ .l = &.empty     } )),
.init(.LtSymbol,     0,        k.Button3,   .a( M.mp.setLayout,        .{ .l = &layouts[2] } )),
.init(.WinTitle,     0,        k.Button2,   .a( M.mp.zoom,             undefined             )),
.init(.StatusText,   0,        k.Button2,   .f( M.mp.spawn,            .{.args = &.{}}       )),
.init(.ClientWin,    MODKEY,   k.Button1,   .A( M.mp.moveMouse,        undefined             )),
.init(.ClientWin,    MODKEY,   k.Button2,   .a( M.mp.toggleFloating,   undefined             )),
.init(.ClientWin,    MODKEY,   k.Button3,   .A( M.mp.resizeMouse,      undefined             )),
.init(.TagBar,       0,        k.Button1,   .a( M.mp.view,             undefined             )),
.init(.TagBar,       0,        k.Button3,   .a( M.mp.toggleView,       undefined             )),
.init(.TagBar,       MODKEY,   k.Button1,   .a( M.mp.tag,              undefined             )),
.init(.TagBar,       MODKEY,   k.Button3,   .a( M.mp.toggleTag,        undefined             )),
};
// zig fmt: on
