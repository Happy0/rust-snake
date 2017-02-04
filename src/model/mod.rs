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

// A cell in the arena is either empty, occupied by a snake part (with a direction the
// tail should continue in when it hits it), or contains a piece of food.
#[derive(Clone)]
pub enum ArenaCell {
    Empty,
    Food,
    SnakePart(Direction),
}

// A cell locaiton in the arena
#[derive(Clone, Copy)]
pub struct CellLocation {
    x: usize,
    y: usize,
}
