const std = @import("std");

const Args = struct {
    part: u8 = 1,
};

pub fn parseArgs() !Args {
    if (std.os.argv.len < 2) {
        @panic("Positional argument part is required");
    }

    const part_str = std.mem.span(std.os.argv[1]);
    const part = try std.fmt.parseInt(u8, part_str, 10);
    return Args{ .part = part };
}

pub const INPUT_FOLDER_NAME: []const u8 = "input";

pub fn readInputFromStdin(allocator: std.mem.Allocator) ![]u8 {
    const stdin = std.fs.File.stdin();
    const stat = try stdin.stat();

    var reader_buf: [1024]u8 = undefined;
    var reader = stdin.reader(&reader_buf);
    return try reader.interface.readAlloc(allocator, stat.size);
}
