pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub type Grid = [[CellType; WIDTH]; HEIGHT];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CellType {
    Empty,
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
