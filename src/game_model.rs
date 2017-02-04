use rand::{thread_rng, Rng};

// A direction that the snake head can travel in
#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

// A cell in the arena is either empty, occupied by a snake part (with a direction the
// tail should continue in when it hits it), or contains a piece of food.
#[derive(Clone)]
pub enum ArenaCell {
    Empty,
    Food,
    SnakePart {direction: Direction }
}

// A cell locaiton in the arena
#[derive(Clone, Copy)]
pub struct CellLocation {
    x: usize,
    y: usize
}

pub struct Arena {
        cells: Vec<Vec<ArenaCell>>,
        snakeHead : CellLocation,
        snakeTail : CellLocation,
        length: usize
}

fn get_arena_centre(arena_length: usize) -> CellLocation {
    let mid = arena_length / 2;
    CellLocation { x: mid, y: mid }
}

fn add_food_random_cell(arena: &mut Vec<Vec<ArenaCell>>) {

}

fn create_empty_cell_grid(length: usize) -> Vec<Vec<ArenaCell>> {
    (0 .. length).map(|_|
        (0 .. length).map(|_| ArenaCell::Empty ).collect()
    ).collect()
}

impl Arena {
    fn new(length: usize) -> Arena {
        let mut cells = create_empty_cell_grid(length);
        let startLocation = get_arena_centre(length);

        let CellLocation { x, y } = startLocation;

        if let Some(startCell) = cells.get_mut(x).and_then(|row| row.get_mut(y)) {
            *startCell = ArenaCell::SnakePart { direction:Direction::Right }
        }

        //add_food_random_cell(&cells);

        Arena { cells : cells,
                snakeHead: startLocation,
                snakeTail: startLocation,
                length: length
            }
    }

}
