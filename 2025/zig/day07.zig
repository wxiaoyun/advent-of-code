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

fn part1(alloc: std.mem.Allocator, input: []u8) !i64 {
    const Set = std.AutoArrayHashMap(usize, void);
    var beams = Set.init(alloc);
    defer beams.deinit();
    var tmp = Set.init(alloc);
    defer tmp.deinit();

    var splits: i64 = 0;
    var first_row: bool = true;
    var row_iter = std.mem.splitScalar(u8, input, '\n');
    while (row_iter.next()) |row| {
        if (row.len == 0) {
            @branchHint(.cold);
            break;
        }

        if (first_row) {
            @branchHint(.cold);
            first_row = false;
            try beams.ensureTotalCapacity(row.len);
            try tmp.ensureTotalCapacity(row.len);

            for (row, 0..) |ch, j| {
                if (ch == 'S') {
                    @branchHint(.cold);
                    _ = beams.fetchPutAssumeCapacity(j, {});
                }
            }
        }

        tmp.clearRetainingCapacity();
        var beams_iter = beams.iterator();
        while (beams_iter.next()) |entry| {
            const j = entry.key_ptr.*;
            if (row[j] != '^') {
                _ = tmp.fetchPutAssumeCapacity(j, {});
                continue;
            }

            splits += 1;
            if (j > 0) {
                _ = tmp.fetchPutAssumeCapacity(j - 1, {});
            }
            if (j + 1 < row.len) {
                _ = tmp.fetchPutAssumeCapacity(j + 1, {});
            }
        }

        std.mem.swap(Set, &beams, &tmp);
    }

    return splits;
}

fn part2(alloc: std.mem.Allocator, input: []u8) !i64 {
    const Map = std.AutoArrayHashMap(usize, i64);
    var beams = Map.init(alloc);
    defer beams.deinit();
    var tmp = Map.init(alloc);
    defer tmp.deinit();

    var first_row: bool = true;
    var row_iter = std.mem.splitScalar(u8, input, '\n');
    while (row_iter.next()) |row| {
        if (row.len == 0) {
            @branchHint(.cold);
            break;
        }

        if (first_row) {
            @branchHint(.cold);
            first_row = false;
            try beams.ensureTotalCapacity(row.len);
            try tmp.ensureTotalCapacity(row.len);

            for (row, 0..) |ch, j| {
                if (ch == 'S') {
                    @branchHint(.cold);
                    _ = beams.fetchPutAssumeCapacity(j, 1);
                }
            }
        }

        tmp.clearRetainingCapacity();
        var beams_iter = beams.iterator();
        while (beams_iter.next()) |entry| {
            const j = entry.key_ptr.*;
            const cnt = entry.value_ptr.*;
            if (row[j] != '^') {
                const prev = tmp.get(j) orelse 0;
                _ = tmp.fetchPutAssumeCapacity(j, prev + cnt);
                continue;
            }

            if (j > 0) {
                const prev = tmp.get(j - 1) orelse 0;
                _ = tmp.fetchPutAssumeCapacity(j - 1, prev + cnt);
            }
            if (j + 1 < row.len) {
                const prev = tmp.get(j + 1) orelse 0;
                _ = tmp.fetchPutAssumeCapacity(j + 1, prev + cnt);
            }
        }

        std.mem.swap(Map, &beams, &tmp);
    }

    var total: i64 = 0;
    var beams_iter = beams.iterator();
    while (beams_iter.next()) |entry| {
        total += entry.value_ptr.*;
    }
    return total;
}

const test_input =
    \\.......S.......
    \\...............
    \\.......^.......
    \\...............
    \\......^.^......
    \\...............
    \\.....^.^.^.....
    \\...............
    \\....^.^...^....
    \\...............
    \\...^.^...^.^...
    \\...............
    \\..^...^.....^..
    \\...............
    \\.^.^.^.^.^...^.
    \\...............
;

test part1 {
    const alc = std.testing.allocator;
    const input = try alc.alloc(u8, test_input.len);
    defer alc.free(input);
    @memcpy(input, test_input);
    try std.testing.expectEqual(21, try part1(alc, input));
}

test part2 {
    const alc = std.testing.allocator;
    const input = try alc.alloc(u8, test_input.len);
    defer alc.free(input);
    @memcpy(input, test_input);
    try std.testing.expectEqual(40, try part2(alc, input));
}
