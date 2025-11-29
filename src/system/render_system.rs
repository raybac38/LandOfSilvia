use crate::system::transform_system::{TransformSystem};


pub struct RenderSystem {
    transform_system : TransformSystem,
}

impl RenderSystem {
    pub fn new(ts : TransformSystem) -> Self {
        return RenderSystem { transform_system: (ts) }
    }
}
