pub const N: usize = 64;
pub const PI: f32 = 3.14159265;
pub const ITER: i32 = 4;
pub const SCALE: usize = 16;
pub const WIDTH: usize = N * SCALE;
pub const HEIGHT: usize = N * SCALE;

#[derive(Clone, Copy)]
pub enum Color {
    WHITE = 0xffffff,
    BLACK = 0x000000,
    RED = 0xff0000,
    GREEN = 0x00ff00,
    BLUE = 0x0000ff,
}
pub fn to_screen_buffer(x: usize, y: usize) -> usize {
    x + y * WIDTH
}

pub fn to_buffer(x: usize, y: usize) -> usize {
    x + y * N
}

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn print_vals(x: usize, y: usize) {
    println!("x: {} y: {}", x, y);
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
