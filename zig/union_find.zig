const std = @import("std");

parent: []usize,
rank: []usize,
components: usize,

const Self = @This();

pub fn init(alloc: std.mem.Allocator, size: usize) !Self {
    var this: Self = undefined;
    this.parent = try alloc.alloc(usize, size);
    this.rank = try alloc.alloc(usize, size);

    for (0..size) |i| {
        this.parent[i] = i;
    }
    @memset(this.rank, 1);
    this.components = size;

    return this;
}

pub fn deinit(self: *Self, alloc: std.mem.Allocator) void {
    alloc.free(self.parent);
    alloc.free(self.rank);
}

pub fn find(self: *Self, n: usize) usize {
    const np = self.parent[n];
    if (n == np) {
        return n;
    }

    const npp = self.find(np);
    self.parent[n] = npp;
    return npp;
}

pub fn @"union"(self: *Self, a: usize, b: usize) void {
    const ap = self.find(a);
    const bp = self.find(b);
    if (ap == bp) {
        return;
    }

    const apr = self.rank[ap];
    const bpr = self.rank[bp];
    if (apr > bpr) {
        self.parent[bp] = ap;
        self.rank[ap] += bpr;
    } else {
        self.parent[ap] = bp;
        self.rank[bp] += apr;
    }
    self.components -= 1;
}

pub fn isConnected(self: *const Self) bool {
    return self.components == 1;
}
