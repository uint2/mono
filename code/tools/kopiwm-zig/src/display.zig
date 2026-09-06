const std = @import("std");

const X = @import("x11.zig");
const NAME = @import("build_opts").name;

pub var c: *X.Display = undefined;

pub fn init() bool {
    c = X.XOpenDisplay(null) orelse {
        return false;
    };
    return true;
}
