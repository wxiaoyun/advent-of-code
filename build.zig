const std = @import("std");

// Although this function looks imperative, it does not perform the build
// directly and instead it mutates the build graph (`b`) that will be then
// executed by an external runner. The functions in `std.Build` implement a DSL
// for defining build steps and express dependencies between them, allowing the
// build runner to parallelize the build automatically (and the cache system to
// know when a step doesn't need to be re-run).
pub fn build(b: *std.Build) void {
    // Standard target options allow the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});
    // Standard optimization options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall. Here we do not
    // set a preferred release mode, allowing the user to decide how to optimize.
    const optimize = b.standardOptimizeOption(.{});
    // It's also possible to define more custom flags to toggle optional features
    // of this build script using `b.option()`. All defined flags (including
    // target and optimize options) will be listed when running `zig build --help`
    // in this directory.

    // This creates a module, which represents a collection of source files alongside
    // some compilation options, such as optimization mode and linked system libraries.
    // Zig modules are the preferred way of making Zig code available to consumers.
    // addModule defines a module that we intend to make available for importing
    // to our consumers. We must give it a name because a Zig package can expose
    // multiple modules and consumers will need to be able to specify which
    // module they want to access.

    const arg_parser_dep = b.dependency("args", .{
        .target = target,
        .optimize = optimize,
    });

    const util_mod = b.addModule("util", .{ .root_source_file = b.path("zig/util.zig"), .target = target, .imports = &.{.{ .name = "args", .module = arg_parser_dep.module("args") }} });

    const mod_2025 = b.addModule("aoc-2025", .{
        .root_source_file = b.path("2025/zig/aoc.zig"),
        .target = target,
        .imports = &.{.{ .name = "util", .module = util_mod }},
    });

    const exe_2025 = b.addExecutable(.{ .name = "aoc-2025", .root_module = b.createModule(.{ .root_source_file = b.path("2025/zig/main.zig"), .target = target, .optimize = optimize, .imports = &.{
        .{ .name = "aoc", .module = mod_2025 },
        .{ .name = "util", .module = util_mod },
    } }) });

    // This declares intent for the executable to be installed into the
    // install prefix when running `zig build` (i.e. when executing the default
    // step). By default the install prefix is `zig-out/` but can be overridden
    // by passing `--prefix` or `-p`.
    b.installArtifact(exe_2025);

    const run_2025_aoc = b.addRunArtifact(exe_2025);
    run_2025_aoc.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_2025_aoc.addArgs(args);
    }

    // This creates a top level step. Top level steps have a name and can be
    // invoked by name when running `zig build` (e.g. `zig build run`).
    // This will evaluate the `run` step rather than the default step.
    // For a top level step to actually do something, it must depend on other
    // steps (e.g. a Run step, as we will see in a moment).
    const run_2025_step = b.step("run-2025", "Run the AoC 2025 code");

    // This creates a RunArtifact step in the build graph. A RunArtifact step
    // invokes an executable compiled by Zig. Steps will only be executed by the
    // runner if invoked directly by the user (in the case of top level steps)
    // or if another step depends on it, so it's up to you to define when and
    // how this Run step will be executed. In our case we want to run it when
    // the user runs `zig build run`, so we create a dependency link.
    run_2025_step.dependOn(&run_2025_aoc.step);
}
