const std = @import("std");
const Allocator = std.mem.Allocator;
const datetime = @import("datetime");

fn gatherArgs(alloc: Allocator) !std.ArrayList([:0]const u8) {
    var vec: std.ArrayList([:0]const u8) = .empty;

    var x: std.process.Args = .{ .vector = undefined };
    var it = try x.iterateAllocator(alloc);
    defer it.deinit();
    while (it.next()) |arg| {
        try vec.append(alloc, arg);
    }
    // std.process.arg
    // defer argvIt.deinit();
    // _ = argvIt.skip(); // the name of *this* binary.
    // while (argvIt.next()) |argv| try vec.append(alloc, argv);
    return vec;
}

fn fingerprint(init: std.process.Init) u64 {
    const t = std.Io.Clock.now(.real, init.io);
    const x: u64 = @intCast(@mod(t.toMicroseconds(), 0x100000));
    return std.hash.int(x);
}

pub fn main(init: std.process.Init) !void {
    var gpa: std.heap.DebugAllocator(.{}) = .init;
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    const now = datetime.datetime.Datetime.now();
    const nowStr = try now.formatISO8601(alloc, false);
    defer alloc.free(nowStr);
    const date = nowStr[0..10];

    std.debug.print("{s}.\n", .{date});

    var argv = try gatherArgs(alloc);
    defer argv.deinit(alloc);

    for (argv.items) |arg| {
        std.debug.print("{s}\n", .{arg});
    }

    // Get a fingerprint for this run.
    var buf: [64]u8 = undefined;
    _ = std.fmt.printInt(&buf, fingerprint(init), 16, .lower, .{});
    const fpr = buf[0..7];

    std.debug.print("{s}\n", .{fpr});
    //
    // var child = std.process.Child.init(argv.items, alloc);
    // try child.spawn();
    //
    // const exitStatus = try child.wait();
    // _ = exitStatus;
}
