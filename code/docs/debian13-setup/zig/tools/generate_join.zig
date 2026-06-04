const std = @import("std");

pub const N: u32 = 50;

pub fn main() !void {
    const cwd = std.fs.cwd();
    const f = try cwd.createFile("src/join.zig", .{});
    defer f.close();
    var write_buffer: [1024]u8 = undefined;
    var writer = f.writer(&write_buffer);
    var w = &writer.interface;
    try w.print("/// Brute-force join. (Up to {d} elements)\n", .{N});
    try w.print("pub fn join(comptime v: []const []const u8) []const u8 {{\n", .{});
    try w.print("return switch (v.len) {{", .{});
    try w.print("0 => \"\",", .{});
    {
        for (1..N + 1) |i| {
            try w.print("{d} => v[0]", .{i});
            for (1..i) |j| try w.print(" ++ \" \" ++ v[{d}]", .{j});
            try w.print(",", .{});
        }
    }
    try w.print("else => unreachable,", .{});
    try w.print("}};}}\n", .{});
    try w.flush();
}
