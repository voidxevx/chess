const std = @import("core").std;
const Piece = @import("../peice/piece.zig").Piece;

const boardAllocator = std.heap.page_allocator;
const BOARD_SIZE = 8;

pub const Board = struct {
    pieces: std.ArrayList(std.ArrayList(?Piece)),

    fn init() !Board {
        var grid: std.ArrayList(std.ArrayList(?Piece)) = .empty;
        for (0..BOARD_SIZE) |_| {
            var page: std.ArrayList(?Piece) = .empty;
            for (0..BOARD_SIZE) |_| {
                try page.append(boardAllocator, null);
            }
            try grid.append(boardAllocator,page);
        }

        return .{
            .pieces = grid,
        };
    }

    fn deinit(self: *Board) void {
        defer self.pieces.deinit(boardAllocator);
        for (self.pieces.items) |*page| {
            page.*.deinit(boardAllocator);
        }
    }
};

var StaticBoard: ?Board = null;

pub export fn init_board() bool {
    if (StaticBoard == null) {
        StaticBoard = Board.init() catch {
            std.debug.print("\x1b[31m[ERROR]\x1b[0m Unable to initialize board", .{});
            return false;
        };

    } else {
        std.debug.print("\x1b[31m[WARNING]\x1b[0m Board already initialized.\n", .{});
    }

    return true;
}

pub export fn deinit_board() void {
    if (StaticBoard != null) {
        StaticBoard.?.deinit();
        StaticBoard = null;
    } else {
        std.debug.print("\x1b[31m[WARNING]\x1b[0m Board not initialized.\n", .{});
    }
}