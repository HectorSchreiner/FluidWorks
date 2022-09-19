pub use fluid::*;
pub use minifb;
use minifb::*;
pub use renderer::*;
pub use utils::*;

mod fluid;
mod renderer;
mod utils;

pub fn main() {
    let mut renderer: Renderer = Renderer {
        buffer: vec![0; WIDTH * HEIGHT],
    };

    let mut fluids: Fluid;
    &fluids.new(2, 0.0, 0.0, 1.0);

    println!("{:?}", fluids);

    let mut window = Window::new("FluidWorks", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();
        (&mut renderer, &window);

        renderer.clear(Color::BLACK); //Clear screen

        renderer.rect(&Square::new(20, 20, (20, 20)), Color::GREEN);
    }
}
