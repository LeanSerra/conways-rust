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

impl Default for ConwaysGrid {
    fn default() -> Self {
        let grid = vec![vec![CellState::Dead; ROWS]; COLS];
        Self { grid }
    }
}

impl ConwaysGrid {
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
                let alive =
                    Self::get_alive_count(&previous_grid, self.get_neighbor_position((row, col)));
                self.compute_next_state(alive, (row, col));
            }
        }
    }

    fn compute_next_state(&mut self, alive_count: usize, (row, col): Position) {
        let Some(grid_row) = self.grid.get(row) else {
            return;
        };
        let Some(cell) = grid_row.get(col) else {
            return;
        };
        match cell {
            CellState::Alive => {
                if !(2..=3).contains(&alive_count) {
                    self.modify_cell((row, col), CellState::Dead);
                }
            }
            CellState::Dead => {
                if alive_count == 3 {
                    self.modify_cell((row, col), CellState::Alive);
                }
            }
        }
    }

    fn get_neighbor_position(&self, (row, col): Position) -> Vec<Position> {
        let offsets = [
            (-1, -1),
            (-1, 0),
            (0, -1),
            (-1, 1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        offsets
            .iter()
            .filter_map(|(offset_x, offset_y)| {
                let new_x = (row as isize + *offset_x) as usize;
                let new_y = (col as isize + *offset_y) as usize;
                if new_x < ROWS && new_y < COLS {
                    Some((new_x, new_y))
                } else {
                    None
                }
            })
            .collect()
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
