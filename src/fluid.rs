use crate::utils::*;

#[derive(Debug)]
pub struct Fluid {
    pub size: i32,
    pub dt: f32,
    pub diffusion: f32,
    pub viscosity: f32,

    pub s: Vec<f32>,
    pub density: Vec<f32>,

    pub vx: Vec<f32>,
    pub vy: Vec<f32>,

    // Tidligere hastighed
    pub vx0: Vec<f32>,
    pub vy0: Vec<f32>,
}

impl Fluid {
    pub fn new(size: i32, dt: f32, diffusion: f32, viscosity: f32) -> Fluid {
        Fluid {
            size,
            dt,
            diffusion,
            viscosity,
            s: Vec::with_capacity(N * N),
            density: Vec::with_capacity(N * N),
            vx: Vec::with_capacity(N * N),
            vy: Vec::with_capacity(N * N),
            vx0: Vec::with_capacity(N * N),
            vy0: Vec::with_capacity(N * N),
        }
    }

    pub fn add_density(&mut self, x: i32, y: i32, ammount: f32) {
        let index = to_buffer(x, y);
        self.density[index] += ammount;
    }

    pub fn add_velocity(&mut self, x: i32, y: i32, ammountX: f32, ammountY: f32) {
        let index = to_buffer(x, y);
        self.vx[index] += ammountX;
        self.vy[index] += ammountY;
    }
}
