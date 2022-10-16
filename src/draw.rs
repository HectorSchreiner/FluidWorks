use crate::renderer;
use crate::renderer::*;
use crate::simulation::*;
use crate::utils::*;
use crate::Fluid;
use crate::Window;
use minifb::*;

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
        //iterer igennem density array size (N*N)
        for y in 0..N {
            for x in 0..N {
                let d = fluid.density[to_buffer(x, y)];
                let color: (u8, u8, u8) = (d as u8 % 255, d as u8 % 255, d as u8 % 255);

                self.rect(
                    &Square::new(
                        SCALE as _,
                        SCALE as _,
                        (x as u32 * SCALE as u32, y as u32 * SCALE as u32),
                    ),
                    color,
                );
            }
        }
    }

    pub fn add_stuff(
        &mut self,
        window: &Window,
        fluid: &mut Fluid,
        density_ammount: f32,
        v_amm_x: f32,
        v_amm_y: f32,
    ) {
        if window.get_mouse_down(MouseButton::Left) {
            window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                fluid.add_density(
                    mouse.0 as usize / SCALE,
                    mouse.1 as usize / SCALE,
                    density_ammount,
                );
                fluid.add_velocity(
                    mouse.0 as usize / SCALE,
                    mouse.1 as usize / SCALE,
                    v_amm_x,
                    v_amm_y,
                )
            });
        }
    }
}

pub fn if_true(b: bool, x: usize, y: usize) {
    if b {
        println!("x: {} y: {}", x, y);
    }
}
