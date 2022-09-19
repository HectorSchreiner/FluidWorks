use crate::utils::*;

#[derive(Debug)]
pub struct Fluid {
    size: i32,
    dt: f32,
    diffusion: f32,
    viscosity: f32,

    s: Vec<f32>,
    density: Vec<f32>,

    Vx: Vec<f32>,
    Vy: Vec<f32>,

    // Tidligere hastighed
    Vx0: Vec<f32>,
    Vy0: Vec<f32>,
}

impl Fluid {
    pub fn new(size: i32, dt: f32, diffusion: f32, viscosity: f32) -> Fluid {
        Fluid {
            size: size,
            dt: dt,
            diffusion: diffusion,
            viscosity: viscosity,
            s: Vec::with_capacity(N * N),
            density: Vec::with_capacity(N * N),
            Vx: Vec::with_capacity(N * N),
            Vy: Vec::with_capacity(N * N),
            Vx0: Vec::with_capacity(N * N),
            Vy0: Vec::with_capacity(N * N),
        }
    }
}

//         self.size = size;
//         self.dt = dt;
//         self.diffusion = diffusion;
//         self.viscosity = viscosity;
//         self.density = !vec[f32; N * N];
//         self.s = !vec[f32; N * N];

//         self.Vx = !vec[f32; N * N];
//         self.Vy = [f32; N * N];

//         self.Vx0 = !vec[f32; N * N];
//         self.Vy0 = !vec[f32; N * N];
