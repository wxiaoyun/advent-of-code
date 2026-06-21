const std = @import("std");

pub const UnionFind = @import("union_find.zig");

const Args = struct {
    part: u8 = 1,
};

pub fn parseArgs(init: std.process.Init) !Args {
    const args = try init.minimal.args.toSlice(init.gpa);
    defer init.gpa.free(args);
    if (args.len < 2) {
        @panic("Positional argument part is required");
    }

    const part_str = std.mem.span(args[1].ptr);
    const part = try std.fmt.parseInt(u8, part_str, 10);
    return Args{ .part = part };
}

pub fn readInputFromStdin(init: std.process.Init) ![]u8 {
    const stdin = std.Io.File.stdin();
    const stat = try stdin.stat(init.io);

    var reader_buf: [1024]u8 = undefined;
    var reader = stdin.reader(init.io, &reader_buf);
    return try reader.interface.readAlloc(init.gpa, stat.size);
}
