use model::{CellLocation, Direction, Grid, GridCell};

pub type GameOver = bool;

pub struct Model {
    grid: Grid,
    direction: Direction,
    length: usize,
    score: i32
}

impl Model {
    pub fn new(length: usize) -> Model {
        let mut grid = Grid::new(length);

        // Add the first bit of food to the grid
        grid.add_food_random_cell();

        Model {
            grid: grid,
            length: length,
            direction: Direction::Right,
            score: 0
        }
    }

    pub fn change_snake_direction(&mut self, new_direction: Direction) {
        if (!self.direction.is_opposite_direction(new_direction)) {
            self.direction = new_direction;
        }
    }

    pub fn game_tick(&mut self) -> GameOver {
        let direction = self.direction;
        let moved_into: Option<GridCell> = self.grid.move_snake(direction);

        // If the snake moves into the wall or itself it is game over
        match moved_into {
            Some(GridCell::Food) => {
                self.score = self.score + 1;
                self.grid.grow_snake();
                self.grid.add_food_random_cell();
                false
            }
            Some(GridCell::Empty) => false,
            Some(GridCell::SnakePart) => true,
            None => true
        }



    }
}
