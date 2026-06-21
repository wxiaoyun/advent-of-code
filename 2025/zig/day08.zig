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

var CONNECTIONS: usize = 1000;

fn compareTuple(_: void, this: [3]usize, other: [3]usize) std.math.Order {
    inline for (0..3) |i| {
        if (this[i] < other[i]) {
            return .lt;
        }
        if (this[i] > other[i]) {
            return .gt;
        }
    }
    return .eq;
}

const MinTupleHeap = std.PriorityQueue([3]usize, void, compareTuple);

fn compareSize(_: void, this: usize, other: usize) std.math.Order {
    if (this < other) {
        return .lt;
    }
    return .gt;
}

const MinSizeHeap = std.PriorityQueue(usize, void, compareSize);

fn part1(alloc: std.mem.Allocator, input: []u8) !i64 {
    var row_iter = std.mem.splitScalar(u8, input, '\n');
    var ncoords: usize = 0;
    while (row_iter.next()) |row| {
        if (row.len == 0) {
            @branchHint(.cold);
            break;
        }
        ncoords += 1;
    }

    var coords = std.ArrayList([3]i64).empty;
    try coords.ensureTotalCapacity(alloc, ncoords);
    defer coords.deinit(alloc);

    row_iter.reset();
    while (row_iter.next()) |row| {
        if (row.len == 0) {
            @branchHint(.cold);
            break;
        }

        var coord_iter = std.mem.splitScalar(u8, row, ',');
        var coord: [3]i64 = undefined;
        inline for (0..3) |i| {
            const coord_str = coord_iter.next().?;
            const coord_num = try std.fmt.parseInt(i64, coord_str, 10);
            coord[i] = coord_num;
        }
        coords.appendAssumeCapacity(coord);
    }

    var heap = MinTupleHeap.initContext({});
    defer heap.deinit(alloc);
    try heap.ensureTotalCapacity(alloc, ncoords);

    for (0..ncoords) |i| {
        const ix = coords.items[i][0];
        const iy = coords.items[i][1];
        const iz = coords.items[i][2];
        for (i + 1..ncoords) |j| {
            const jx = coords.items[j][0];
            const jy = coords.items[j][1];
            const jz = coords.items[j][2];

            const dist_sq = std.math.pow(i64, ix - jx, 2) + std.math.pow(i64, iy - jy, 2) + std.math.pow(i64, iz - jz, 2);

            try heap.push(alloc, .{ @intCast(dist_sq), i, j });
        }
    }

    var uf = try util.UnionFind.init(alloc, ncoords);
    defer uf.deinit(alloc);
    for (0..CONNECTIONS) |_| {
        const tuple = heap.pop() orelse break;
        const i = tuple[1];
        const j = tuple[2];
        uf.@"union"(i, j);
    }

    var map = std.AutoArrayHashMapUnmanaged(usize, usize).empty;
    try map.ensureTotalCapacity(alloc, ncoords);
    defer map.deinit(alloc);
    for (0..CONNECTIONS) |n| {
        const component = uf.find(n);
        _ = map.fetchPutAssumeCapacity(component, uf.rank[component]);
    }

    var size_heap = MinSizeHeap.empty;
    defer size_heap.deinit(alloc);
    try size_heap.ensureTotalCapacity(alloc, 4);

    var map_iter = map.iterator();
    while (map_iter.next()) |e| {
        try size_heap.push(alloc, e.value_ptr.*);
        if (size_heap.items.len > 3) {
            _ = size_heap.pop();
        }
    }

    var result: i64 = 1;
    for (size_heap.items) |s| {
        result *= @intCast(s);
    }
    return result;
}

fn part2(alloc: std.mem.Allocator, input: []u8) !i64 {
    var row_iter = std.mem.splitScalar(u8, input, '\n');
    var ncoords: usize = 0;
    while (row_iter.next()) |row| {
        if (row.len == 0) {
            @branchHint(.cold);
            break;
        }
        ncoords += 1;
    }

    var coords = std.ArrayList([3]i64).empty;
    try coords.ensureTotalCapacity(alloc, ncoords);
    defer coords.deinit(alloc);

    row_iter.reset();
    while (row_iter.next()) |row| {
        if (row.len == 0) {
            @branchHint(.cold);
            break;
        }

        var coord_iter = std.mem.splitScalar(u8, row, ',');
        var coord: [3]i64 = undefined;
        inline for (0..3) |i| {
            const coord_str = coord_iter.next().?;
            const coord_num = try std.fmt.parseInt(i64, coord_str, 10);
            coord[i] = coord_num;
        }
        coords.appendAssumeCapacity(coord);
    }

    var heap = MinTupleHeap.empty;
    defer heap.deinit(alloc);
    try heap.ensureTotalCapacity(alloc, ncoords);

    for (0..ncoords) |i| {
        const ix = coords.items[i][0];
        const iy = coords.items[i][1];
        const iz = coords.items[i][2];
        for (i + 1..ncoords) |j| {
            const jx = coords.items[j][0];
            const jy = coords.items[j][1];
            const jz = coords.items[j][2];

            const dist_sq = std.math.pow(i64, ix - jx, 2) + std.math.pow(i64, iy - jy, 2) + std.math.pow(i64, iz - jz, 2);

            try heap.push(alloc, .{ @intCast(dist_sq), i, j });
        }
    }

    var uf = try util.UnionFind.init(alloc, ncoords);
    defer uf.deinit(alloc);
    while (true) {
        const tuple = heap.pop() orelse break;
        const i = tuple[1];
        const j = tuple[2];
        uf.@"union"(i, j);

        if (uf.isConnected()) {
            return coords.items[i][0] * coords.items[j][0];
        }
    }

    unreachable;
}
