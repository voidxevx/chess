
pub const Location = extern struct {
    x_pos: u8,
    y_pos: u8,

    pub fn init(x: u8, y: u8) Location {
        return .{
            .x_pos = x,
            .y_pos = y,
        };
    }
};