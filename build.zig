//! zig build scripting
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib_only = b.option(bool, "lib", "Compile only the static library") orelse false;

    // Core module -- used as pch
    const coreMod = b.addModule("core", .{
        .root_source_file = b.path("src/core.zig"),
        .target = target,
    });

    const root_mod = b.createModule(.{
        .root_source_file = b.path("src/root.zig"),
        .target =  target,
        .optimize = optimize,
        .imports = &.{
            .{ .name = "core", .module = coreMod }
        }
    });

    if (lib_only) {
        // main library accessed by both zig and rust files
        const lib = b.addLibrary(.{
            .name = "chess",
            .linkage = .static,
            .root_module = root_mod,
        });
        lib.pie = true;

        b.installArtifact(lib);
    } else {
        // const cargo_build = b.addSystemCommand(&.{
        //     "cargo",
        //     "build"
        // });
        //
        // const cargo_build_step = b.step("rsbuild", "Build the rust project");
        // b.default_step.dependOn(&cargo_build_step)
        
        const client_exe = b.addExecutable(.{
            .name = "client",
            .root_module = b.createModule(.{
                .root_source_file = b.path("src/client/main.zig"),
                .target = target,
                .optimize = optimize,
                .imports = &.{
                    .{ .name = "chess", .module = root_mod },
                    .{ .name = "core", .module = coreMod }
                }
            }),
        });

        client_exe.addLibraryPath(b.path("target/debug"));
        client_exe.linkSystemLibrary("chess_rs_bridge");

        b.installArtifact(client_exe);

        const head_exe = b.addExecutable(.{
            .name = "head",
            .root_module = b.createModule(.{
                .root_source_file = b.path("src/client/main.zig"),
                .target = target,
                .optimize = optimize,
                .imports = &.{
                    .{ .name = "chess", .module = root_mod },
                    .{ .name = "core", .module = coreMod }
                }
            })
        });

        head_exe.addLibraryPath(b.path("target/debug"));
        head_exe.linkSystemLibrary("chess_rs_bridge");

        b.installArtifact(head_exe);

        const client_tests = b.addTest(.{
            .root_module =  client_exe.root_module,
        });

        const head_tests = b.addTest(.{
            .root_module = head_exe.root_module,
        });

        const client_run_tests = b.addRunArtifact(client_tests);
        const head_run_tests = b.addRunArtifact(head_tests);

        const client_test_step = b.step("test-client", "Run client tests");
        client_test_step.dependOn(&client_run_tests.step);

        const head_test_step = b.step("test-head", "Run server head tests");
        head_test_step.dependOn(&head_run_tests.step);
    }

    const root_tests = b.addTest(.{
        .root_module = root_mod,
    });

    const root_run_tests = b.addRunArtifact(root_tests);

    const test_step = b.step("test-root", "Run tests");
    test_step.dependOn(&root_run_tests.step);

}
