use std::vec;

use crate::utils::*;

#[derive(Debug)]
pub struct Fluid {
    pub size: i32,
    pub dt: f32,
    pub diffusion: f32,
    pub viscosity: f32,

    pub s: Box<[f32; N * N]>,
    pub density: Box<[f32; N * N]>,

    pub vx: Box<[f32; N * N]>,
    pub vy: Box<[f32; N * N]>,

    pub vx0: Box<[f32; N * N]>,
    pub vy0: Box<[f32; N * N]>,
}

impl Fluid {
    pub fn new(size: i32, dt: f32, diffusion: f32, viscosity: f32) -> Fluid {
        Fluid {
            size,
            dt,
            diffusion,
            viscosity,
            s: Box::new([0.0; N * N]),
            density: Box::new([0.0; N * N]),
            vx: Box::new([0.0; N * N]),
            vy: Box::new([0.0; N * N]),
            vx0: Box::new([0.0; N * N]),
            vy0: Box::new([0.0; N * N]),
        }
    }

    pub fn add_density(&mut self, x: usize, y: usize, ammount: f32) {
        self.density[to_buffer(x, y)] += ammount;
    }

    pub fn add_velocity(&mut self, x: usize, y: usize, ammount_x: f32, ammount_y: f32) {
        let index = to_buffer(x, y);
        self.vx[index] += ammount_x;
        self.vy[index] += ammount_y;
    }
}
