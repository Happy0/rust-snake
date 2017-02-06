use model::{CellLocation, Direction, GridCell};

use rand::Rng;

// Question for Sam...
extern crate rand;

pub struct Grid {
    cells: Vec<Vec<GridCell>>,
}

impl Grid {
    pub fn new(length: usize) -> Grid {
        Grid { cells: vec![vec![GridCell::Empty; length]; length] }
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
        None //todo
    }

    /** Get the grid cell if it is in range otherwise return 'Nothing' */
    pub fn get_cell(&mut self, location: CellLocation) -> Option<&mut GridCell> {
        let CellLocation { x, y } = location;
        self.cells.get_mut(x).and_then(|row| row.get_mut(y))
    }
}
