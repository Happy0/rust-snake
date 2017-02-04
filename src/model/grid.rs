use model::{GridCell, CellLocation};

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

    pub fn add_food_random_cell(arena: &mut Vec<Vec<GridCell>>) {}

    pub fn get_cell_if_in_range(&mut self, location: CellLocation) -> Option<&mut GridCell> {
        let CellLocation { x, y } = location;
        self.cells.get_mut(x).and_then(|row| row.get_mut(y))
    }
}
