const std = @import("std");

const util = @import("util");

pub fn main(init: std.process.Init) !void {
    const args = try util.parseArgs(init);
    const input = try util.readInputFromStdin(init);
    defer init.gpa.free(input);

    const result = switch (args.part) {
        1 => try part1(init.gpa, input),
        2 => try part2(init.gpa, input),
        else => @panic("Illegal part"),
    };

    std.debug.print("{}\n", .{result});
}

fn part1(_: std.mem.Allocator, input: []const u8) !i64 {
    var lines = std.mem.splitSequence(u8, input, "\n");

    var zero_cnt: i64 = 0;
    var cur_dial: i64 = 50;
    while (lines.next()) |line| {
        if (line.len == 0) break;

        const dir: i64 = if (line[0] == 'L') -1 else 1;
        const step = try std.fmt.parseInt(i64, line[1..], 10);

        cur_dial = @mod(cur_dial + dir * step, 100);
        if (cur_dial == 0) {
            zero_cnt += 1;
        }
    }

    return zero_cnt;
}

fn part2(_: std.mem.Allocator, input: []const u8) !i64 {
    var lines = std.mem.splitSequence(u8, input, "\n");

    var zero_cnt: i64 = 0;
    var cur_dial: i64 = 50;
    while (lines.next()) |line| {
        if (line.len == 0) break;

        const dir: i64 = if (line[0] == 'L') -1 else 1;
        const step = try std.fmt.parseInt(i64, line[1..], 10);

        const n_cross_over = switch (dir) {
            1 => @divFloor(cur_dial + step, 100),
            -1 => @divFloor(@mod(100 - cur_dial, 100) + step, 100),
            else => unreachable,
        };

        cur_dial = @mod(cur_dial + dir * step, 100);
        zero_cnt += n_cross_over;
    }

    return zero_cnt;
}
