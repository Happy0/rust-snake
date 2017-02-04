use model::{ArenaCell, CellLocation};

pub struct Grid {
    cells: Vec<Vec<ArenaCell>>
}

impl Grid {
    pub fn new(length: usize) -> Grid {
        Grid { cells: vec![vec![ArenaCell::Empty; length]; length] }
    }

    pub fn center(&self) -> CellLocation {
        let mid = self.cells.len() / 2;
        CellLocation { x: mid, y: mid }
    }

    pub fn add_food_random_cell(arena: &mut Vec<Vec<ArenaCell>>) {}

    pub fn modify_arena_cell(&mut self, location: CellLocation, value: ArenaCell) {
        let CellLocation { x, y } = location;

        if let Some(startCell) = self.cells.get_mut(x).and_then(|row| row.get_mut(y)) {
            *startCell = value;
        } else {
            panic!("eeeeee");
        }
    }
}
