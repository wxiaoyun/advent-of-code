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

fn digits(n: i64, buf: []u8) usize {
    var cur = n;
    var i: usize = 0;
    while (cur > 0) : (cur = @divTrunc(cur, 10)) {
        buf[i] = @truncate(@abs(@rem(cur, 10)));
        i += 1;
    }

    return i;
}

fn isValid1(n: i64, buf: []u8) bool {
    const ndigits = digits(n, buf);

    if (@mod(ndigits, 2) == 1) {
        return false;
    }

    const half_len = ndigits / 2;

    for (0..half_len) |i| {
        if (buf[i] != buf[half_len + i]) {
            return false;
        }
    }

    return true;
}

fn part1(_: std.mem.Allocator, input: []const u8) !i64 {
    var buf: [20]u8 = undefined;
    var ranges = std.mem.splitSequence(u8, input, ",");

    var result: i64 = 0;
    while (ranges.next()) |range| {
        if (range.len == 0) continue;
        var bounds = std.mem.splitSequence(u8, range, "-");
        const left = bounds.next() orelse @panic("Bad input");
        const right = bounds.next() orelse @panic("Bad input");
        const left_trimmed = std.mem.trim(u8, left, " \t\n\r");
        const right_trimmed = std.mem.trim(u8, right, " \t\n\r");
        const l = try std.fmt.parseInt(i64, left_trimmed, 10);
        const r = try std.fmt.parseInt(i64, right_trimmed, 10);

        var i = l;
        while (i <= r) : (i += 1) {
            if (isValid1(i, &buf)) {
                result += i;
            }
        }
    }

    return result;
}

fn isValid2(n: i64, buf: []u8) bool {
    const ndigits = digits(n, buf);

    var step: usize = 1;
    check_step: while (step <= ndigits / 2) : (step += 1) {
        if (ndigits % step != 0) continue;

        for (0..step) |i| {
            var j = i;
            while (j < ndigits) : (j += step) {
                if (buf[i] != buf[j]) continue :check_step;
            }
        }

        return true;
    }

    return false;
}

fn part2(_: std.mem.Allocator, input: []const u8) !i64 {
    var buf: [20]u8 = undefined;
    var ranges = std.mem.splitSequence(u8, input, ",");

    var result: i64 = 0;
    while (ranges.next()) |range| {
        if (range.len == 0) continue;
        var bounds = std.mem.splitSequence(u8, range, "-");
        const left = bounds.next() orelse @panic("Bad input");
        const right = bounds.next() orelse @panic("Bad input");
        const left_trimmed = std.mem.trim(u8, left, " \t\n\r");
        const right_trimmed = std.mem.trim(u8, right, " \t\n\r");
        const l = try std.fmt.parseInt(i64, left_trimmed, 10);
        const r = try std.fmt.parseInt(i64, right_trimmed, 10);

        var i = l;
        while (i <= r) : (i += 1) {
            if (isValid2(i, &buf)) {
                result += i;
            }
        }
    }

    return result;
}

const test_input =
    \\11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
;

test part1 {
    const alc = std.testing.allocator;
    try std.testing.expectEqual(1227775554, try part1(alc, test_input));
}

test part2 {
    const alc = std.testing.allocator;
    try std.testing.expectEqual(4174379265, try part2(alc, test_input));
}
