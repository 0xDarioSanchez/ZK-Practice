#[derive(Debug, Clone)]
pub struct BinaryTree {
    depth: usize,
    leaves: Vec<Vec<u8>>,
}

impl BinaryTree {
    pub fn new(depth: usize) -> Self {
        Self { depth, leaves: vec![] }
    }
}