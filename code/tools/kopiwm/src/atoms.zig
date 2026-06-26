//! This file contains X Atoms as enums.

// A good place to start reading is
// https://x.org/releases/X11R7.6/doc/xorg-docs/specs/ICCCM/icccm.html

const X = @import("x11.zig");
const EnumArray = @import("enum_array.zig").EnumArray;
const std = @import("std");

pub fn initializeAtomsForEnum(
    dpy: *X.Display,
    comptime Key: type,
    array: *EnumArray(Key, X.Atom),
) void {
    for (std.enums.values(Key)) |key| {
        array.set(key, X.XInternAtom(dpy, key.asStr(), false).?);
    }
}

/// (dwm) WM* atoms.
pub const WM = enum {
    const Self = @This();

    Delete,
    Protocols,
    State,
    TakeFocus,

    pub fn asStr(self: *const Self) [*c]const u8 {
        return switch (self.*) {
            .Delete => "WM_DELETE_WINDOW",
            .Protocols => "WM_PROTOCOLS",
            .State => "WM_STATE",
            .TakeFocus => "WM_TAKE_FOCUS",
        };
    }
};

/// (dwm) Net* atoms.
///
/// See
///
///   https://specifications.freedesktop.org/wm/1.5/
///
/// For more details.
pub const Net = enum {
    const Self = @This();

    ActiveWindow,
    ClientList,
    Supported,
    /// The Window Manager MUST set this property on the root window to be the
    /// ID of a child window created by himself, to indicate that a compliant
    /// window manager is active. The child window MUST also have the
    /// _NET_SUPPORTING_WM_CHECK property set to the ID of the child window.
    /// The child window MUST also have the _NET_WM_NAME property set to the
    /// name of the Window Manager.
    ///
    /// Rationale: The child window is used to distinguish an active Window
    /// Manager from a stale _NET_SUPPORTING_WM_CHECK property that happens to
    /// point to another window. If the _NET_SUPPORTING_WM_CHECK window on the
    /// client window is missing or not properly set, clients SHOULD assume
    /// that no conforming Window Manager is present.
    ///
    /// source: https://specifications.freedesktop.org/wm/1.5/
    ///
    /// Look for "_NET_SUPPORTING_WM_CHECK" in the subpages (for the subpage,
    /// look for "Root Window Properties (and Related Messages)").
    WMCheck,
    WMFullscreen,
    WMName,
    WMState,
    WMWindowType,
    WMWindowTypeDialog,

    pub fn asStr(self: *const Self) [*c]const u8 {
        return switch (self.*) {
            .ActiveWindow => "_NET_ACTIVE_WINDOW",
            .ClientList => "_NET_CLIENT_LIST",
            .Supported => "_NET_SUPPORTED",
            .WMCheck => "_NET_SUPPORTING_WM_CHECK",
            .WMFullscreen => "_NET_WM_STATE_FULLSCREEN",
            .WMName => "_NET_WM_NAME",
            .WMState => "_NET_WM_STATE",
            .WMWindowType => "_NET_WM_WINDOW_TYPE",
            .WMWindowTypeDialog => "_NET_WM_WINDOW_TYPE_DIALOG",
        };
    }
};

pub var __WM: EnumArray(WM, X.Atom) = .empty;
pub var __NET: EnumArray(Net, X.Atom) = .empty;

pub inline fn net(key: Net) X.Atom {
    return __NET.get(key);
}

pub inline fn wm(key: WM) X.Atom {
    return __WM.get(key);
}
