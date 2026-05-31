use macroquad::prelude::*;

const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const PADDLE_W: f32 = 12.0;
const PADDLE_H: f32 = 80.0;
const BALL_SIZE: f32 = 12.0;
const PADDLE_OFFSET: f32 = 20.0;
// TODO: add const PADDLE_SPEED: f32 = 400.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width: WINDOW_W as i32,
        window_height: WINDOW_H as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // TODO: change these to `let mut` so they can be updated each frame
    let left_paddle  = Rect::new(PADDLE_OFFSET, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H);
    let right_paddle = Rect::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H);
    let ball         = Rect::new(WINDOW_W / 2.0 - BALL_SIZE / 2.0, WINDOW_H / 2.0 - BALL_SIZE / 2.0, BALL_SIZE, BALL_SIZE);

    loop {
        let dt = get_frame_time();

        // TODO: move left_paddle with W (up) and S (down)
        //   if is_key_down(KeyCode::W) { left_paddle.y -= PADDLE_SPEED * dt; }
        //   if is_key_down(KeyCode::S) { left_paddle.y += PADDLE_SPEED * dt; }

        // TODO: move right_paddle with Up and Down arrow keys

        // TODO: clamp both paddles to stay within the screen
        //   left_paddle.y  = left_paddle.y.clamp(0.0, WINDOW_H - PADDLE_H);
        //   right_paddle.y = right_paddle.y.clamp(0.0, WINDOW_H - PADDLE_H);

        clear_background(BLACK);

        let mut y = 10.0;
        while y < WINDOW_H {
            draw_line(WINDOW_W / 2.0, y, WINDOW_W / 2.0, y + 15.0, 2.0, DARKGRAY);
            y += 25.0;
        }

        draw_rectangle(left_paddle.x,  left_paddle.y,  left_paddle.w, left_paddle.h,  WHITE);
        draw_rectangle(right_paddle.x, right_paddle.y, right_paddle.w, right_paddle.h, WHITE);
        draw_rectangle(ball.x, ball.y, ball.w, ball.h, WHITE);

        let score_text = "0   0";
        let dims = measure_text(score_text, None, 48, 1.0);
        draw_text(score_text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);

        next_frame().await;
    }
}
