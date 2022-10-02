use crate::renderer::*;
use crate::simulation::*;
use crate::utils::*;
use crate::Fluid;

impl Fluid {
    pub fn time_step(&mut self) {
        //let N = self.size as usize;
        let viscosity = self.viscosity;
        let diffusion = self.diffusion;
        let dt = self.dt;
        let vx = &self.vx;
        let vy = &self.vy;
        let vx0 = &self.vx0;
        let vy0 = &self.vy0;
        let s = &self.s;
        let density = &self.density;

        diffuse(1, vx0.to_vec(), vx.to_vec(), viscosity, dt);
        diffuse(2, vy0.to_vec(), vy.to_vec(), viscosity, dt);

        project(vx0.to_vec(), vy0.to_vec(), vx.to_vec(), vy.to_vec());

        //advect(1, Vx, Vx0, Vx0, Vy0, dt);
        //advect(2, Vy, Vy0, Vx0, Vy0, dt);

        project(vx.to_vec(), vy.to_vec(), vx0.to_vec(), vy0.to_vec());

        diffuse(0, s.to_vec(), density.to_vec(), diffusion, dt);
        //advect(0, density, s, vx, vy, dt);
    }
}

impl Renderer {
    pub fn render_fluid(&mut self, fluid: &Fluid) {
        let mut color;
        for i in 0..N {
            for j in 0..N {
                let d = fluid.density[to_buffer(i, j)];
                if d > 5.0 {
                    color = Color::WHITE;
                } else {
                    color = Color::BLACK;
                }
                self.rect(&Square::new(1, 1, (i as u32, j as u32)), color);
            }
        }
    }
}
