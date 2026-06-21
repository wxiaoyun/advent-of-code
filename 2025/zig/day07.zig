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

fn part1(alloc: std.mem.Allocator, input: []u8) !i64 {
    const Set = std.AutoArrayHashMapUnmanaged(usize, void);
    var beams = try Set.init(alloc, &[_]usize{}, &[_]void{});
    defer beams.deinit(alloc);
    var tmp = try Set.init(alloc, &[_]usize{}, &[_]void{});
    defer tmp.deinit(alloc);

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
            try beams.ensureTotalCapacity(alloc, row.len);
            try tmp.ensureTotalCapacity(alloc, row.len);

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
    const Map = std.AutoArrayHashMapUnmanaged(usize, i64);
    var beams = Map.empty;
    defer beams.deinit(alloc);
    var tmp = Map.empty;
    defer tmp.deinit(alloc);

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
            try beams.ensureTotalCapacity(alloc, row.len);
            try tmp.ensureTotalCapacity(alloc, row.len);

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
