use macroquad::prelude::*;

const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const PADDLE_W: f32 = 12.0;
const PADDLE_H: f32 = 80.0;
const BALL_SIZE: f32 = 12.0;
const PADDLE_OFFSET: f32 = 20.0;
const PADDLE_SPEED: f32 = 400.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width: WINDOW_W as i32,
        window_height: WINDOW_H as i32,
        ..Default::default()
    }
}

struct Paddle {
    rect: Rect,
}

impl Paddle {
    fn new(x: f32) -> Self {
        Paddle {
            rect: Rect::new(x, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H),
        }
    }

    fn update(&mut self, dt: f32, up: KeyCode, down: KeyCode) {
        if is_key_down(up)   { self.rect.y -= PADDLE_SPEED * dt; }
        if is_key_down(down) { self.rect.y += PADDLE_SPEED * dt; }
        self.rect.y = self.rect.y.clamp(0.0, WINDOW_H - PADDLE_H);
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}

// TODO: add `vel: Vec2` field to Ball
struct Ball {
    rect: Rect,
}

impl Ball {
    // TODO: initialise vel: Vec2::new(300.0, 220.0) in new()
    fn new() -> Self {
        Ball {
            rect: Rect::new(
                WINDOW_W / 2.0 - BALL_SIZE / 2.0,
                WINDOW_H / 2.0 - BALL_SIZE / 2.0,
                BALL_SIZE,
                BALL_SIZE,
            ),
        }
    }

    // TODO: add fn update(&mut self, dt: f32)
    //   1. move:  self.rect.x += self.vel.x * dt;
    //             self.rect.y += self.vel.y * dt;
    //   2. top wall:    if self.rect.y < 0.0 { self.rect.y = 0.0; self.vel.y = self.vel.y.abs(); }
    //   3. bottom wall: if self.rect.y + self.rect.h > WINDOW_H { ... vel.y = -vel.y.abs(); }
    //   4. left wall (temp):  if self.rect.x < 0.0 { ... vel.x = vel.x.abs(); }
    //   5. right wall (temp): if self.rect.x + self.rect.w > WINDOW_W { ... vel.x = -vel.x.abs(); }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}

fn draw_centre_line() {
    let mut y = 10.0;
    while y < WINDOW_H {
        draw_line(WINDOW_W / 2.0, y, WINDOW_W / 2.0, y + 15.0, 2.0, DARKGRAY);
        y += 25.0;
    }
}

fn draw_score(left: u32, right: u32) {
    let text = format!("{}   {}", left, right);
    let dims = measure_text(&text, None, 48, 1.0);
    draw_text(&text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut left  = Paddle::new(PADDLE_OFFSET);
    let mut right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W);
    // TODO: change to `let mut ball` once update() is implemented
    let ball = Ball::new();

    loop {
        let dt = get_frame_time();

        left.update(dt, KeyCode::W, KeyCode::S);
        right.update(dt, KeyCode::Up, KeyCode::Down);
        // TODO: call ball.update(dt)

        clear_background(BLACK);
        draw_centre_line();
        left.draw();
        right.draw();
        ball.draw();
        draw_score(0, 0);

        next_frame().await;
    }
}
