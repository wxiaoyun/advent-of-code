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

const CharMap = std.AutoHashMap(u8, u8);

fn part1(alloc: std.mem.Allocator, input: []u8) !i64 {
    var cs = CharMap.init(alloc);
    defer cs.deinit();
    try cs.ensureTotalCapacity(5);

    for (input, 0..) |c, i| {
        {
            const entry = cs.getOrPutAssumeCapacity(c);
            if (entry.found_existing) {
                entry.value_ptr.* += 1;
            } else {
                entry.value_ptr.* = 1;
            }
        }

        if (i >= 4) {
            const key = input[i - 4];
            const entry = cs.getEntry(key).?;
            entry.value_ptr.* -= 1;
            if (entry.value_ptr.* == 0) {
                _ = cs.remove(key);
            }
        }

        if (cs.unmanaged.size == 4) {
            const idx: i64 = @intCast(i);
            return idx + 1;
        }
    }

    return -1;
}

fn part2(alloc: std.mem.Allocator, input: []u8) !i64 {
    const n_distinct = 14;

    var cs = CharMap.init(alloc);
    defer cs.deinit();
    try cs.ensureTotalCapacity(n_distinct + 1);

    for (input, 0..) |c, i| {
        {
            const entry = cs.getOrPutAssumeCapacity(c);
            if (entry.found_existing) {
                entry.value_ptr.* += 1;
            } else {
                entry.value_ptr.* = 1;
            }
        }

        if (i >= n_distinct) {
            const key = input[i - n_distinct];
            const entry = cs.getEntry(key).?;
            entry.value_ptr.* -= 1;
            if (entry.value_ptr.* == 0) {
                _ = cs.remove(key);
            }
        }

        if (cs.unmanaged.size == n_distinct) {
            const idx: i64 = @intCast(i);
            return idx + 1;
        }
    }

    return -1;
}
