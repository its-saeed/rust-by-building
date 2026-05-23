use macroquad::prelude::*;

#[macroquad::main("A Ball Moves")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Draw a white circle at the centre of the screen.
        // The window is 800×600 — centre is (400.0, 300.0).
        // Hint: draw_circle(x, y, radius, color)
        // TODO

        next_frame().await;
    }
}
