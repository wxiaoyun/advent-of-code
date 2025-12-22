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

const File = struct {
    parent: ?*File = null,
    data: union(enum) {
        file: usize,
        dir: std.StringArrayHashMapUnmanaged(File),
    },

    const Self = @This();

    fn newFile(s: usize) File {
        return .{ .data = .{ .file = s } };
    }

    fn newDir() File {
        return .{ .data = .{ .dir = .empty } };
    }

    fn size(self: *Self) usize {
        switch (self.data) {
            .file => |s| return s,
            .dir => |d| {
                var s: usize = 0;

                var iter = d.iterator();
                while (iter.next()) |f| {
                    s += f.value_ptr.size();
                }

                return s;
            },
        }
    }

    fn sumWithinLimit(self: *Self, limit: usize) usize {
        switch (self.data) {
            .file => return 0,
            .dir => |d| {
                var total_sizes: usize = 0;

                var iter = d.iterator();
                while (iter.next()) |f| {
                    total_sizes += f.value_ptr.sumWithinLimit(limit);
                }

                const dir_size = self.size();
                if (dir_size <= limit) {
                    total_sizes += dir_size;
                }

                return total_sizes;
            },
        }
    }

    fn smallestDirAboveLimit(self: *Self, limit: usize) usize {
        switch (self.data) {
            .file => return std.math.maxInt(usize),
            .dir => |d| {
                var best: usize = std.math.maxInt(usize);

                const self_size = self.size();
                if (self_size >= limit) {
                    best = self_size;
                }

                var iter = d.iterator();
                while (iter.next()) |f| {
                    const child_best = f.value_ptr.smallestDirAboveLimit(limit);
                    best = @min(best, child_best);
                }

                return best;
            },
        }
    }
};

const FileSystem = struct {
    arena: std.heap.ArenaAllocator,
    root: *File,
    cwd: *File,

    const Self = @This();

    fn init(alloc: std.mem.Allocator) !Self {
        var arena = std.heap.ArenaAllocator.init(alloc);
        var arena_allocator = arena.allocator();

        const root_dir = try arena_allocator.create(File);
        root_dir.* = File.newDir();

        return .{
            .arena = arena,
            .root = root_dir,
            .cwd = root_dir,
        };
    }

    fn deinit(self: *Self) void {
        self.arena.deinit();
        self.root = undefined;
        self.cwd = undefined;
    }

    fn insertFile(self: *Self, name: []const u8, size: usize) !void {
        const alloc = self.arena.allocator();

        switch (self.cwd.data) {
            .file => |_| @panic("file insertion into file"),
            .dir => |*dir| {
                const res = try dir.getOrPut(alloc, name);
                if (res.found_existing) {
                    return;
                }

                var file = File.newFile(size);
                file.parent = self.cwd;
                res.value_ptr.* = file;
            },
        }
    }

    fn insertDir(self: *Self, name: []const u8) !void {
        const alloc = self.arena.allocator();

        switch (self.cwd.data) {
            .file => |_| @panic("file insertion into file"),
            .dir => |*dir| {
                const res = try dir.getOrPut(alloc, name);
                if (res.found_existing) {
                    return;
                }

                var new_dir = File.newDir();
                new_dir.parent = self.cwd;
                res.value_ptr.* = new_dir;
            },
        }
    }

    fn changeDir(self: *Self, path: []const u8) void {
        if (std.mem.eql(u8, path, "/")) {
            self.cwd = self.root;
            return;
        }

        if (std.mem.eql(u8, path, "..")) {
            self.cwd = self.cwd.parent.?;
            return;
        }

        const dir = switch (self.cwd.data) {
            .file => |_| @panic("cwd is a not a directory"),
            .dir => |dir| dir,
        };

        self.cwd = dir.getEntry(path).?.value_ptr;
    }
};

fn part1(alloc: std.mem.Allocator, input: []u8) !i64 {
    var fs = try FileSystem.init(alloc);
    defer fs.deinit();

    var cmd_hist_iter = std.mem.splitSequence(u8, input, "$ ");
    while (cmd_hist_iter.next()) |cmd_hist| {
        var line_iter = std.mem.splitScalar(u8, cmd_hist, '\n');
        const cmd = line_iter.next().?;
        var cmd_iter = std.mem.splitScalar(u8, cmd, ' ');
        const cmd_name = cmd_iter.next().?;

        if (std.mem.eql(u8, cmd_name, "cd")) {
            const dir_name = cmd_iter.next().?;
            fs.changeDir(dir_name);
            continue;
        }

        if (std.mem.eql(u8, cmd_name, "ls")) {
            while (line_iter.next()) |output_line| {
                if (output_line.len == 0) {
                    continue;
                }

                var output_iter = std.mem.splitScalar(u8, output_line, ' ');
                const data = output_iter.next().?;
                const name = output_iter.next().?;

                if (std.mem.eql(u8, data, "dir")) {
                    try fs.insertDir(name);
                } else {
                    try fs.insertFile(name, try std.fmt.parseInt(usize, data, 10));
                }
            }
        }
    }

    return @intCast(fs.root.sumWithinLimit(100000));
}

fn part2(alloc: std.mem.Allocator, input: []u8) !i64 {
    var fs = try FileSystem.init(alloc);
    defer fs.deinit();

    var cmd_hist_iter = std.mem.splitSequence(u8, input, "$ ");
    while (cmd_hist_iter.next()) |cmd_hist| {
        var line_iter = std.mem.splitScalar(u8, cmd_hist, '\n');
        const cmd = line_iter.next().?;
        var cmd_iter = std.mem.splitScalar(u8, cmd, ' ');
        const cmd_name = cmd_iter.next().?;

        if (std.mem.eql(u8, cmd_name, "cd")) {
            const dir_name = cmd_iter.next().?;
            fs.changeDir(dir_name);
            continue;
        }

        if (std.mem.eql(u8, cmd_name, "ls")) {
            while (line_iter.next()) |output_line| {
                if (output_line.len == 0) {
                    continue;
                }

                var output_iter = std.mem.splitScalar(u8, output_line, ' ');
                const data = output_iter.next().?;
                const name = output_iter.next().?;

                if (std.mem.eql(u8, data, "dir")) {
                    try fs.insertDir(name);
                } else {
                    try fs.insertFile(name, try std.fmt.parseInt(usize, data, 10));
                }
            }
        }
    }

    const root_size = fs.root.size();
    const cur_free_size = 70000000 - root_size;
    const additional_free_size = 30000000 - cur_free_size;
    const dir_to_free_size = fs.root.smallestDirAboveLimit(additional_free_size);
    return @intCast(dir_to_free_size);
}
