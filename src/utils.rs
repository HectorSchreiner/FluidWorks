pub const N: usize = 252;
pub const WIDTH: usize = N;
pub const HEIGHT: usize = N;
pub const PI: f32 = 3.14159265;
pub const ITER: i32 = 4;

#[derive(Clone, Copy)]
pub enum Color {
    WHITE = 0xffffff,
    BLACK = 0x000000,
    RED = 0xff0000,
    GREEN = 0x00ff00,
    BLUE = 0x0000ff,
}

pub fn to_buffer(x: usize, y: usize) -> usize {
    x + y * N
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
