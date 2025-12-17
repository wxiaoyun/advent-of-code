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
    return try solve(alloc, input, 2);
}

fn part2(alloc: std.mem.Allocator, input: []const u8) !i64 {
    return try solve(alloc, input, 12);
}

fn includeDigit(digit: i8, buf: []i8) void {
    var d: i8 = digit;
    for (buf) |*slot| {
        if (d < slot.*) return;
        const temp = slot.*;
        slot.* = d;
        d = temp;
    }
}

fn buildNum(builder: []i8) i64 {
    var acc: i64 = 0;
    var i: usize = 0;
    while (i < builder.len) : (i += 1) {
        acc = acc * 10 + builder[i];
    }
    return acc;
}

fn largestNDigitNumber(n: usize, digits: []const i8, buf: []i8) i64 {
    if (buf.len != n) @panic("Buffer length must be equal to n");
    @memset(buf, -1);

    var largest: i64 = 0;
    var filled_count: usize = 0;

    var j: usize = digits.len;
    while (j > 0) {
        j -= 1;
        const d = digits[j];

        if (filled_count < n) {
            buf[n - 1 - filled_count] = d;
            filled_count += 1;
        } else {
            includeDigit(d, buf);
        }

        if (filled_count == n) {
            largest = @max(largest, buildNum(buf));
        }
    }
    return largest;
}

fn solve(alloc: std.mem.Allocator, input: []const u8, comptime n: usize) !i64 {
    var buf: [n]i8 = undefined;
    var digits_buf: ?[]i8 = null;
    var sum: i64 = 0;

    var rows = std.mem.splitScalar(u8, input, '\n');
    while (rows.next()) |row| {
        if (row.len == 0) {
            continue;
        }

        if (digits_buf == null) {
            digits_buf = try alloc.alloc(i8, row.len);
        }

        if (digits_buf) |dbuf| {
            for (row, 0..) |ch, i| {
                const ch_i8: i8 = @intCast(ch);
                dbuf[i] = ch_i8 - '0';
            }
            const largest_num = largestNDigitNumber(@as(usize, n), dbuf, &buf);
            sum += largest_num;
        } else {
            @panic("Uninitialised digits buffer");
        }
    }

    if (digits_buf) |dbuf| {
        alloc.free(dbuf);
    }

    return sum;
}

const test_input =
    \\987654321111111
    \\811111111111119
    \\234234234234278
    \\818181911112111
;
