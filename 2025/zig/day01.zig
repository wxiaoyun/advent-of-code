const std = @import("std");

const util = @import("util");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();
    defer _ = gpa.deinit();

    const args = try util.parseArgs(alloc);
    defer args.deinit();

    const input = try util.readInput(alloc, 2025, 1);
    defer alloc.free(input);

    switch (args.options.part) {
        1 => try part1(alloc, input),
        2 => try part2(alloc, input),
        else => @panic("Illegal part"),
    }
}

fn part1(_: std.mem.Allocator, input: []const u8) !void {
    var lines = std.mem.splitSequence(u8, input, "\n");

    var zero_cnt: i64 = 0;
    var cur_dial: i64 = 50;
    while (lines.next()) |line| {
        if (line.len == 0) break;

        const dir: i64 = if (line[0] == 'L') -1 else 1;
        const step = std.fmt.parseInt(i64, line[1..], 10) catch @panic("Failed to parse step");

        cur_dial = @mod(cur_dial + dir * step, 100);
        if (cur_dial == 0) {
            zero_cnt += 1;
        }
    }

    std.debug.print("Zero count: {}\n", .{zero_cnt});
}

fn part2(_: std.mem.Allocator, input: []const u8) !void {
    var lines = std.mem.splitSequence(u8, input, "\n");

    var zero_cnt: i64 = 0;
    var cur_dial: i64 = 50;
    while (lines.next()) |line| {
        if (line.len == 0) break;

        const dir: i64 = if (line[0] == 'L') -1 else 1;
        const step = std.fmt.parseInt(i64, line[1..], 10) catch @panic("Failed to parse step");

        const n_cross_over = switch (dir) {
            1 => @divFloor(cur_dial + step, 100),
            -1 => @divFloor(@mod(100 - cur_dial, 100) + step, 100),
            else => unreachable,
        };

        cur_dial = @mod(cur_dial + dir * step, 100);
        zero_cnt += n_cross_over;
    }

    std.debug.print("Zero count: {}\n", .{zero_cnt});
}
