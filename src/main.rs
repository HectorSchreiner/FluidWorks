pub use fluid::*;
pub use minifb;
use minifb::*;
pub use renderer::*;
pub use utils::*;

mod draw;
mod fluid;
mod renderer;
pub mod simulation;
mod utils;

pub fn main() {
    let mut renderer: Renderer = Renderer {
        buffer: vec![0; WIDTH * HEIGHT],
    };

    let mut fluid = Fluid::new(0, 0.1, 0.0, 0.0);
    let mut window = Window::new("FluidWorks", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();
        //(&mut renderer, &window);

        renderer.clear(Color::WHITE);
        fluid.time_step();
        renderer.render_fluid(&fluid);
        renderer.add_stuff(&window, &mut fluid, 1000.0, 0.0, -20.0);
    }
}
