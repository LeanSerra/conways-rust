mod conways;

use conways::ConwaysGrid;
use macroquad::prelude::*;

const CELL_SIZE: f32 = 20_f32;
const UPDATE_TIMER: f64 = 0.1;

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
    let alive_cells = vec![
        (1, 3),
        (1, 4),
        (2, 1),
        (2, 6),
        (3, 7),
        (4, 1),
        (4, 7),
        (5, 2),
        (5, 3),
        (5, 4),
        (5, 5),
        (5, 6),
        (5, 7),
    ];
    let mut conways = ConwaysGrid::from_alive_cells(alive_cells);
    let mut last_updated = 0_f64;
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
        if get_time() - last_updated > UPDATE_TIMER {
            last_updated = get_time();
            conways.next_iteration();
        }

        next_frame().await
    }
}
