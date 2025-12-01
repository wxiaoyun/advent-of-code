const std = @import("std");

const argsParser = @import("args");

const ArgSpec = struct {
    day: u8 = 1,
    part: u8 = 1,

    pub const shorthands = .{
        .d = "day",
        .p = "part",
    };
};

pub fn parseArgs(allocator: std.mem.Allocator) !argsParser.ParseArgsResult(ArgSpec, null) {
    const options = try argsParser.parseForCurrentProcess(ArgSpec, allocator, .print);
    return options;
}

pub const INPUT_FOLDER_NAME: []const u8 = "input";

pub fn readInput(allocator: std.mem.Allocator, year: u32, day: u8) []const u8 {
    const input = std.fs.cwd().readFile(allocator, "{d}/{s}/{d}.txt", .{ year, INPUT_FOLDER_NAME, day }) catch @panic("Failed to read input");
    return input;
}
