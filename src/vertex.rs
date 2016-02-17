#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vertex {
    pub x: usize,
    pub y: usize
}

impl Vertex {
    pub fn new(x: usize, y: usize) -> Vertex {
        Vertex {
            x: x,
            y: y
        }
    }
}
