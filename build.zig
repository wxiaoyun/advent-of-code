const std = @import("std");

pub fn build(b: *std.Build) void {
    const t = b.standardTargetOptions(.{});
    const o = b.standardOptimizeOption(.{});

    const arg_parser_dep = b.dependency("args", .{
        .target = t,
        .optimize = o,
    });

    const util_mod = b.addModule("util", .{ .root_source_file = b.path("zig/util.zig"), .target = t, .imports = &.{.{ .name = "args", .module = arg_parser_dep.module("args") }} });

    const common_imports: []const std.Build.Module.Import = &.{.{ .name = "util", .module = util_mod }};

    const AocYear = struct {
        year: u32,
        days: []const u8 = &.{},
    };

    const solutions = [_]AocYear{.{ .year = 2025, .days = &.{1} }};

    for (solutions) |aoc_year| {
        for (aoc_year.days) |day| {
            setup(b, t, o, common_imports, aoc_year.year, day);
        }
    }
}

fn setup(
    b: *std.Build,
    t: std.Build.ResolvedTarget,
    o: std.builtin.OptimizeMode,
    imports: []const std.Build.Module.Import,
    year: u32,
    day: u8,
) void {
    const name = b.fmt("aoc-{d}-{d}", .{ year, day });
    const path = b.fmt("{d}/zig/day{d:0>2}.zig", .{ year, day });

    const exe = b.addExecutable(.{ .name = name, .root_module = b.createModule(.{ .root_source_file = b.path(path), .target = t, .optimize = o, .imports = imports }) });

    b.installArtifact(exe);
    const run_aoc = b.addRunArtifact(exe);
    run_aoc.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_aoc.addArgs(args);
    }

    const run_step = b.step(name, "Run the specified AoC code");
    run_step.dependOn(&run_aoc.step);
}
