use crate::{component::transform_component::TransformComponent, utils::sparse_set::SparseSet};

pub struct TransformSystem {
    transform_components : SparseSet<TransformComponent>,
}

impl TransformSystem {
    
    pub fn new() -> Self {
        TransformSystem { transform_components: SparseSet::new() }
    }
}

