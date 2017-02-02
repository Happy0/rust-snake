use rand::distributions::{IndependentSample, Range};

mod game_model {

    // A direction that the snake head can travel in
    pub enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    // A cell in the arena is either empty, occupied by a snake part (with a direction the
    // tail should continue in when it hits it), or contains a piece of food.
    pub enum ArenaCell {
        Empty,
        Food,
        SnakePart {direction: Direction }
    }

    // A cell locaiton in the arena
    pub struct CellLocation {
        x: i32,
        y: i32
    }

    pub struct Arena {
            cells: Vec<Vec<ArenaCell>>,
            snakeHead : CellLocation,
            snakeTail : CellLocation,
            food: CellLocation,
            length: i32
    }

    fn get_arena_centre(arena_length: i32) -> CellLocation {
        let mid = arena_length / 2;
        CellLocation { x: mid, y: mid }
    }

    fn generate_random_food_location(arena_length: i32) -> CellLocation {
        let mut rng = Range::new(0, arena_length);
        let x: i32 = rng.thread_rng();
        let y: i32 = rng.thread_rng();

        CellLocation { x: x, y: y }
    }

    fn create_empty_cell_grid(length: i32) -> Vec<Vec<ArenaCell>> {
        (0 .. length).map(|_|
            (0 .. length).map(|_| ArenaCell::Empty ).collect()
        ).collect()
    }

    impl Arena {
        fn new(length: i32) -> Arena {
            let cells = create_empty_cell_grid(length);
            let startLocation = get_arena_centre(length);

            let foodLocation = generate_random_food_location(length);

            Arena { cells : cells,
                    snakeHead: startLocation,
                    snakeTail: startLocation,
                    food: foodLocation,
                    length: length
                }
        }

    }

}
