mod conways;

use conways::ConwaysGrid;
use macroquad::prelude::*;

const CELL_SIZE: f32 = 20_f32;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Conway's Game of Life"),
        window_width: 640,
        window_height: 640,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let conways = ConwaysGrid::default();
    loop {
        clear_background(WHITE);
        for (row_num, row) in conways.grid.iter().enumerate() {
            for (col_num, col) in row.iter().enumerate() {
                draw_rectangle(
                    row_num as f32 * CELL_SIZE,
                    col_num as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    match col {
                        conways::CellState::Alive => WHITE,
                        conways::CellState::Dead => BLACK,
                    },
                );
            }
        }

        next_frame().await
    }
}
