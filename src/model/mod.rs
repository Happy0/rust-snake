use rand::{thread_rng, Rng};

pub mod model;
mod grid;

pub use self::grid::Grid;
pub use self::model::Model;

// A direction that the snake head can travel in
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn is_opposite_direction(&self, other: Direction) -> bool {
        match *self {
            Direction::Up => other == Direction::Down,
            Direction::Down => other == Direction::Up,
            Direction::Left => other == Direction::Right,
            Direction::Right => other == Direction::Left
        }
    }
}

// A cell in the grid is either empty, occupied by a snake part (with a direction the
// tail should continue in when it hits it), or contains a piece of food.
#[derive(Clone)]
pub enum GridCell {
    Empty,
    Food,
    SnakePart,
}

impl GridCell {
    pub fn change_cell(&mut self, value: GridCell) {
        *self = value;
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            GridCell::Empty => true,
            _ => false,
        }
    }
}

// A cell locaiton in the arena
#[derive(Clone, Copy)]
pub struct CellLocation {
    x: usize,
    y: usize,
}
