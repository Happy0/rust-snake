use model::{ArenaCell, CellLocation, Direction, Grid};

pub struct Arena {
    grid: Grid,
    snakeHead: CellLocation,
    snakeTail: CellLocation,
    length: usize,
}

impl Arena {
    fn new(length: usize) -> Arena {
        let mut grid = Grid::new(length);
        let startLocation = grid.center();

        grid.modify_arena_cell(startLocation, ArenaCell::SnakePart(Direction::Right));

        //add_food_random_cell(&cells);

        Arena {
            grid: grid,
            snakeHead: startLocation,
            snakeTail: startLocation,
            length: length,
        }
    }
}
