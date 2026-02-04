#[derive(Clone, Copy, PartialEq)]
pub enum Stone {
    Empty,
    Black,
    White,
}
pub struct Board {
    size: usize,
    grid: Vec<Vec<Stone>>,
}
impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            size,
            grid: vec![vec![Stone::Empty; size]; size],
        }
    }
}