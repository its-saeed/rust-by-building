use macroquad::prelude::*;

const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const PADDLE_W: f32 = 12.0;
const PADDLE_H: f32 = 80.0;
const BALL_SIZE: f32 = 12.0;
const PADDLE_OFFSET: f32 = 20.0;
const PADDLE_SPEED: f32 = 400.0;
const WIN_SCORE: u32 = 5;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width: WINDOW_W as i32,
        window_height: WINDOW_H as i32,
        ..Default::default()
    }
}

// TODO: add WaitingToStart variant → enum State { WaitingToStart, Playing, GameOver }
enum State {
    Playing,
    GameOver,
}

struct Score {
    left:  u32,
    right: u32,
}

impl Score {
    fn new() -> Self {
        Score { left: 0, right: 0 }
    }

    fn update(&mut self, ball: &mut Ball) -> Option<&'static str> {
        if ball.rect.x + ball.rect.w < 0.0 {
            self.right += 1;
            ball.reset();
        }
        if ball.rect.x > WINDOW_W {
            self.left += 1;
            ball.reset();
        }
        if self.left  >= WIN_SCORE { return Some("Left player wins!"); }
        if self.right >= WIN_SCORE { return Some("Right player wins!"); }
        None
    }

    fn draw(&self) {
        let text = format!("{}   {}", self.left, self.right);
        let dims = measure_text(&text, None, 48, 1.0);
        draw_text(&text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);
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

struct Ball {
    rect: Rect,
    vel:  Vec2,
}

impl Ball {
    fn new() -> Self {
        Ball {
            rect: Rect::new(
                WINDOW_W / 2.0 - BALL_SIZE / 2.0,
                WINDOW_H / 2.0 - BALL_SIZE / 2.0,
                BALL_SIZE,
                BALL_SIZE,
            ),
            vel: Vec2::new(300.0, 220.0),
        }
    }

    fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt;
        self.rect.y += self.vel.y * dt;

        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
            self.vel.y = self.vel.y.abs();
        }
        if self.rect.y + self.rect.h > WINDOW_H {
            self.rect.y = WINDOW_H - self.rect.h;
            self.vel.y = -self.vel.y.abs();
        }
    }

    fn deflect(&mut self, paddle: &Paddle) {
        let hit = (self.rect.y + self.rect.h / 2.0 - paddle.rect.y) / paddle.rect.h;
        let factor = (hit - 0.5) * 2.0;
        let speed = (self.vel.x * self.vel.x + self.vel.y * self.vel.y).sqrt();
        self.vel.y = factor * speed * 0.75;
    }

    // TODO: change return type to bool — return true when a paddle was hit.
    // TODO: inside each overlap branch, multiply self.vel *= 1.05 to speed the ball up.
    fn check_paddles(&mut self, left: &Paddle, right: &Paddle) {
        if self.rect.overlaps(&left.rect) {
            self.rect.x = left.rect.x + left.rect.w;
            self.deflect(left);
            self.vel.x = self.vel.x.abs();
        }
        if self.rect.overlaps(&right.rect) {
            self.rect.x = right.rect.x - self.rect.w;
            self.deflect(right);
            self.vel.x = -self.vel.x.abs();
        }
    }

    fn reset(&mut self) {
        self.rect.x = WINDOW_W / 2.0 - BALL_SIZE / 2.0;
        self.rect.y = WINDOW_H / 2.0 - BALL_SIZE / 2.0;
        let dir_x = if macroquad::rand::gen_range(0, 2) == 0 { 1.0_f32 } else { -1.0 };
        let dir_y = if macroquad::rand::gen_range(0, 2) == 0 { 1.0_f32 } else { -1.0 };
        self.vel = Vec2::new(dir_x * 300.0, dir_y * 180.0);
    }

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

#[macroquad::main(window_conf)]
async fn main() {
    // TODO: load sounds before the game loop:
    //   let bounce_sound = load_sound("assets/bounce.wav").await.ok();
    //   let score_sound  = load_sound("assets/score.wav").await.ok();
    // (Using .ok() means missing files produce None instead of panicking.)

    let mut left  = Paddle::new(PADDLE_OFFSET);
    let mut right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W);
    let mut ball  = Ball::new();
    let mut score  = Score::new();
    let mut winner = "";
    // TODO: change initial state to State::WaitingToStart
    let mut state  = State::Playing;

    loop {
        let dt = get_frame_time();

        clear_background(BLACK);
        draw_centre_line();
        left.draw();
        right.draw();
        ball.draw();
        score.draw();

        match state {
            // TODO: add State::WaitingToStart arm:
            //   draw a centred "PONG" title and "Press Space to start" hint
            //   if is_key_pressed(KeyCode::Space) { state = State::Playing; }

            State::Playing => {
                left.update(dt, KeyCode::W, KeyCode::S);
                right.update(dt, KeyCode::Up, KeyCode::Down);
                ball.update(dt);

                // TODO: capture the return value of check_paddles:
                //   let hit = ball.check_paddles(&left, &right);
                //   if hit { if let Some(ref s) = bounce_sound { play_sound_once(s); } }
                ball.check_paddles(&left, &right);

                // TODO: to play score_sound, snapshot the total before update and compare after:
                //   let prev = score.left + score.right;
                //   if let Some(w) = score.update(&mut ball) { winner = w; state = State::GameOver; }
                //   if score.left + score.right > prev {
                //       if let Some(ref s) = score_sound { play_sound_once(s); }
                //   }
                if let Some(w) = score.update(&mut ball) {
                    winner = w;
                    state  = State::GameOver;
                }
            }

            State::GameOver => {
                let dims = measure_text(winner, None, 48, 1.0);
                draw_text(winner, WINDOW_W / 2.0 - dims.width / 2.0, WINDOW_H / 2.0, 48.0, WHITE);

                let hint = "Press R to restart";
                let hdims = measure_text(hint, None, 24, 1.0);
                draw_text(hint, WINDOW_W / 2.0 - hdims.width / 2.0, WINDOW_H / 2.0 + 40.0, 24.0, GRAY);

                if is_key_pressed(KeyCode::R) {
                    score = Score::new();
                    ball.reset();
                    left  = Paddle::new(PADDLE_OFFSET);
                    right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W);
                    state = State::Playing;
                }
            }
        }

        next_frame().await;
    }
}
