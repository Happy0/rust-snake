use rand::{thread_rng, Rng};

pub mod arena;
mod grid;

pub use self::grid::Grid;

// A direction that the snake head can travel in
#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// A cell in the grid is either empty, occupied by a snake part (with a direction the
// tail should continue in when it hits it), or contains a piece of food.
#[derive(Clone)]
pub enum GridCell {
    Empty,
    Food,
    SnakePart(Direction),
}

impl GridCell {
    pub fn change_cell(&mut self, value: GridCell) {
        *self = value;
    }
}

// A cell locaiton in the arena
#[derive(Clone, Copy)]
pub struct CellLocation {
    x: usize,
    y: usize,
}
