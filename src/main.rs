mod conways;

use conways::ConwaysGrid;
use macroquad::prelude::*;

const CELL_SIZE: f32 = 20_f32;
const UPDATE_TIMER: f64 = 0.1;
const PRESS_START_TEXT: &str = "Press space to start";
const CONTROLS_TEXT: &str = "Press 0-9 in game for different configurations";
const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 640;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Conway's Game of Life"),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
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
    let mut game_started = false;
    loop {
        clear_background(WHITE);
        if game_started {
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
        } else {
            if is_key_pressed(KeyCode::Space) {
                game_started = true;
            }
            clear_background(BLACK);
            let start_text_size = measure_text(PRESS_START_TEXT, None, 60, 1_f32);
            let controls_text_size = measure_text(CONTROLS_TEXT, None, 30, 1_f32);
            let mid_point = (SCREEN_WIDTH as f32 / 2_f32, SCREEN_HEIGHT as f32 / 2_f32);
            let mid_point_start_text = (
                start_text_size.width / 2_f32,
                start_text_size.height / 2_f32,
            );
            let mid_point_controls_text = (
                controls_text_size.width as f32 / 2_f32,
                controls_text_size.height as f32 / 2_f32,
            );
            draw_text(
                PRESS_START_TEXT,
                mid_point.0 - mid_point_start_text.0,
                mid_point.1 - mid_point_start_text.1,
                60_f32,
                WHITE,
            );

            draw_text(
                CONTROLS_TEXT,
                mid_point.0 - mid_point_controls_text.0,
                mid_point.1 - mid_point_controls_text.1 + mid_point_start_text.0 + 30_f32,
                30_f32,
                WHITE,
            );
        }

        next_frame().await
    }
}
