use macroquad::prelude::*;

#[macroquad::main("conways-rust")]
async fn main() {
    loop {
        clear_background(RED);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
