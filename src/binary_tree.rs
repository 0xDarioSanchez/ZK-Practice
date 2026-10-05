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
        Self { depth, leaves: Vec::new() }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn len(&self) -> usize {
        self.leaves.len()
    }
    
    
}