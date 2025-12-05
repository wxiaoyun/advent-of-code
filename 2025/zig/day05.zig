const std = @import("std");

const util = @import("util");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();
    defer _ = gpa.deinit();

    const args = try util.parseArgs();
    const input = try util.readInputFromStdin(alloc);
    defer alloc.free(input);

    const result = switch (args.part) {
        1 => try part1(alloc, input),
        2 => try part2(alloc, input),
        else => @panic("Illegal part"),
    };

    std.debug.print("Result: {}\n", .{result});
}

fn compareTuple(_: void, this: [2]i64, other: [2]i64) bool {
    if (this[0] < other[0]) {
        return true;
    }
    if (this[0] > other[0]) {
        return false;
    }
    if (this[1] < other[1]) {
        return true;
    }
    return false;
}

fn compareInt(_: void, this: i64, other: i64) bool {
    if (this < other) {
        return true;
    }
    return false;
}

fn part1(alloc: std.mem.Allocator, input: []u8) !i64 {
    var parts = std.mem.splitSequence(u8, input, "\n\n");
    const ranges_raw = parts.next().?;
    const queries_raw = parts.next().?;

    var ranges_iter = std.mem.splitScalar(u8, ranges_raw, '\n');
    var ranges = std.ArrayList([2]i64).empty;
    defer ranges.deinit(alloc);

    while (ranges_iter.next()) |row| {
        if (row.len == 0) {
            break;
        }
        var nums = std.mem.splitScalar(u8, row, '-');
        const left = try std.fmt.parseInt(i64, nums.next().?, 10);
        const right = try std.fmt.parseInt(i64, nums.next().?, 10);
        try ranges.append(alloc, .{ left, right });
    }

    var queries_iter = std.mem.splitScalar(u8, queries_raw, '\n');
    var queries = std.ArrayList(i64).empty;
    defer queries.deinit(alloc);

    while (queries_iter.next()) |row| {
        if (row.len == 0) {
            break;
        }
        const num = try std.fmt.parseInt(i64, row, 10);
        try queries.append(alloc, num);
    }

    std.sort.heap([2]i64, ranges.items, {}, compareTuple);
    var merged_ranges = std.ArrayList([2]i64).empty;
    defer merged_ranges.deinit(alloc);
    try merged_ranges.ensureTotalCapacity(alloc, ranges.items.len);

    try ranges.append(alloc, .{ std.math.maxInt(i64), std.math.maxInt(i64) });
    var prev = ranges.items[0];
    for (1..ranges.items.len) |i| {
        const cur = ranges.items[i];
        if (prev[1] < cur[0]) {
            try merged_ranges.append(alloc, prev);
            prev = cur;
            continue;
        }

        prev[1] = @max(prev[1], cur[1]);
    }

    std.sort.heap(i64, queries.items, {}, compareInt);

    var fresh_cnt: i64 = 0;
    var i: usize = 0;
    outer: for (queries.items) |q| {
        while (i < merged_ranges.items.len) {
            const range = merged_ranges.items[i];

            // q < s <= e
            if (q < range[0]) {
                continue :outer;
            }

            // s <= q <= e
            if (q <= range[1]) {
                fresh_cnt += 1;
                continue :outer;
            }

            // s <= e < q
            i += 1;
        }
    }

    return fresh_cnt;
}

fn part2(alloc: std.mem.Allocator, input: []u8) !i64 {
    var parts = std.mem.splitSequence(u8, input, "\n\n");
    const ranges_raw = parts.next().?;

    var ranges_iter = std.mem.splitScalar(u8, ranges_raw, '\n');
    var ranges = std.ArrayList([2]i64).empty;
    defer ranges.deinit(alloc);

    while (ranges_iter.next()) |row| {
        if (row.len == 0) {
            break;
        }
        var nums = std.mem.splitScalar(u8, row, '-');
        const left = try std.fmt.parseInt(i64, nums.next().?, 10);
        const right = try std.fmt.parseInt(i64, nums.next().?, 10);
        try ranges.append(alloc, .{ left, right });
    }

    std.sort.heap([2]i64, ranges.items, {}, compareTuple);
    var merged_ranges = std.ArrayList([2]i64).empty;
    defer merged_ranges.deinit(alloc);
    try merged_ranges.ensureTotalCapacity(alloc, ranges.items.len);

    try ranges.append(alloc, .{ std.math.maxInt(i64), std.math.maxInt(i64) });
    var prev = ranges.items[0];
    for (1..ranges.items.len) |i| {
        const cur = ranges.items[i];
        if (prev[1] < cur[0]) {
            try merged_ranges.append(alloc, prev);
            prev = cur;
            continue;
        }

        prev[1] = @max(prev[1], cur[1]);
    }

    var fresh_cnt: i64 = 0;
    for (merged_ranges.items) |range| {
        fresh_cnt += range[1] - range[0] + 1;
    }

    return fresh_cnt;
}

const test_input =
    \\3-5
    \\10-14
    \\16-20
    \\12-18
    \\
    \\1
    \\5
    \\8
    \\11
    \\17
    \\32
;

test part1 {
    const alc = std.testing.allocator;
    const input = try alc.alloc(u8, test_input.len);
    defer alc.free(input);
    @memcpy(input, test_input);
    try std.testing.expectEqual(3, try part1(alc, input));
}

test part2 {
    const alc = std.testing.allocator;
    const input = try alc.alloc(u8, test_input.len);
    defer alc.free(input);
    @memcpy(input, test_input);
    try std.testing.expectEqual(14, try part2(alc, input));
}
