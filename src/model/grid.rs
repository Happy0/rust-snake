use model::{CellLocation, Direction, GridCell};

use std::collections::VecDeque;

use rand::Rng;

// Question for Sam...
extern crate rand;

pub struct Grid {
    cells: Vec<Vec<GridCell>>,
    snake_locations: VecDeque<CellLocation>,
}

impl Grid {
    pub fn new(length: usize) -> Grid {
        let mut grid = Grid {
            cells: vec![vec![GridCell::Empty; length]; length],
            snake_locations: VecDeque::new(),
        };

        let startLocation = grid.center();

        // Set the snake's initial location on the grid
        if let Some(cell) = grid.get_cell(startLocation) {
            cell.change_cell(GridCell::SnakePart);
        } else {
            panic!("This shouldnae happen...");
        }

        grid.add_food_random_cell();
        grid.snake_locations.push_back(startLocation);

        grid
    }

    pub fn center(&self) -> CellLocation {
        let mid = self.cells.len() / 2;
        CellLocation { x: mid, y: mid }
    }

    pub fn add_food_random_cell(&mut self) {

        // We collect all the empty cells in the grid that are candidates for placing the new
        // piece of food on

        let mut empty_cells: Vec<&mut GridCell> = self.cells
            .iter_mut()
            .flat_map(|row| {
                row.iter_mut()
                    .filter(|cell| cell.is_empty())
            })
            .collect();

        // Choose one of the empty cells at random
        let empty_cell = rand::thread_rng().choose_mut(&mut empty_cells);

        // Change the cell to a Food cell
        empty_cell.map(|e| e.change_cell(GridCell::Food));
    }

    /** Moves the snake forward, returning a copy of the grid cell that the snake entered into.
     *  If the snake enters a food cell it is returned before the cell is turned into a snake
     *   cell for example.
     */
    pub fn move_snake(&mut self, direction: Direction) -> Option<GridCell> {
        let snake_head = self.get_snake_head().expect("I must've done goofed hard if this fails");

        let nextCell = snake_head.get_neighbour(direction);

        let previous = match self.get_cell(nextCell) {
            None => None,
            Some(cell_ahead) => {
                let old_neighbour = cell_ahead.clone();
                *cell_ahead = GridCell::SnakePart;


                Some(old_neighbour)
            }
        };

        previous.and_then(|p| {
            if !p.is_food() {
                self.remove_last_tail();
            }
            Some(p)
        })
    }

    fn get_snake_head(&self) -> Option<CellLocation> {
        self.snake_locations.iter().last().map(|x| *x)
    }

    fn remove_last_tail(&mut self) {
        if let Some(last_cell) = self.snake_locations.pop_front() {
            self.get_cell(last_cell).map(|cell| cell.change_cell(GridCell::Empty));
        } else {
            panic!("I dun goofed.");
        }
    }

    /** Get the grid cell if it is in range otherwise return 'Nothing' */
    fn get_cell(&mut self, location: CellLocation) -> Option<&mut GridCell> {
        let CellLocation { x, y } = location;
        self.cells.get_mut(x).and_then(|row| row.get_mut(y))
    }
}
