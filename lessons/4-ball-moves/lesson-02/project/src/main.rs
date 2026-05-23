use macroquad::prelude::*;

// TODO: define a Vec2 struct with two f32 fields: x and y
// TODO: add #[derive(Clone, Copy)] above it
// TODO: add an impl block with fn new(x: f32, y: f32) -> Self

#[macroquad::main("A Ball Moves")]
async fn main() {
    // TODO: replace these two floats with a Vec2
    let x = 400.0_f32;
    let y = 300.0_f32;

    loop {
        clear_background(BLACK);
        draw_circle(x, y, 20.0, WHITE);
        next_frame().await;
    }
}
