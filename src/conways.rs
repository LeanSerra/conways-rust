const ROWS: usize = 32;
const COLS: usize = 32;

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

    pub fn next_iteration(&mut self) {
        for row in self.grid.iter() {
            for cell in row.iter() {
                todo!("Implement game rules");
            }
        }
    }
}
