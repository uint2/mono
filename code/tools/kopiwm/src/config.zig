const std = @import("std");
const X = @import("x11.zig");
const k = @import("x11.zig").keys;
const lt = @import("layout.zig");
const Layout = @import("layout.zig").Layout;
const SchemeState = @import("color_scheme.zig").SchemeState;
const N = @import("enums.zig").N;
const Scheme = @import("color_scheme.zig").Scheme;
const EnumArray = @import("enum_array.zig").EnumArray;
const Arg = @import("lazy_fn.zig").Arg;
const BarPosition = @import("enums.zig").BarPosition;
const Key = @import("enums.zig").Key;
const Button = @import("enums.zig").Button;
const Rule = @import("enums.zig").Rule;
const M = @import("main.zig");
const EM = @import("x11.zig").eventMask;
const defaults = @import("config_defaults.zig");

// False means use custom config. Defaults are kept because Zig does very
// aggressive tree-shaking, so lots of code is just not checked at comptime if
// a minimal config is used, so we keep it as a switch if ever we want to debug
// those functions.
const USE_DEFAULT_CONFIG = false;

pub const BUTTONMASK = EM.ButtonPressMask | EM.ButtonReleaseMask;
pub const MOUSEMASK = BUTTONMASK | EM.PointerMotionMask;

/// Number of pixels to snap during movement.
pub const snap: i32 = 32;

/// border pixel of windows
pub const borderpx: u32 = 1;

pub const Tag = struct {
    text: []const u8,
    key: X.KeySym,
};

/// The tags. These will determine which clients are visible on the screen.
/// The mask corresponding to the first element of this array would be the one
/// with only the least significant bit set.
/// ```
/// 0b00001 <=> tags[0]
/// 0b00010 <=> tags[1]
/// 0b00100 <=> tags[2]
/// ```
pub const tags = [_]Tag{
    .{ .text = "1", .key = k.XK_1 },
    .{ .text = "2", .key = k.XK_2 },
    .{ .text = "3", .key = k.XK_3 },
    .{ .text = "4", .key = k.XK_4 },
    .{ .text = "T", .key = k.XK_0 },
};

// Amazingly, Zig throws a COMPILE ERROR if `tags.len` is >= 32. This is because
// the maximum meaningful left-shift is by 31 for a u32 type, and so Zig
// takes a u5 as the left-shift amount. Which means that `tags.len` will first
// be casted to a u5 and panics with "type 'u5' cannot represent integer ..." if
// it's too large. At which point, either don't use that many tags, or change
// the tag mask to use more bits.
pub const TAGMASK: u32 = (@as(u32, 1) << tags.len) - 1;

/// Gets the mask corresponding to a tag. It is on the user to guarantee that the index is
/// within bounds.
pub inline fn tagMask(tag_index: usize) u32 {
    return @as(u32, 1) << @intCast(tag_index);
}

pub const fonts = [_][]const u8{"sans:size=10.5"};

/// Factor of the master area size [0.05...0.95].
pub const mfact: f32 = 0.5;

/// Number of clients in master area
pub const nmaster = 1;

/// Respect size hints in tiled resizals
pub const resizehints: bool = true;

/// Force focus on the fullscreen window
pub const lockfullscreen: bool = true;

/// Refresh rate (per second) for client move/resize
pub const refreshrate: u16 = 60;

/// False means hide bar.
pub const show_bar: bool = true;

pub const bar_pos: BarPosition = .top;

pub const bar_height: u32 = 20;

pub const layouts = [_]Layout{
    .{ .symbol = "[]=", .arrange = M.tile },
    .{ .symbol = "[M]", .arrange = M.monocle },
};

fn initColors() EnumArray(SchemeState, Scheme([]const u8)) {
    const gray1: []const u8 = "#222222";
    const gray2: []const u8 = "#444444";
    const gray3: []const u8 = "#bbbbbb";
    const accent_400: []const u8 = "#d8b4fe";

    var c: EnumArray(SchemeState, Scheme([]const u8)) = undefined;
    // zig fmt: off
    c.set(.Normal,   .{ .fg = gray3, .bg = gray1,      .border = gray2      });
    c.set(.Selected, .{ .fg = gray1, .bg = accent_400, .border = accent_400 });
    c.set(.Bar,      .{ .fg = gray3, .bg = gray2,      .border = gray2      });
    // zig fmt: on
    return c;
}
pub const colors = initColors();

const m = struct {
    pub const alt = X.masks.Mod1Mask;
    pub const ctrl = X.masks.ControlMask;
    pub const shft = X.masks.ShiftMask;
    pub const gui = X.masks.Mod4Mask;

    pub const hyper = alt | ctrl | shft | gui;
    /// The 3 keys on the bottom row. Ctrl + Alt + GUI.
    pub const bot3 = alt | ctrl | gui;
};

const MODKEY = m.gui;
const launchcmd: [*:null]const ?[*:0]const u8 = &.{ "rofi", "-show", "run", "-matching", "fuzzy", "-sort", "-sorting-method", "fzf" };
const termcmd: [*:null]const ?[*:0]const u8 = &.{"kitty"};

// zig fmt: off
const my_keys = [_]Key{
    // will there still be a null there thanks to Zig?
    .init(MODKEY,                   k.XK_space,  .f(M.spawn,               .{ .args = launchcmd  })),
    .init(MODKEY,                   k.XK_Return, .f(M.spawn,               .{ .args = termcmd    })),
    // .init(MODKEY,                   k.XK_j,      .f(M.focusStack,          .{ .d = .Next         })),
    // .init(MODKEY,                   k.XK_k,      .f(M.focusStack,          .{ .d = .Prev         })),
    .init(MODKEY|m.ctrl|m.shft,     k.XK_equal,  .a(M.setMFact,            .{ .f =  0.04         })),
    .init(MODKEY|m.ctrl|m.shft,     k.XK_minus,  .a(M.setMFact,            .{ .f = -0.04         })),
    .init(MODKEY,                   k.XK_Return, .a(M.zoom,                undefined              )),
    .init(MODKEY,                   k.XK_Tab,    .a(M.focusStack,          .{ .d = .Next         })),
    .init(MODKEY,                   k.XK_q,      .f(M.killClient,          undefined              )),
    .init(MODKEY|m.ctrl,            k.XK_f,      .a(M.toggleFloating,      undefined              )),
    .init(m.hyper,                  k.XK_q,      .f(M.quit,                undefined              )),
    .init(m.bot3,                   k.XK_b,      .a(M.toggleBar,           undefined              )),
    .init(m.hyper,                  k.XK_b,      .a(M.toggleBarPosition,   undefined              )),
};
// zig fmt: on

/// A template of what's to be mapped for each tag available.
// zig fmt: off
const my_tag_keys = [_]Key{
    .init(MODKEY,           0, .a(M.view,       .{ .ui = 0 })),
    .init(MODKEY|m.shft,    0, .a(M.tag,        .{ .ui = 0 })),
};
// zig fmt: on

pub fn initKeys(
    base: []const Key,
    tag_keys: []const Key,
) [base.len + tag_keys.len * tags.len]Key {
    const K = base.len + tag_keys.len * tags.len;
    var arr: [K]Key = undefined;
    @memcpy(arr[0..base.len], base);
    const T = tag_keys.len;
    for (0..tags.len) |i| {
        const j = base.len + i * tag_keys.len;
        const tag_mask = @as(u32, 1) << @intCast(i);
        @memcpy(arr[j .. j + T], tag_keys);
        for (j..j + T) |l| {
            arr[l].sym = tags[i].key;
            arr[l].lf.arg = .{ .ui = tag_mask };
        }
    }
    return arr;
}
pub const keys = initKeys(
    if (USE_DEFAULT_CONFIG) &defaults.base_keys else &my_keys,
    if (USE_DEFAULT_CONFIG) &defaults.tag_keys else &my_tag_keys,
);

// zig fmt: off
pub const my_buttons = [_]Button{
.init(.ClientWin,    MODKEY,   k.Button1,   .A( M.moveMouse,        undefined)),
.init(.ClientWin,    MODKEY,   k.Button3,   .A( M.resizeMouse,      undefined)),
.init(.TagBar,       0,        k.Button1,   .a( M.view,             undefined)),
.init(.TagBar,       0,        k.Button3,   .a( M.toggleView,       undefined)),
};
// zig fmt: on
pub const buttons: []const Button = if (USE_DEFAULT_CONFIG) &defaults.buttons else &my_buttons;

// zig fmt: off
pub const rules = [_]Rule{
    Rule{ .class = "firefox",  .tags = tagMask(1), .is_floating = false },
    Rule{ .class = "discord",  .tags = tagMask(2), .is_floating = false },
    Rule{ .class = "Telegram", .tags = tagMask(2), .is_floating = false },
};
// zig fmt: on
