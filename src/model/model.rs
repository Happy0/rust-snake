use model::{CellLocation, Direction, Grid, GridCell};

pub struct Model {
    grid: Grid,
    snakeHead: CellLocation,
    snakeTail: CellLocation,
    direction: Direction,
    length: usize,
}

impl Model {
    fn new(length: usize) -> Model {
        let mut grid = Grid::new(length);
        let startLocation = grid.center();

        // Set the snake's initial location on the grid
        if let Some(cell) = grid.get_cell_if_in_range(startLocation) {
            cell.change_cell(GridCell::SnakePart);
        } else {
            panic!("This shouldnae happen...");
        }

        // Add the first bit of food to the grid
        grid.add_food_random_cell();

        //add_food_random_cell(&cells);

        Model {
            grid: grid,
            snakeHead: startLocation,
            snakeTail: startLocation,
            direction: Direction::Right,
            length: length,
        }
    }
}
