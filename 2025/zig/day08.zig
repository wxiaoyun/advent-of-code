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
            std.debug.print("{s}\n", .{coord_str});
            const coord_num = try std.fmt.parseInt(i64, coord_str, 10);
            coord[i] = coord_num;
        }
        coords.appendAssumeCapacity(coord);
    }

    var heap = MinTupleHeap.init(alloc, {});
    defer heap.deinit();
    try heap.ensureTotalCapacity(ncoords);

    for (0..ncoords) |i| {
        const ix = coords.items[i][0];
        const iy = coords.items[i][1];
        const iz = coords.items[i][2];
        for (i + 1..ncoords) |j| {
            const jx = coords.items[j][0];
            const jy = coords.items[j][1];
            const jz = coords.items[j][2];

            const dist_sq = std.math.pow(i64, ix - jx, 2) + std.math.pow(i64, iy - jy, 2) + std.math.pow(i64, iz - jz, 2);

            try heap.add(.{ @intCast(dist_sq), i, j });
        }
    }

    var uf = try util.UnionFind.init(alloc, ncoords);
    defer uf.deinit(alloc);
    for (0..CONNECTIONS) |_| {
        const tuple = heap.removeOrNull() orelse break;
        uf.@"union"(tuple[1], tuple[2]);
    }

    var map = std.AutoArrayHashMap(usize, usize).init(alloc);
    try map.ensureTotalCapacity(ncoords);
    defer map.deinit();
    for (0..CONNECTIONS) |n| {
        const component = uf.find(n);
        _ = map.fetchPutAssumeCapacity(component, uf.rank[component]);
    }

    var size_heap = MinSizeHeap.init(alloc, {});
    defer size_heap.deinit();
    try size_heap.ensureTotalCapacity(4);

    var map_iter = map.iterator();
    while (map_iter.next()) |e| {
        try size_heap.add(e.value_ptr.*);
        if (size_heap.items.len > 3) {
            _ = size_heap.remove();
        }
    }

    var result: i64 = 1;
    for (size_heap.items) |s| {
        result *= @intCast(s);
    }
    return result;
}

fn part2(_: std.mem.Allocator, _: []u8) !i64 {
    return 0;
}

const test_input =
    \\162,817,812
    \\57,618,57
    \\906,360,560
    \\592,479,940
    \\352,342,300
    \\466,668,158
    \\542,29,236
    \\431,825,988
    \\739,650,466
    \\52,470,668
    \\216,146,977
    \\819,987,18
    \\117,168,530
    \\805,96,715
    \\346,949,466
    \\970,615,88
    \\941,993,340
    \\862,61,35
    \\984,92,344
    \\425,690,689
;

test part1 {
    const alc = std.testing.allocator;
    const input = try alc.alloc(u8, test_input.len);
    defer alc.free(input);
    @memcpy(input, test_input);
    CONNECTIONS = 10;
    try std.testing.expectEqual(40, try part1(alc, input));
}

test part2 {
    const alc = std.testing.allocator;
    const input = try alc.alloc(u8, test_input.len);
    defer alc.free(input);
    @memcpy(input, test_input);
    try std.testing.expectEqual(25272, try part2(alc, input));
}
