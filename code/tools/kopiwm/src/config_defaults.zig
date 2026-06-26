const std = @import("std");
const k = @import("x11.zig").keys;
const m = @import("x11.zig").masks;
const Layout = @import("layout.zig").Layout;
const SchemeState = @import("enums.zig").SchemeState;
const Scheme = @import("drw.zig").Scheme;
const EnumArray = @import("enum_array.zig").EnumArray;
const Key = @import("enums.zig").Key;
const Button = @import("enums.zig").Button;
const M = @import("main.zig");
const cfg = @import("config.zig");

pub const layouts = [_]Layout{
    .{ .symbol = "[]=", .arrange = M.tile },
    .empty,
    .{ .symbol = "[M]", .arrange = M.monocle },
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
    .init(MODKEY,            k.XK_p,      .f(M.spawn,          .{ .args = launchcmd  })),
    .init(MODKEY|ShiftMask,  k.XK_Return, .f(M.spawn,          .{ .args = termcmd    })),
    .init(MODKEY,            k.XK_b,      .a(M.toggleBar,      undefined              )),
    .init(MODKEY,            k.XK_j,      .a(M.focusStack,     .{ .d = .Next         })),
    .init(MODKEY,            k.XK_k,      .a(M.focusStack,     .{ .d = .Prev         })),
    .init(MODKEY,            k.XK_i,      .a(M.incNMaster,     .{ .i =  1            })),
    .init(MODKEY,            k.XK_d,      .a(M.incNMaster,     .{ .i = -1            })),
    .init(MODKEY,            k.XK_h,      .a(M.setMFact,       .{ .f =  0.05         })),
    .init(MODKEY,            k.XK_l,      .a(M.incNMaster,     .{ .f = -0.05         })),
    .init(MODKEY,            k.XK_Return, .a(M.zoom,           undefined              )),
    .init(MODKEY,            k.XK_Tab,    .a(M.view,           undefined              )),
    .init(MODKEY|ShiftMask,  k.XK_c,      .f(M.killClient,     undefined              )),
    .init(MODKEY,            k.XK_t,      .a(M.setLayout,      .{ .l = &layouts[0]   })),
    .init(MODKEY,            k.XK_f,      .a(M.setLayout,      .{ .l = &layouts[1]   })),
    .init(MODKEY,            k.XK_m,      .a(M.setLayout,      .{ .l = &layouts[2]   })),
    .init(MODKEY,            k.XK_space,  .a(M.setLayout,      .{ .l = &.empty       })),
    .init(MODKEY|ShiftMask,  k.XK_space,  .a(M.toggleFloating, undefined              )),
    .init(MODKEY,            k.XK_0,      .a(M.view,           .{ .ui = ~@as(u32, 0) })),
    .init(MODKEY|ShiftMask,  k.XK_0,      .a(M.tag,            .{ .ui = ~@as(u32, 0) })),
    .init(MODKEY,            k.XK_comma,  .a(M.focusMon,       .{ .d = .Prev         })),
    .init(MODKEY,            k.XK_period, .a(M.focusMon,       .{ .d = .Next         })),
    .init(MODKEY|ShiftMask,  k.XK_comma,  .a(M.tagMonitor,     .{ .d = .Prev         })),
    .init(MODKEY|ShiftMask,  k.XK_period, .a(M.tagMonitor,     .{ .d = .Next         })),
    .init(MODKEY|ShiftMask,  k.XK_q,      .f(M.quit,           undefined              )),
};
// zig fmt: on

/// A template of what's to be mapped for each tag available.
// zig fmt: off
pub const tag_keys = [_]Key{
    .init(MODKEY,                       0, .a(M.view,       .{ .ui = 0 })),
    .init(MODKEY|ControlMask,           0, .a(M.toggleView, .{ .ui = 0 })),
    .init(MODKEY|ShiftMask,             0, .a(M.tag,        .{ .ui = 0 })),
    .init(MODKEY|ShiftMask|ControlMask, 0, .a(M.toggleTag,  .{ .ui = 0 })),
};
// zig fmt: on

pub const keys = cfg.initKeys(&base_keys);

// zig fmt: off
pub const buttons = [_]Button{
.init(.LtSymbol,     0,        k.Button1,   .a( M.setLayout,        .{ .l = &.empty     } )),
.init(.LtSymbol,     0,        k.Button3,   .a( M.setLayout,        .{ .l = &layouts[2] } )),
.init(.WinTitle,     0,        k.Button2,   .a( M.zoom,             undefined             )),
.init(.StatusText,   0,        k.Button2,   .f( M.spawn,            .{.args = &.{}}       )),
.init(.ClientWin,    MODKEY,   k.Button1,   .A( M.moveMouse,        undefined             )),
.init(.ClientWin,    MODKEY,   k.Button2,   .a( M.toggleFloating,   undefined             )),
.init(.ClientWin,    MODKEY,   k.Button3,   .A( M.resizeMouse,      undefined             )),
.init(.TagBar,       0,        k.Button1,   .a( M.view,             undefined             )),
.init(.TagBar,       0,        k.Button3,   .a( M.toggleView,       undefined             )),
.init(.TagBar,       MODKEY,   k.Button1,   .a( M.tag,              undefined             )),
.init(.TagBar,       MODKEY,   k.Button3,   .a( M.toggleTag,        undefined             )),
};
// zig fmt: on
