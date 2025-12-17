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

fn part1(_: std.mem.Allocator, input: []u8) !i64 {
    var rows = std.mem.splitScalar(u8, input, '\n');
    var nrow: usize = 0;
    var ncol: usize = 0;
    while (rows.next()) |row| : (nrow += 1) {
        if (row.len == 0) {
            break;
        }
        ncol = row.len;
    }

    var n_accessible: i64 = 0;
    for (0..nrow) |i| {
        for (0..ncol) |j| {
            if (isAccessible(input, nrow, ncol, i, j)) {
                n_accessible += 1;
            }
        }
    }

    return n_accessible;
}

fn isAccessible(mat: []u8, nrow: usize, ncol: usize, r: usize, c: usize) bool {
    const row_len = ncol + 1;
    if (mat[r * row_len + c] != '@') {
        return false;
    }
    var r_start: usize = 0;
    if (r > 0) {
        r_start = r - 1;
    }
    var r_end = nrow;
    if (r + 2 <= nrow) {
        r_end = r + 2;
    }
    var c_start: usize = 0;
    if (c > 0) {
        c_start = c - 1;
    }
    var c_end = ncol;
    if (c + 2 <= ncol) {
        c_end = c + 2;
    }

    var roll_cnt: usize = 0;
    for (r_start..r_end) |rr| {
        for (c_start..c_end) |cc| {
            if (r == rr and c == cc) {
                continue;
            }
            if (mat[rr * row_len + cc] == '@') {
                roll_cnt += 1;
            }
        }
    }

    return roll_cnt < 4;
}

fn part2(alloc: std.mem.Allocator, input: []u8) !i64 {
    var rows = std.mem.splitScalar(u8, input, '\n');
    var nrow: usize = 0;
    var ncol: usize = 0;
    while (rows.next()) |row| : (nrow += 1) {
        if (row.len == 0) {
            break;
        }
        ncol = row.len;
    }

    // Holds pairs of (row, col) tuples
    var work_queue = std.ArrayList(usize).empty;
    defer work_queue.deinit(alloc);
    try work_queue.ensureTotalCapacity(alloc, nrow * ncol * 2);

    for (0..nrow) |i| {
        for (0..ncol) |j| {
            if (isAccessible(input, nrow, ncol, i, j)) {
                try work_queue.append(alloc, i);
                try work_queue.append(alloc, j);
            }
        }
    }

    var n_accessible: i64 = 0;
    while (work_queue.items.len > 1) {
        const c = work_queue.pop().?;
        const r = work_queue.pop().?;
        if (isAccessible(input, nrow, ncol, r, c)) {
            try removeSelfAndNotifyNeighbor(alloc, &work_queue, input, nrow, ncol, r, c);
            n_accessible += 1;
        }
    }
    return n_accessible;
}

fn removeSelfAndNotifyNeighbor(alloc: std.mem.Allocator, work_queue: *std.ArrayList(usize), mat: []u8, nrow: usize, ncol: usize, r: usize, c: usize) !void {
    const row_len = ncol + 1;

    mat[r * row_len + c] = '.';

    var r_start: usize = 0;
    if (r > 0) {
        r_start = r - 1;
    }
    var r_end = nrow;
    if (r + 2 <= nrow) {
        r_end = r + 2;
    }
    var c_start: usize = 0;
    if (c > 0) {
        c_start = c - 1;
    }
    var c_end = ncol;
    if (c + 2 <= ncol) {
        c_end = c + 2;
    }

    for (r_start..r_end) |rr| {
        for (c_start..c_end) |cc| {
            if (mat[rr * row_len + cc] == '@') {
                try work_queue.append(alloc, rr);
                try work_queue.append(alloc, cc);
            }
        }
    }
}
