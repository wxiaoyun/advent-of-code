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

fn part1(alloc: std.mem.Allocator, input: []const u8) !i64 {
    var nrow: usize = 0;
    var row_iter = std.mem.splitScalar(u8, input, '\n');
    while (row_iter.next()) |r| {
        if (r.len == 0) {
            @branchHint(.unlikely);
            break;
        }
        nrow += 1;
    }

    var mat = try std.ArrayList([]const u8).initCapacity(alloc, nrow);
    defer mat.deinit(alloc);

    row_iter.reset();
    while (row_iter.next()) |r| {
        if (r.len == 0) {
            @branchHint(.unlikely);
            break;
        }
        mat.appendAssumeCapacity(r);
    }
    const ncol = mat.items[0].len;

    var observables = std.AutoArrayHashMap(usize, void).init(alloc);
    defer observables.deinit();
    try observables.ensureTotalCapacity(nrow * ncol);

    for (0..nrow) |i| {
        const row = mat.items[i];
        var prefix_prev: u8 = 0;
        var suffix_prev: u8 = 0;
        for (0..ncol) |j| {
            if (row[j] > prefix_prev) {
                const cell = i * ncol + j;
                observables.putAssumeCapacity(cell, {});
            }
            prefix_prev = @max(prefix_prev, row[j]);

            const jj = ncol - j - 1;
            if (row[jj] > suffix_prev) {
                const cell = i * ncol + jj;
                observables.putAssumeCapacity(cell, {});
            }
            suffix_prev = @max(suffix_prev, row[jj]);
        }
    }

    for (0..ncol) |j| {
        var prefix_prev: u8 = 0;
        var suffix_prev: u8 = 0;
        for (0..nrow) |i| {
            if (mat.items[i][j] > prefix_prev) {
                const cell = i * ncol + j;
                observables.putAssumeCapacity(cell, {});
            }
            prefix_prev = @max(prefix_prev, mat.items[i][j]);

            const ii = nrow - i - 1;
            if (mat.items[ii][j] > suffix_prev) {
                const cell = ii * ncol + j;
                observables.putAssumeCapacity(cell, {});
            }
            suffix_prev = @max(suffix_prev, mat.items[ii][j]);
        }
    }

    return @intCast(observables.unmanaged.count());
}

fn part2(alloc: std.mem.Allocator, input: []const u8) !i64 {
    var nrow: usize = 0;
    var row_iter = std.mem.splitScalar(u8, input, '\n');
    while (row_iter.next()) |r| {
        if (r.len == 0) {
            @branchHint(.unlikely);
            break;
        }
        nrow += 1;
    }

    var mat = try std.ArrayList([]const u8).initCapacity(alloc, nrow);
    defer mat.deinit(alloc);

    row_iter.reset();
    while (row_iter.next()) |r| {
        if (r.len == 0) {
            @branchHint(.unlikely);
            break;
        }
        mat.appendAssumeCapacity(r);
    }
    _ = mat.items[0].len;

    return 0;
}
