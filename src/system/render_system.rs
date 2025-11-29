use raylib::prelude::RaylibDrawHandle;

use crate::system::transform_system::{TransformSystem};


pub struct RenderSystem {
    transform_system : TransformSystem,
}

impl RenderSystem {
    pub fn new(ts : TransformSystem) -> Self {
        return RenderSystem { transform_system: (ts) }
    }

    /*
    This function will allow the render system to render the curret scene
     */
    pub fn render_frame(&self, draw_handler : RaylibDrawHandle) {
        
    }
}
