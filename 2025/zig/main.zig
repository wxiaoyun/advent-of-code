const std = @import("std");

const aoc = @import("aoc");
const util = @import("util");

pub fn main() !void {
    const args = try util.parseArgs(std.heap.page_allocator);

    switch (args.options.day) {
        1 => switch (args.options.part) {
            1 => aoc.day01.part1(),
            2 => aoc.day01.part2(),
        },
    }
}
