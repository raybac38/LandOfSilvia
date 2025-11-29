use crate::{core::window::Window, system::{render_system::RenderSystem, transform_system::TransformSystem}};

mod core;
mod component;
mod system;
mod utils;

fn main() {
    let transform_system = TransformSystem::new();
    let render_system = RenderSystem::new(transform_system);

    let mut window = Window::new(800, 600, "Land Of Silvia", render_system);

    while window.is_open() {
        window.render_frame();
    }
}