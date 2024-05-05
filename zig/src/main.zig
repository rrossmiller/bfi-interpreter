const std = @import("std");

pub fn main() !void {
    std.debug.print("{s}\n", .{std.os.argv});
    const fname = std.os.argv[1];
    var file = try std.fs.cwd().readFile(fname, .{});
    defer file.close();
}
