const ROWS: usize = 32;
const COLS: usize = 32;

pub type Position = (usize, usize);

pub struct ConwaysGrid {
    pub grid: Vec<Vec<CellState>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

impl ConwaysGrid {
    pub fn default() -> Self {
        let grid = vec![vec![CellState::Dead; ROWS]; COLS];
        Self { grid }
    }

    fn modify_cell(&mut self, (row, col): Position, new_state: CellState) {
        if let Some(grid_row) = self.grid.get_mut(row) {
            if let Some(cell) = grid_row.get_mut(col) {
                *cell = new_state;
            }
        }
    }

    pub fn from_alive_cells(alive_cells: &[Position]) -> Self {
        let mut conways_grid = Self::default();
        alive_cells
            .iter()
            .for_each(|cell| conways_grid.modify_cell(*cell, CellState::Alive));
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

    fn compute_next_state(&mut self, previous_grid: &[Vec<CellState>], (row, col): Position) {
        let neighbors: Vec<Position> = self.get_neighbor_position((row, col));
        let alive = Self::get_alive_count(previous_grid, neighbors);
        let Some(grid_row) = self.grid.get(row) else {
            return;
        };
        let Some(cell) = grid_row.get(col) else {
            return;
        };
        match cell {
            CellState::Alive => {
                if !(2..=3).contains(&alive) {
                    self.modify_cell((row, col), CellState::Dead);
                }
            }
            CellState::Dead => {
                if alive == 3 {
                    self.modify_cell((row, col), CellState::Alive);
                }
            }
        }
    }

    fn get_neighbor_position(&self, (row, col): Position) -> Vec<Position> {
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
        for (row, col) in neighbors {
            if let Some(grid_row) = previous_grid.get(row) {
                if let Some(CellState::Alive) = grid_row.get(col) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_alive() {
        let grid = ConwaysGrid::from_alive_cells(&[(16, 16)]);
        let neighbors = grid.get_neighbor_position((17, 16));
        assert_eq!(1, ConwaysGrid::get_alive_count(&grid.grid, neighbors));
    }

    #[test]
    fn rule_underpopulation() {
        let mut grid = ConwaysGrid::from_alive_cells(&[(1, 1)]);
        if let Some(row) = grid.grid.get(1) {
            if let Some(cell) = row.get(1) {
                assert_eq!(CellState::Alive, *cell);
            }
        }
        grid.next_iteration();
        if let Some(row) = grid.grid.get(1) {
            if let Some(cell) = row.get(1) {
                assert_eq!(CellState::Dead, *cell);
            }
        }
    }
    #[test]
    fn rule_survive() {
        let mut grid = ConwaysGrid::from_alive_cells(&[(1, 1), (1, 2), (1, 3)]);
        if let Some(row) = grid.grid.get(1) {
            if let Some(cell) = row.get(2) {
                assert_eq!(CellState::Alive, *cell);
            }
        }
        grid.next_iteration();
        if let Some(row) = grid.grid.get(1) {
            if let Some(cell) = row.get(2) {
                assert_eq!(CellState::Alive, *cell);
            }
        }
    }

    #[test]
    fn rule_overpopulation() {
        let mut grid = ConwaysGrid::from_alive_cells(&[(1, 1), (2, 0), (2, 1), (2, 2), (3, 1)]);
        if let Some(row) = grid.grid.get(2) {
            if let Some(cell) = row.get(1) {
                assert_eq!(CellState::Alive, *cell);
            }
        }
        grid.next_iteration();
        if let Some(row) = grid.grid.get(2) {
            if let Some(cell) = row.get(1) {
                assert_eq!(CellState::Dead, *cell);
            }
        }
    }

    #[test]
    fn rule_reproduction() {
        let mut grid = ConwaysGrid::from_alive_cells(&[(1, 1), (1, 2), (2, 1)]);
        if let Some(row) = grid.grid.get(2) {
            if let Some(cell) = row.get(2) {
                assert_eq!(CellState::Dead, *cell);
            }
        }
        grid.next_iteration();
        if let Some(row) = grid.grid.get(2) {
            if let Some(cell) = row.get(2) {
                assert_eq!(CellState::Alive, *cell);
            }
        }
    }
}
