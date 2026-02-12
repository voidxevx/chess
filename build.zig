//! zig build scripting
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Core module -- used as pch
    const coreMod = b.addModule("core", .{
        .root_source_file = b.path("src/core.zig"),
        .target = target,
    });

    // root module -- this is separate to allow it to be used for the static library
    // and as a module that can be included by the zig executables
    const root_mod = b.createModule(.{
        .root_source_file = b.path("src/root.zig"),
        .target =  target,
        .optimize = optimize,
        .imports = &.{
            .{ .name = "core", .module = coreMod }
        }
    });

    // branch of zig build that only compiles the the static library linked by rust
    // invoked using zig build -Dlib=true
    const lib_only = b.option(bool, "lib", "Compile only the static library") orelse false;
    if (lib_only) {
        // main library accessed by rust files
        const lib = b.addLibrary(.{
            .name = "chess",
            .linkage = .static,
            .root_module = root_mod,
        });
        lib.pie = true;

        b.installArtifact(lib);
    } else {
        // build the rust static library before linking 
        // into the zig exes.
        // this will automatically call the above branch to compile the zig library.
        const cargo_build = b.addSystemCommand(&.{
            "cargo",
            "build"
        });
        
        // rust build is invoked using zig build cargo
        const cargo_build_step = b.step("cargo", "Build the rust static lib before zig exes");
        cargo_build_step.dependOn(&cargo_build.step);


        // Client executable
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

        client_exe.pie = true;

        // link the rust static library into the client exe
        client_exe.addLibraryPath(b.path("target/debug"));
        client_exe.linkSystemLibrary("bridge");
        client_exe.linkSystemLibrary("unwind");
        client_exe.linkLibC();
        b.installArtifact(client_exe);


        // server head executable
        const head_exe = b.addExecutable(.{
            .name = "head",
            .root_module = b.createModule(.{
                .root_source_file = b.path("src/head/main.zig"),
                .target = target,
                .optimize = optimize,
                .imports = &.{
                    .{ .name = "chess", .module = root_mod },
                    .{ .name = "core", .module = coreMod }
                }
            }),
        });

        // link the rust static library into the server head exe
        head_exe.addLibraryPath(b.path("target/debug"));
        head_exe.linkSystemLibrary("bridge");
        head_exe.linkSystemLibrary("unwind");
        b.installArtifact(head_exe);

        // tests for client
        const client_tests = b.addTest(.{
            .root_module =  client_exe.root_module,
        });
        // tests for server head
        const head_tests = b.addTest(.{
            .root_module = head_exe.root_module,
        });
        // run artifacts
        const client_run_tests = b.addRunArtifact(client_tests);
        const head_run_tests = b.addRunArtifact(head_tests);

        // client tests invoked by zig build cargo test-client
        const client_test_step = b.step("test-client", "Run client tests");
        client_test_step.dependOn(&client_run_tests.step);
        // server head tests invoked by zig build cargo test-head
        const head_test_step = b.step("test-head", "Run server head tests");
        head_test_step.dependOn(&head_run_tests.step);


        // client run artifact
        const run_client = b.addRunArtifact(client_exe);
        run_client.step.dependOn(b.getInstallStep());
        // server head run artifact
        const run_head = b.addRunArtifact(head_exe);
        run_head.step.dependOn(b.getInstallStep());

        if (b.args) |args| {
            run_client.addArgs(args);
            run_head.addArgs(args);
        }

        // running the client is invoked by zig build cargo run-client
        const run_client_step = b.step("run-client", "Run the client executable");
        run_client_step.dependOn(&run_client.step);

        // running server head is invoked by zig build cargo run-head
        const run_head_step = b.step("run-head", "Run the server head executable");
        run_head_step.dependOn(&run_head.step);
       
    }

    // call tests for rust static lib
    const cargo_test = b.addSystemCommand(&.{
        "cargo",
        "test"
    });

    // tests from root module
    const root_tests = b.addTest(.{
        .root_module = root_mod,
    });
    const root_run_tests = b.addRunArtifact(root_tests);

    // tests for root module and rust lib is invoked by zig build cargo test-root
    const test_step = b.step("test-root", "Run tests");
    test_step.dependOn(&root_run_tests.step);
    test_step.dependOn(&cargo_test.step);

}
