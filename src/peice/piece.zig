const std = @import("core").std;
const Location = @import("location.zig").Location;

/// A board piece -- stores
pub const Piece = struct {
    /// the position of the piece
    position: Location,
    /// the character displayed on the board when rendered
    displayChar: u8,
    /// the team of the piece
    team: u1,

    pub fn init(x: u8, y: u8, char: u8, team: u1) Piece {
        return .{
            .position = Location.init(x, y),
            .displayChar = char,
            .team = team
        };
    }

};