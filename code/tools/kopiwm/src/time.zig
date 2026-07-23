const std = @import("std");

/// C standard library.
const C = @cImport({
    @cInclude("time.h");
    @cInclude("stdlib.h");
});

pub fn strftime(buffer: []u8, comptime format: []const u8, comptime timezone: ?[]const u8) []const u8 {
    const t = C.time(null);
    const n = C.strftime(buffer.ptr, buffer.len, format.ptr, C.localtime(&t));
    if (timezone) |tz| {
        _ = C.setenv("TZ", tz.ptr, 1);
        C.tzset();
    }
    return buffer[0..n];
}

// test {
//     var buf: [20]u8 = undefined;
//     const date = strftime(&buf, "%Y-%m-%d");
//     try std.testing.expectEqualStrings("2026-07-23", date);
// }
