#[derive(Debug)]
pub struct CFGEdge {
    origin: usize,
    dest: usize,
}

impl CFGEdge {
    pub fn new(origin: usize, dest: usize) -> Self {
        Self { origin, dest }
    }

    pub fn get_origin(&self) -> usize {
        self.origin
    }

    pub fn get_dest(&self) -> usize {
        self.dest
    }
}
