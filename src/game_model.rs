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
        SnakePart(direction: Direction)
    }

    // A cell locaiton in the arena
    pub struct CellLocation {
        x: Int,
        y: Int
    }

    pub struct Arena {
            cells: Vec<Vec<ArenaCell>>,
            snakeHead : CellLocation,
            snakeTail : CellLocation,
            food: CellLocation,
            length: Int
    }

    fn get_arena_centre(arena_length: Int) : CellLocation {
        ArenaCell(arena_length / 2, arena_length / 2)
    }

    fn create_empty_cell_grid(length: Int) : Vec<Vec<ArenaCell>> {
        (0 .. length).map(|_|
            (0 .. length).map(|_| Empty).collect()
        ).collect()
    }

    impl Arena {
        fn new(length: Int) -> Arena {
            let cells = create_empty_cell_grid(length)


        }

    }

}
