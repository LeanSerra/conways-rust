const ROWS: usize = 32;
const COLS: usize = 32;

pub type Position = (usize, usize);

pub struct ConwaysGrid {
    pub grid: Vec<Vec<CellState>>,
}

#[derive(Clone, Debug)]
pub enum CellState {
    Dead,
    Alive,
}

impl ConwaysGrid {
    pub fn default() -> Self {
        let grid = vec![vec![CellState::Dead; ROWS]; COLS];
        Self { grid }
    }

    fn modify_cell(&mut self, pos: Position, new_state: CellState) {
        if let Some(grid_row) = self.grid.get_mut(pos.0) {
            if let Some(cell) = grid_row.get_mut(pos.1) {
                *cell = new_state;
            }
        }
    }

    pub fn from_alive_cells(alive_cells: Vec<Position>) -> Self {
        let mut conways_grid = Self::default();
        alive_cells
            .into_iter()
            .for_each(|cell| conways_grid.modify_cell(cell, CellState::Alive));
        conways_grid
    }

    pub fn next_iteration(&mut self) {
        let previous_grid = self.grid.clone();
        for row in 0..ROWS {
            for col in 0..COLS {
                self.compute_next_state(&previous_grid, (row, col));
            }
        }
    }

    fn compute_next_state(&mut self, previous_grid: &[Vec<CellState>], pos: Position) {
        let neighbors: Vec<Position> = self.get_neighbor_position(pos);
        let alive = Self::get_alive_count(previous_grid, neighbors);
        let (row, col) = pos;
        if let Some(row) = self.grid.get(row) {
            if let Some(cell) = row.get(col) {
                match cell {
                    CellState::Alive => {
                        if !(2..=3).contains(&alive) {
                            self.modify_cell(pos, CellState::Dead);
                        }
                    }
                    CellState::Dead => {
                        if alive == 3 {
                            self.modify_cell(pos, CellState::Alive);
                        }
                    }
                }
            }
        }
    }

    fn get_neighbor_position(&self, pos: Position) -> Vec<Position> {
        let (row, col) = (pos.0, pos.1);
        let mut neighbors: Vec<Position> = Vec::new();
        // Top
        if row > 0 {
            // Left
            if col > 0 {
                neighbors.push((row - 1, col - 1));
            }
            if col + 1 < COLS {
                neighbors.push((row - 1, col + 1));
            }
            // Mid
            neighbors.push((row - 1, col));
        }
        // Mid Left
        if col > 0 {
            neighbors.push((row, col - 1));
        }
        // Mid Right
        if col + 1 < COLS {
            neighbors.push((row, col + 1));
        }
        // Bottom
        if row + 1 < ROWS {
            // Left
            if col > 0 {
                neighbors.push((row + 1, col - 1));
            }
            // Right
            if col + 1 < COLS {
                neighbors.push((row + 1, col + 1));
            }
            // Mid
            neighbors.push((row + 1, col));
        }
        neighbors
    }

    fn get_alive_count(previous_grid: &[Vec<CellState>], neighbors: Vec<Position>) -> usize {
        let mut count: usize = 0;
        for pos in neighbors {
            let (row, col) = pos;
            if let Some(grid_row) = previous_grid.get(row) {
                if let Some(CellState::Alive) = grid_row.get(col) {
                    count += 1;
                }
            }
        }
        count
    }
}
