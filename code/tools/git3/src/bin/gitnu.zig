const std = @import("std");
const log = std.log;

const git3 = @import("git3");

const logger = @import("monologue").init(.{ .print_colors = true });

pub const std_options: std.Options = .{
    .log_level = .debug,
    .logFn = logger.logFn,
};

pub fn main() !void {
    var cwdBuf: [1024]u8 = undefined;
    const cwd = std.fs.cwd().realpath(".", &cwdBuf) catch |err| {
        return std.debug.print("Failed to get current working directory.\n{any}", .{err});
    };
    log.info("cwd: {s}", .{cwd});
    // var t = try std.Thread.spawn(.{}, yes, .{});
    // t.join();
}

fn yes() void {
    while (true) {
        std.Thread.sleep(std.time.ns_per_s * 1);
        log.info("HEYY", .{});
    }
}
