use model::{CellLocation, Direction, Grid, GridCell};

pub struct Model {
    grid: Grid,
    snakeHead: CellLocation,
    snakeTail: CellLocation,
    direction: Direction,
    length: usize,
    score: i32
}

impl Model {
    pub fn new(length: usize) -> Model {
        let mut grid = Grid::new(length);
        let startLocation = grid.center();

        // Set the snake's initial location on the grid
        if let Some(cell) = grid.get_cell(startLocation) {
            cell.change_cell(GridCell::SnakePart);
        } else {
            panic!("This shouldnae happen...");
        }

        // Add the first bit of food to the grid
        grid.add_food_random_cell();

        Model {
            grid: grid,
            length: length,
            snakeHead: startLocation,
            snakeTail: startLocation,
            direction: Direction::Right,
            score: 0
        }
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    //todo typedef GameOver = bool
    pub fn game_tick(&mut self) -> bool {
        let direction = self.direction;
        let moved_into: Option<GridCell> = self.grid.move_snake(direction);

        match moved_into {
            Some(GridCell::Food) => {
                self.score = self.score + 1;
                false
            }
            Some(_) => false,
            None => true
        }



    }
}
