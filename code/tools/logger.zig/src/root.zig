const std = @import("std");

pub const LogOpts = struct {
    print_colors: bool,
};

pub fn init(comptime opts: LogOpts) type {
    return struct {
        pub fn logFn(
            comptime message_level: std.log.Level,
            comptime scope: @Type(.enum_literal),
            comptime format: []const u8,
            args: anytype,
        ) void {
            const color = switch (message_level) {
                .debug => "\x1b[36m",
                .info => "\x1b[32m",
                .warn => "\x1b[33m",
                .err => "\x1b[31m",
            };
            const level = txt: {
                const txt = comptime message_level.asText();
                break :txt if (opts.print_colors) color ++ txt ++ "\x1b[m" else txt;
            };
            const prefix = if (scope == .default) ": " else "(" ++ @tagName(scope) ++ "): ";
            var buffer: [64]u8 = undefined;
            const stderr = std.debug.lockStderrWriter(&buffer);
            defer std.debug.unlockStderrWriter();
            nosuspend stderr.print(level ++ prefix ++ format ++ "\n", args) catch return;
        }
    };
}
