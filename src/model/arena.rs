use model::{CellLocation, Direction, Grid, GridCell};

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

        // Set the snake's initial location on the grid
        if let Some(cell) = grid.get_cell_if_in_range(startLocation) {
            cell.change_cell(GridCell::SnakePart(Direction::Right));
        } else {
            panic!("This shouldnae happen...");
        }

        // Add the first bit of food to the grid
        grid.add_food_random_cell();

        //add_food_random_cell(&cells);

        Arena {
            grid: grid,
            snakeHead: startLocation,
            snakeTail: startLocation,
            length: length,
        }
    }
}
