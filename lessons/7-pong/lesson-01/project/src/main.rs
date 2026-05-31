use macroquad::prelude::*;

// TODO: define constants for the game dimensions
// const WINDOW_W: f32 = 800.0;
// const WINDOW_H: f32 = 600.0;
// const PADDLE_W: f32 = 12.0;
// const PADDLE_H: f32 = 80.0;
// const BALL_SIZE: f32 = 12.0;
// const PADDLE_OFFSET: f32 = 20.0;

// TODO: define window_conf() -> Conf and pass it to #[macroquad::main(...)]
#[macroquad::main("Pong")]
async fn main() {
    // TODO: create three Rect values — left_paddle, right_paddle, ball
    //   Rect::new(x, y, w, h)
    //   Position the paddles near the left/right edges, centred vertically.
    //   Position the ball at the centre of the screen.

    loop {
        clear_background(BLACK);

        // TODO: draw the dashed centre line
        //   use a while loop: draw_line segments every 25 pixels down the screen

        // TODO: draw left_paddle, right_paddle, ball using draw_rectangle

        // TODO: draw the score "0   0" centred at the top
        //   use measure_text to get the width, then offset x accordingly

        next_frame().await;
    }
}
