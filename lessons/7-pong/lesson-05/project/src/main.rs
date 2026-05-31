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

// TODO: define enum State { Playing, GameOver }

// TODO: define struct Score { left: u32, right: u32 }
// impl Score {
//     fn new() -> Self { ... }
//     fn update(&mut self, ball: &mut Ball) -> Option<&'static str> { ... }
//       - if ball exits left:  self.right += 1; ball.reset();
//       - if ball exits right: self.left  += 1; ball.reset();
//       - if self.left  >= WIN_SCORE { return Some("Left player wins!"); }
//       - if self.right >= WIN_SCORE { return Some("Right player wins!"); }
//       - None
//     fn draw(&self) { ... }  // format "left   right", measure_text, draw_text centred
// }

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
        // TODO: remove the two left/right wall bounce blocks below —
        // the ball should now exit the screen so scoring can trigger.
        if self.rect.x < 0.0 {
            self.rect.x = 0.0;
            self.vel.x = self.vel.x.abs();
        }
        if self.rect.x + self.rect.w > WINDOW_W {
            self.rect.x = WINDOW_W - self.rect.w;
            self.vel.x = -self.vel.x.abs();
        }
    }

    fn deflect(&mut self, paddle: &Paddle) {
        let hit = (self.rect.y + self.rect.h / 2.0 - paddle.rect.y) / paddle.rect.h;
        let factor = (hit - 0.5) * 2.0;
        let speed = (self.vel.x * self.vel.x + self.vel.y * self.vel.y).sqrt();
        self.vel.y = factor * speed * 0.75;
    }

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

    // TODO: add fn reset(&mut self)
    //   - move rect back to screen centre
    //   - randomise vel using macroquad::rand::gen_range(0, 2) for x and y directions

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
    let mut left  = Paddle::new(PADDLE_OFFSET);
    let mut right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W);
    let mut ball  = Ball::new();
    // TODO: let mut score = Score::new();
    // TODO: let mut winner = "";
    // TODO: let mut state = State::Playing;

    loop {
        let dt = get_frame_time();

        // TODO: wrap update logic in match state { State::Playing => { ... } State::GameOver => { ... } }

        // State::Playing arm should contain:
        left.update(dt, KeyCode::W, KeyCode::S);
        right.update(dt, KeyCode::Up, KeyCode::Down);
        ball.update(dt);
        ball.check_paddles(&left, &right);

        // TODO: use if let to capture the winner and transition:
        //   if let Some(w) = score.update(&mut ball) { winner = w; state = State::GameOver; }

        clear_background(BLACK);
        draw_centre_line();
        left.draw();
        right.draw();
        ball.draw();
        // TODO: replace draw_score(0, 0) with score.draw()
        let text = "0   0";
        let dims = measure_text(text, None, 48, 1.0);
        draw_text(text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);

        // TODO: State::GameOver arm should draw the winner and "Press R to restart"
        //   if is_key_pressed(KeyCode::R) { reset score, ball, paddles, state }

        next_frame().await;
    }
}
