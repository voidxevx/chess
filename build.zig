//! zig build scripting
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const coreMod = b.addModule("core", .{
        .root_source_file = b.path("src/core.zig"),
        .target = target,
    });

    const lib = b.addLibrary(.{
        .name = "chess",
        .linkage = .static,
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/root.zig"),
            .target =  target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "core", .module = coreMod }
            }
        }),
    });

    lib.pie = true;

    b.installArtifact(lib);


    const tests = b.addTest(.{
        .root_module = lib.root_module,
    });

    const run_tests = b.addRunArtifact(tests);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_tests.step);

}
