use rand::{thread_rng, Rng};

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

pub struct Arena {
    cells: Vec<Vec<ArenaCell>>,
    snakeHead: CellLocation,
    snakeTail: CellLocation,
    length: usize,
}

fn get_arena_centre(arena_length: usize) -> CellLocation {
    let mid = arena_length / 2;
    CellLocation { x: mid, y: mid }
}

fn add_food_random_cell(arena: &mut Vec<Vec<ArenaCell>>) {}

fn modify_arena_cell(arena: &mut Vec<Vec<ArenaCell>>, location: CellLocation, value: ArenaCell) {
    let CellLocation { x, y } = location;

    if let Some(startCell) = arena.get_mut(x).and_then(|row| row.get_mut(y)) {
        *startCell = value;
    } else {
        panic!("eeeeee");
    }
}

fn create_empty_cell_grid(length: usize) -> Vec<Vec<ArenaCell>> {
    vec![vec![ArenaCell::Empty; length]; length]
}

impl Arena {
    fn new(length: usize) -> Arena {
        let mut arena = create_empty_cell_grid(length);
        let startLocation = get_arena_centre(length);

        modify_arena_cell(&mut arena,
                          startLocation,
                          ArenaCell::SnakePart(Direction::Right));

        //add_food_random_cell(&cells);

        Arena {
            cells: arena,
            snakeHead: startLocation,
            snakeTail: startLocation,
            length: length,
        }
    }
}
