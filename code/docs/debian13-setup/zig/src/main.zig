//! Khang's personal Debian 13 bootstrapper.
//!
//! Assumptions:
//! 1. `sudo` is installed.
//! 2. We have a POSIX system (no doy).
//!    a. Path separators are '/'.

const std = @import("std");
const Allocator = std.mem.Allocator;
const Reader = std.Io.Reader;
const log = std.log;

const join = @import("join.zig").join;

/// Join paths using the '/' path separator, which is a safe assumption to make.
fn path_join(comptime a: []const u8, comptime b: []const u8) []const u8 {
    var v = a;
    while (v.len > 0) {
        v = if (v[v.len - 1] == '/') v[0 .. v.len - 1] else return v ++ "/" ++ b;
    }
    return b;
}

pub const std_options: std.Options = .{
    .log_level = .debug,
    .logFn = @import("logger.zig").customLog,
};

/// Prints a reader line by line, with a banner prepended to each line.
fn print_reader(reader: *Reader, banner: []const u8) void {
    loop: while (true) {
        const line = reader.takeDelimiterInclusive('\n') catch |e| switch (e) {
            error.EndOfStream => break :loop,
            else => {
                log.err("Error while printing reader: {}\n", .{e});
                return;
            },
        };
        const line2 = if (std.mem.endsWith(u8, line, "\n")) line[0 .. line.len - 1] else line;
        log.info("{s}{s}", .{ banner, line2 });
    }
}

const StepTag = enum { args, arg };
const Step = union(StepTag) {
    const Self = @This();

    args: []const []const u8,
    arg: []const u8,

    pub fn toArgs(comptime self: *const Self) []const []const u8 {
        return switch (self.*) {
            .args => |args| args,
            .arg => |arg| &.{ "sh", "-c", arg },
        };
    }

    /// Export the entire command as one space-separated string.
    pub fn asStr(comptime self: *const Self) []const u8 {
        return switch (self.*) {
            .args => |args| join(args),
            .arg => |arg| arg,
        };
    }

    pub fn run(comptime self: *const Self, alloc: Allocator) !void {
        const args = self.toArgs();
        var proc = std.process.Child.init(args, alloc);
        proc.stdin_behavior = .Inherit;
        proc.stdout_behavior = .Pipe;
        proc.stderr_behavior = .Pipe;
        log.info("\x1b[33m$ {s}\x1b[m", .{self.asStr()});
        proc.spawn() catch |err| {
            log.err("Failed to spawn: {s}", .{self.asStr()});
            return err;
        };
        var stdout_buf: [1024]u8 = undefined;
        var stdout_reader = proc.stdout.?.reader(&stdout_buf);
        var stderr_buf: [1024]u8 = undefined;
        var stderr_reader = proc.stderr.?.reader(&stderr_buf);
        const t1 = try std.Thread.spawn(.{}, print_reader, .{ &stdout_reader.interface, "[stdout] " });
        const t2 = try std.Thread.spawn(.{}, print_reader, .{ &stderr_reader.interface, "[stderr] " });
        t1.join();
        t2.join();
        _ = try proc.wait();
    }

    pub fn run_all(comptime selves: []const Self, alloc: Allocator) !void {
        inline for (selves) |self| {
            try self.run(alloc);
        }
    }
};

fn cmds(comptime args: []const []const u8) Step {
    return Step{ .args = args };
}

fn cmd0(comptime arg: []const u8) Step {
    return Step{ .arg = arg };
}

const HOMEDIR = "/home/khang";
const REPODIR = path_join(HOMEDIR, "repos");

const NEOVIM_SOURCE_DIR = path_join(REPODIR, "neovim");
const NEOVIM_REMOTE_URL = "https://github.com/neovim/neovim.git";
const neovim = [_]Step{
    cmds(&.{ "mkdir", "-p", NEOVIM_SOURCE_DIR }),
    // cmds(&.{ "mkdir", "-p", NEOVIM_SOURCE_DIR }),
} ++ [_]Step{};

pub fn main() !void {
    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .init;
    const alloc = gpa.allocator();
    // std.meta.fields

    try Step.run_all(&neovim, alloc);
}
