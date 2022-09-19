pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;
pub const PI: f32 = 3.1415;
pub const N: usize = 600;

#[derive(Clone, Copy)]
pub enum Color {
    WHITE = 0xffffff,
    BLACK = 0x000000,
    RED = 0xff0000,
    GREEN = 0x00ff00,
    BLUE = 0x0000ff,
    DARK_GREY = 0x202020,
}

pub struct Position {
    pub x: u32,
    pub y: u32,
}
impl From<(u32, u32)> for Position {
    fn from(position: (u32, u32)) -> Self {
        Self {
            x: position.0,
            y: position.1,
        }
    }
}