mod conways;

use conways::{ConwaysGrid, Position};
use macroquad::prelude::*;

const CELL_SIZE: f32 = 20_f32;
const UPDATE_TIMER: f64 = 0.1;
const PRESS_START_TEXT: &str = "Press space to start";
const CONTROLS_TEXT: &str = "Press 1-5 in game for different configurations";
const EXIT_KEY_TEXT: &str = "Press esc to exit";
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
    let configurations: [Vec<Position>; 5] = [
        vec![
            (1, 1),
            (1, 2),
            (2, 1),
            (2, 3),
            (3, 2),
            (6, 6),
            (6, 7),
            (7, 6),
            (7, 8),
            (8, 7),
            (11, 11),
            (11, 12),
            (12, 11),
            (12, 13),
            (13, 12),
            (16, 16),
            (16, 17),
            (17, 16),
            (17, 18),
            (18, 17),
            (21, 21),
            (21, 22),
            (22, 21),
            (22, 23),
            (23, 22),
            (26, 26),
            (26, 27),
            (27, 26),
            (27, 28),
            (28, 27),
        ],
        vec![
            (1, 1),
            (1, 2),
            (2, 1),
            (2, 2),
            (16, 16),
            (16, 17),
            (17, 16),
            (17, 17),
            (5, 5),
            (5, 6),
            (5, 7),
        ],
        vec![(1, 3), (2, 1), (2, 4), (3, 1), (3, 4), (4, 1)],
        vec![(1, 3), (2, 1), (2, 3), (3, 2), (3, 3)],
        vec![
            (3, 5),
            (3, 6),
            (3, 7),
            (5, 3),
            (5, 9),
            (6, 3),
            (6, 9),
            (7, 3),
            (7, 9),
            (9, 5),
            (9, 6),
            (9, 7),
        ],
    ];
    let mut index: usize = 0;
    let mut conways = ConwaysGrid::from_alive_cells(configurations[index].clone());
    let mut last_updated = 0_f64;
    let mut game_started = false;
    loop {
        clear_background(WHITE);
        if game_started {
            if let Some(last_keypress) = get_last_key_pressed() {
                match last_keypress {
                    KeyCode::Key1 => index = 0,
                    KeyCode::Key2 => index = 1,
                    KeyCode::Key3 => index = 2,
                    KeyCode::Key4 => index = 3,
                    KeyCode::Key5 => index = 4,
                    _ => {}
                }
                conways = ConwaysGrid::from_alive_cells(configurations[index].clone());
            }

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

            draw_text(EXIT_KEY_TEXT, 20_f32, 30_f32, 30_f32, WHITE);
        }

        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        next_frame().await
    }
}
