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

    pub fn add_density(&mut self, x: usize, y: usize, ammount: f32) {
        let index = to_buffer(x, y);
        self.density[index] += ammount;
    }

    pub fn add_velocity(&mut self, x: usize, y: usize, ammount_x: f32, ammount_y: f32) {
        let index = to_buffer(x, y);
        self.vx[index] += ammount_x;
        self.vy[index] += ammount_y;
    }
}
