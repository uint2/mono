const std = @import("std");
const log = @import("std").log;

const X_TUTORIAL_SOURCE: []const u8 = @embedFile("x11.zig");

test "All inline functions for docs" {
    var iter = std.mem.splitScalar(u8, X_TUTORIAL_SOURCE, '\n');
    while (iter.next()) |line| {
        // We don't want any non-inlined functions because they really are just
        // macros of X11 library functions that contain docs.
        const contains_pub_fn = std.mem.containsAtLeast(u8, line, 1, "pub fn");
        try std.testing.expect(!contains_pub_fn);
    }
}

test "All inline functions have sources" {
    var iter = std.mem.splitScalar(u8, X_TUTORIAL_SOURCE, '\n');
    const n = 2;
    var prev: [n]?[]const u8 = undefined;
    while (iter.next()) |line| {
        if (std.mem.startsWith(u8, line, "pub inline fn")) {
            log.info("line: {s}", .{line});
            try std.testing.expectStringStartsWith(prev[1].?, "///");
            try std.testing.expectStringStartsWith(prev[0].?, "/// source: https://");
        }
        @memmove(prev[1..n], prev[0 .. n - 1]);
        prev[0] = line;
    }
}

// test "All struct docs have sources" {
//     var iter = std.mem.splitScalar(u8, X_TUTORIAL_SOURCE, '\n');
//     const n = 2;
//     var prev: [n]?[]const u8 = undefined;
//     while (iter.next()) |line| {
//         const select = std.mem.startsWith(u8, line, "pub const X") and
//             std.mem.containsAtLeast(u8, line, 1, "= X.");
//         if (select) {
//             log.info("line: {s}", .{line});
//             try std.testing.expectStringStartsWith(prev[1].?, "///");
//             try std.testing.expectStringStartsWith(prev[0].?, "/// source: https://x.org/");
//         }
//         @memmove(prev[1..n], prev[0 .. n - 1]);
//         prev[0] = line;
//     }
// }

test "Don't use weird quotes" {
    const q = @import("quotes.zig");
    try std.testing.expect(!std.mem.containsAtLeast(u8, X_TUTORIAL_SOURCE, 1, q.Q_APOSTROPHE));
    try std.testing.expect(!std.mem.containsAtLeast(u8, X_TUTORIAL_SOURCE, 1, q.Q_RIGHT_SINGLE_QUOTATION_MARK));
    try std.testing.expect(!std.mem.containsAtLeast(u8, X_TUTORIAL_SOURCE, 1, q.Q_LEFT_SINGLE_QUOTATION_MARK));
    try std.testing.expect(!std.mem.containsAtLeast(u8, X_TUTORIAL_SOURCE, 1, q.Q_RIGHT_DOUBLE_QUOTATION_MARK));
    try std.testing.expect(!std.mem.containsAtLeast(u8, X_TUTORIAL_SOURCE, 1, q.Q_LEFT_DOUBLE_QUOTATION_MARK));
}
