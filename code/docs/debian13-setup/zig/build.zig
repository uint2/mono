const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const tool = b.addExecutable(.{
        .name = "generate_join",
        .root_module = b.createModule(.{
            .root_source_file = b.path("tools/generate_join.zig"),
            .target = b.graph.host,
        }),
    });
    const tool_step = b.addRunArtifact(tool);

    const exe = b.addExecutable(.{
        .name = "debian13",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{},
        }),
    });
    exe.step.dependOn(&tool_step.step);

    b.installArtifact(exe);

    const run_step = b.step("run", "Run the app");

    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);

    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| run_cmd.addArgs(args);
}
