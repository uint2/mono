const X = @import("x11.zig");

/// Represents a possible which one might be in that warrants a unique color scheme.
pub const SchemeState = enum {
    Normal,
    Selected,
    Bar,
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
