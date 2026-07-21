const std = @import("std");
const Allocator = std.mem.Allocator;

fn gatherArgs(alloc: Allocator) !std.ArrayList([:0]const u8) {
    var vec: std.ArrayList([:0]const u8) = .empty;
    var argvIt = try std.process.argsWithAllocator(alloc);
    defer argvIt.deinit();
    _ = argvIt.skip(); // the name of *this* binary.
    while (argvIt.next()) |argv| try vec.append(alloc, argv);
    return vec;
}

fn fingerprint() u64 {
    const x: u64 = @intCast(@mod(std.time.microTimestamp(), 0x100000));
    return std.hash.int(x);
}

pub fn main() !void {
    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .init;
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    var argv = try gatherArgs(alloc);
    defer argv.deinit(alloc);

    for (argv.items) |arg| {
        std.debug.print("{s}\n", .{arg});
    }

    // Get a fingerprint for this run.
    var buf: [64]u8 = undefined;
    _ = std.fmt.printInt(&buf, fingerprint(), 16, .lower, .{});
    const fpr = buf[0..7];

    std.debug.print("{s}\n", .{fpr});

    var child = std.process.Child.init(argv.items, alloc);
    try child.spawn();

    const exitStatus = try child.wait();
    _ = exitStatus;
}
