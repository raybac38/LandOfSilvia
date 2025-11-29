use crate::system::{render_system::RenderSystem, transform_system::TransformSystem};


mod component;
mod system;
mod utils;

fn main() {
    let transform_system = TransformSystem::new();
    let render_system = RenderSystem::new(transform_system);

}