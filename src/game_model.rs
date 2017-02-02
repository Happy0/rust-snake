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
    x: i32,
    y: i32
}

pub struct Arena {
        cells: Vec<Vec<ArenaCell>>,
        snakeHead : CellLocation,
        snakeTail : CellLocation,
        length: i32
}

fn get_arena_centre(arena_length: i32) -> CellLocation {
    let mid = arena_length / 2;
    CellLocation { x: mid, y: mid }
}

fn add_food_random_cell(arena: &mut Vec<Vec<ArenaCell>>) {
    let mut rng = thread_rng();

    // Halp... I don't know what i'm doing. I should probably read a rust tutorial or two
    // before i try to use it :P. Screw it, I'm going to bed...

    let flattenedCells = arena.iter().flatMap(|x| x);

    let emptyCells = flattenedCells.iter().filter(|cell| match cell {
        Empty => true,
        _ => false
    }).collect::<Vec<ArenaCell>>();

    let cell = rng.choose_mut(emptyCells);

    match cell {
        Some(c) => *c = ArenaCell::Food,
        None => println!("fdgdfgsdf")
    }

}

fn create_empty_cell_grid(length: i32) -> Vec<Vec<ArenaCell>> {
    (0 .. length).map(|_|
        (0 .. length).map(|_| ArenaCell::Empty ).collect()
    ).collect()
}

impl Arena {
    fn new(length: i32) -> Arena {
        let mut cells = create_empty_cell_grid(length);
        let startLocation = get_arena_centre(length);

        add_food_random_cell(&mut cells);

        Arena { cells : cells,
                snakeHead: startLocation,
                snakeTail: startLocation,
                length: length
            }
    }

}
