const std = @import("std");

const argsParser = @import("args");

const ArgSpec = struct {
    part: u8 = 1,

    pub const shorthands = .{
        .p = "part",
    };
};

pub fn parseArgs(allocator: std.mem.Allocator) !argsParser.ParseArgsResult(ArgSpec, null) {
    const options = try argsParser.parseForCurrentProcess(ArgSpec, allocator, .print);
    return options;
}

pub const INPUT_FOLDER_NAME: []const u8 = "input";

pub fn readInput(allocator: std.mem.Allocator, year: u32, day: u8) []const u8 {
    const file_path = std.fmt.allocPrint(allocator, "{d}/{s}/{d}.txt", .{ year, INPUT_FOLDER_NAME, day }) catch @panic("Failed to allocate file path");
    defer allocator.free(file_path);

    const cwd = std.fs.cwd();
    const input = cwd.readFileAlloc(
        allocator,
        file_path,
        std.math.maxInt(usize),
    ) catch @panic("Failed to read input");
    return input;
}
