use crate::utils::{Color, Position, HEIGHT, PI, WIDTH};
use line_drawing::Bresenham;

pub struct Renderer {
    pub buffer: Vec<u32>,
}
impl Renderer {
    pub fn draw_pixel(&mut self, position: Position, color: Color) {
        if position.x < WIDTH as u32
            && position.x > 0
            && position.y < HEIGHT as u32
            && position.y > 0
        {
            self.buffer[(position.x + position.y * WIDTH as u32) as usize] = color as _;
        }
    }

    pub fn rect(&mut self, square: &Square, color: Color) {
        let pos_y = square.position.y;
        let pos_x = square.position.x;

        for y in pos_y..square.height + pos_y {
            for x in pos_x..square.lenght + pos_x {
                self.buffer[(y * WIDTH as u32 + x) as usize] = color as _;
            }
        }
    }

    pub fn line(&mut self, line: &Line, color: Color) {
        for (x, y) in Bresenham::new(
            (line.pos_1.x as i32, line.pos_1.y as i32),
            (line.pos_2.x as i32, line.pos_2.y as i32),
        ) {
            self.draw_pixel(
                Position {
                    x: x as u32,
                    y: y as u32,
                },
                color,
            );
        }
    }

    pub fn clear(&mut self, color: Color) {
        for iter in 0..HEIGHT * WIDTH {
            self.buffer[iter] = color as _;
        }
    }
}
pub struct Square {
    pub lenght: u32,
    pub height: u32,
    pub position: Position,
}
impl Square {
    pub fn new(lenght: u32, height: u32, position: impl Into<Position>) -> Square {
        Square {
            lenght,
            height,
            position: position.into(),
        }
    }
}

pub struct Line {
    pub pos_1: Position,
    pub pos_2: Position,
}
impl Line {
    pub fn new(pos_1: impl Into<Position>, pos_2: impl Into<Position>) -> Line {
        Line {
            pos_1: pos_1.into(),
            pos_2: pos_2.into(),
        }
    }

    pub fn length_of_line(&self) -> f32 {
        let dx: f32 = (self.pos_1.x as f32 - self.pos_2.x as f32).abs();
        let dy: f32 = (self.pos_1.y as f32 - self.pos_2.y as f32).abs();
        return (dx * dx + dy * dy).powf(0.5);
    }
}
