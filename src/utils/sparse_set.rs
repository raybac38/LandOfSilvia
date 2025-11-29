pub struct SparseSet<T> {
    dense: Vec<T>,
    sparse: Vec<usize>,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            dense: Vec::new(),
            sparse: Vec::new(),
        }
    }

    fn insert(&self, element: T, index: usize) {
        let index_dense = self.dense.len();
    }
}
