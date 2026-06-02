use std::collections::VecDeque;
use macroquad::prelude::*;

const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const PADDLE_W: f32 = 12.0;
const PADDLE_H: f32 = 80.0;
const BALL_SIZE: f32 = 12.0;
const PADDLE_OFFSET: f32 = 20.0;
const PADDLE_SPEED: f32 = 400.0;
const WIN_SCORE: u32 = 5;
const TRAIL_LEN: usize = 12;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width:  WINDOW_W as i32,
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

    fn update(&mut self, ball: &Ball<'_>) -> bool {
        let left_exit  = ball.rect.x + ball.rect.w < 0.0;
        let right_exit = ball.rect.x > WINDOW_W;
        if left_exit  { self.right += 1; }
        if right_exit { self.left  += 1; }
        left_exit || right_exit
    }

    fn draw(&self) {
        let text = format!("{}   {}", self.left, self.right);
        let dims = measure_text(&text, None, 48, 1.0);
        draw_text(&text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);
    }
}

struct Paddle<'a> {
    rect:    Rect,
    texture: &'a Texture2D,
}

impl<'a> Paddle<'a> {
    fn new(x: f32, texture: &'a Texture2D) -> Self {
        Paddle {
            rect: Rect::new(x, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H),
            texture,
        }
    }

    fn update(&mut self, dt: f32, up: KeyCode, down: KeyCode) {
        if is_key_down(up)   { self.rect.y -= PADDLE_SPEED * dt; }
        if is_key_down(down) { self.rect.y += PADDLE_SPEED * dt; }
        self.rect.y = self.rect.y.clamp(0.0, WINDOW_H - PADDLE_H);
    }

    fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.rect.x,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.rect.w, self.rect.h)),
                ..Default::default()
            },
        );
    }
}

struct Ball<'a> {
    rect:    Rect,
    vel:     Vec2,
    texture: &'a Texture2D,
    trail:   VecDeque<Vec2>,
}

impl<'a> Ball<'a> {
    fn new(texture: &'a Texture2D) -> Self {
        Ball {
            rect: Rect::new(
                WINDOW_W / 2.0 - BALL_SIZE / 2.0,
                WINDOW_H / 2.0 - BALL_SIZE / 2.0,
                BALL_SIZE,
                BALL_SIZE,
            ),
            vel: Vec2::new(300.0, 220.0),
            texture,
            trail: VecDeque::new(),
        }
    }

    fn update(&mut self, dt: f32) {
        self.trail.push_back(self.rect.center());
        if self.trail.len() > TRAIL_LEN {
            self.trail.pop_front();
        }

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

    // TODO: return bool (true if hit) and multiply self.vel *= 1.05 on each hit
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
        self.trail.clear();
        self.rect.x = WINDOW_W / 2.0 - BALL_SIZE / 2.0;
        self.rect.y = WINDOW_H / 2.0 - BALL_SIZE / 2.0;
        let dir_x = if macroquad::rand::gen_range(0, 2) == 0 { 1.0_f32 } else { -1.0 };
        let dir_y = if macroquad::rand::gen_range(0, 2) == 0 { 1.0_f32 } else { -1.0 };
        self.vel = Vec2::new(dir_x * 300.0, dir_y * 180.0);
    }

    fn draw(&self) {
        let len = self.trail.len();
        for (i, &pos) in self.trail.iter().enumerate() {
            let t = i as f32 / len as f32;
            draw_circle(pos.x, pos.y, t * BALL_SIZE * 0.5, Color::new(1.0, 1.0, 1.0, t * 0.6));
        }
        draw_texture_ex(
            self.texture,
            self.rect.x,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.rect.w, self.rect.h)),
                ..Default::default()
            },
        );
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
    let paddle_texture = load_texture("assets/paddle.png").await.unwrap();
    let ball_texture   = load_texture("assets/ball.png").await.unwrap();

    // TODO: load sounds:
    //   let bounce_sound = load_sound("assets/bounce.wav").await.ok();
    //   let score_sound  = load_sound("assets/score.wav").await.ok();

    let mut left  = Paddle::new(PADDLE_OFFSET, &paddle_texture);
    let mut right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W, &paddle_texture);
    let mut ball  = Ball::new(&ball_texture);
    let mut score  = Score::new();
    let mut winner = "";
    // TODO: change to State::WaitingToStart
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
            // TODO: add State::WaitingToStart arm — title + "Press Space to start"

            State::Playing => {
                left.update(dt, KeyCode::W, KeyCode::S);
                right.update(dt, KeyCode::Up, KeyCode::Down);
                ball.update(dt);

                // TODO: let hit = ball.check_paddles(...); play bounce_sound if hit
                ball.check_paddles(&left, &right);

                if score.update(&ball) {
                    ball.reset();
                    // TODO: play score_sound here
                    if score.left  >= WIN_SCORE { winner = "Left player wins!";  state = State::GameOver; }
                    if score.right >= WIN_SCORE { winner = "Right player wins!"; state = State::GameOver; }
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
                    left  = Paddle::new(PADDLE_OFFSET, &paddle_texture);
                    right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W, &paddle_texture);
                    state = State::Playing;
                }
            }
        }

        next_frame().await;
    }
}
