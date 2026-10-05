#[derive(Debug, Clone)]
pub struct BinaryTree {
    depth: usize,
    leaves: Vec<Vec<u8>>,
}

impl BinaryTree {
    pub fn new(depth: usize) -> Self {
        assert!(
            depth < usize::BITS as usize,
            "depth must leave room for a 2^depth capacity"
        );
        let capacity = 1usize << depth;
        let mut leaves = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            leaves.push(vec![0u8]);
        }
        Self { depth, leaves }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn len(&self) -> usize {
        self.leaves.len()
    }
    
    
}