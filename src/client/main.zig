const std = @import("core").std;

const MainErrors = error {
    MainLoopError,
};

extern "C" fn main_loop(local: bool) bool;

const clientAllocator = std.heap.page_allocator;

pub fn main() !void {

    var local = false;
    {
        const args = try std.process.argsAlloc(clientAllocator);
        defer std.process.argsFree(clientAllocator, args);
        for (args) |arg| {
            if (std.mem.eql(u8, arg, "local"))
                local = true;
        }
    }



    if (!main_loop(local)) {
        return MainErrors.MainLoopError;
    }
}