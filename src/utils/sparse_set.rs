#[allow(dead_code)]

pub struct SparseSet<T> {
    dense: Vec<Option<T>>,
    sparse: Vec<Option<usize>>,
    dense_current_capacity : usize
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            dense: Vec::new(),
            sparse: Vec::new(),
            dense_current_capacity: 0
        }
    }

    pub fn insert(& mut self, element: T, index: usize) {
        let dense_index = self.find_free_place_in_dense();
        self.dense[dense_index] = Some(element);

        if self.sparse.len() <= index {
            self.sparse.resize(index + 1, None);
        }
        self.sparse[index] = Some(dense_index);
    }
    
    pub fn get(&self, index : usize) -> Option<&T> {
        let dense_index = self.sparse.get(index)?.as_ref()?;
        self.dense.get(*dense_index)?.as_ref()
    }

    pub fn iter(&self, f : impl Fn(&T)){
        for i in &self.dense {
            match i {
                None => (),
                Some(k) => f(k)
            }
        }
    }

    fn find_free_place_in_dense(& mut self) -> usize {
        for i in 0..self.dense.len(){
            if self.dense[i].is_none() {
                return i
            }
        }
        let index = self.dense.len();
        self.dense.push(None);
        index
    }



    
}


