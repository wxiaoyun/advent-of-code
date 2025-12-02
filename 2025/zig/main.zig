const std = @import("std");

const aoc = @import("aoc");
const util = @import("util");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();
    defer _ = gpa.deinit();

    const args = try util.parseArgs(alloc);
    defer args.deinit();

    switch (args.options.day) {
        1 => switch (args.options.part) {
            1 => aoc.day01.part1(alloc),
            2 => aoc.day01.part2(alloc),
            else => @panic("Part not implemented"),
        },
        else => @panic("Day not implemented"),
    }
}
