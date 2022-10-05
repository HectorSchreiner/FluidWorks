use crate::renderer::*;
use crate::simulation::*;
use crate::utils::*;
use crate::Fluid;

impl Fluid {
    pub fn time_step(&mut self) {
        //let N = self.size as usize;
        let viscosity = self.viscosity;
        let diffusion = self.diffusion;
        let dt = &mut self.dt;
        let vx = &mut self.vx;
        let vy = &mut self.vy;
        let vx0 = &mut self.vx0;
        let vy0 = &mut self.vy0;
        let s = &mut self.s;
        let density = &mut self.density;

        diffuse(1, vx0, vx, viscosity, dt);
        diffuse(2, vy0, vy, viscosity, dt);

        project(vx0, vy0, vx, vy);

        advect(1, vx, vx0, vx0, vy0, dt);
        advect(2, vy, vy0, vx0, vy0, dt);

        project(vx, vy, vx0, vy0);

        diffuse(0, s, density, diffusion, dt);
        advect(0, density, s, vx, vy, dt);
    }
}

impl Renderer {
    pub fn render_fluid(&mut self, fluid: &Fluid) {
        for i in 0..N {
            for j in 0..N {
                let x = i * SCALE;
                let y = j * SCALE;
                let d = fluid.density[to_buffer(i, j)];
                println!("{:?}", fluid.density[1000]);
                let color = ((d % 255.0) as u8, (d % 255.0) as u8, (d % 255.0) as u8);
                self.rect(
                    &Square::new(SCALE as _, SCALE as _, (x as u32, y as u32)),
                    color,
                );
            }
        }
    }

    pub fn density_adder(&mut self) {}
}
