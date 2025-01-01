const std = @import("std");
const ArrayList = std.ArrayList;

const SqliteDB = struct {
    filename: []const u8,
    data: ArrayList,
    fn start(self: *SqliteDB) !void {
        std.debug.print("Inicializando o {}\n", .{&self.filename});
    }
    fn size(self: *SqliteDB) !void {
        std.debug.print("{} elementos", .{self.data.len()});
    }
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var list = ArrayList(u64).init(allocator);
    defer list.deinit();

    const numbers:[10]u64 = [10]u64{1,2,3,4,5,6,7,8,9,10};
    for (numbers) |number|{
        try list.append(number);
    }

    std.debug.print("Capacidade {}\n", .{list.capacity});
    

}
