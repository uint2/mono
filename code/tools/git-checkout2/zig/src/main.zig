const std = @import("std");
const mem = std.mem;
const path = std.fs.path;

/// Represents the minimal amount of data for a worktree. In the examples below,
/// for the first worktree, `branch` would be "tree-1" and `dir` would be "dir-1".
///
/// worktree /home/z/repos/git-checkout2/.git/dir-1
/// HEAD 8e1777b9188170fffdfa221e1ec82042d21462d0
/// branch refs/heads/tree-1
///
/// worktree /home/z/repos/git-checkout2/.git/tree-2
/// HEAD 8e1777b9188170fffdfa221e1ec82042d21462d0
/// branch refs/heads/tree-2
const Worktree = struct {
    branch: []u8,
    dir: []u8,
};

pub fn main() !void {
    const use_gpa = true;
    const allocator: mem.Allocator = switch (use_gpa) {
        true => a: {
            var gpa = std.heap.GeneralPurposeAllocator(.{}).init;
            break :a gpa.allocator();
        },
        false => a: {
            var fbuffer: [0x1000]u8 = undefined;
            var fba: std.heap.FixedBufferAllocator = .init(&fbuffer);
            break :a fba.allocator();
        },
    };

    // Lists worktrees. Example output is
    // ```
    // worktree /home/z/repos/git-checkout2
    // HEAD 8e1777b9188170fffdfa221e1ec82042d21462d0
    // branch refs/heads/main
    //
    // worktree /home/z/repos/git-checkout2/.git/tree-1
    // HEAD 8e1777b9188170fffdfa221e1ec82042d21462d0
    // branch refs/heads/tree-1
    //
    // worktree /home/z/repos/git-checkout2/.git/tree-2
    // HEAD 8e1777b9188170fffdfa221e1ec82042d21462d0
    // branch refs/heads/tree-2
    // ```
    var git_worktree = std.process.Child.init(
        &.{ "git", "worktree", "list", "--porcelain" },
        allocator,
    );
    git_worktree.stdout_behavior = .Pipe;
    try git_worktree.spawn();

    var worktrees: std.ArrayList(Worktree) = .empty;
    {
        const buffer = try allocator.alloc(u8, 0x200);
        defer allocator.free(buffer);
        var reader_ = git_worktree.stdout.?.reader(buffer);
        var reader = &reader_.interface;
        loop: while (true) {
            const line = reader.takeDelimiterInclusive('\n') catch |e| switch (e) {
                error.EndOfStream => break :loop,
                else => return e,
            };
            if (mem.startsWith(u8, line, "worktree")) {
                const basename = mem.trimEnd(u8, path.basename(line), "\n");
                var t = try worktrees.addOne(allocator);
                t.dir = try allocator.alloc(u8, basename.len);
                @memcpy(t.dir, basename);
            } else if (mem.startsWith(u8, line, "branch")) {
                const basename = mem.trimEnd(u8, path.basename(line), "\n");
                var t: *Worktree = &worktrees.items[worktrees.items.len - 1];
                t.branch = try allocator.alloc(u8, basename.len);
                @memcpy(t.branch, basename);
            }
        }
    }
    _ = try git_worktree.wait();

    for (worktrees.items) |d| {
        std.debug.print("dir: {s} / {s}\n", .{ d.dir, d.branch });
    }
}
