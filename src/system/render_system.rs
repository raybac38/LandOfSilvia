use raylib::{models::Model, prelude::RaylibDrawHandle};

use crate::{system::transform_system::TransformSystem, utils::sparse_set::SparseSet};


pub struct RenderSystem {
    transform_system : TransformSystem,
    models : SparseSet<Model>
}

impl RenderSystem {
    pub fn new(transform_system : TransformSystem) -> Self {
        return RenderSystem { 
            transform_system: (transform_system),
            models : SparseSet::new()
        }
    }

    /*
    This function will allow the render system to render the curret scene
     */
    pub fn render_frame(&self, draw_handler : RaylibDrawHandle) {
        
    }
}
