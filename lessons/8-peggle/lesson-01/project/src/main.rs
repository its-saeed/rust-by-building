use macroquad::prelude::*;

mod state;
mod level;
mod peg;
mod cannon;
mod bucket;

use cannon::Cannon;
use bucket::Bucket;

pub const WINDOW_W:    f32 = 800.0;
pub const WINDOW_H:    f32 = 600.0;
pub const PEG_RADIUS:  f32 = 8.0;
pub const BALL_RADIUS: f32 = 6.0;
pub const BUCKET_W:    f32 = 80.0;
pub const BUCKET_H:    f32 = 16.0;
pub const CANNON_LEN:  f32 = 40.0;

fn window_conf() -> Conf {
    Conf {
        window_title:  "Peggle Nights".to_owned(),
        window_width:  WINDOW_W as i32,
        window_height: WINDOW_H as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut pegs   = level::level_one();
    let     cannon = Cannon::default();
    let     bucket = Bucket::default();

    loop {
        clear_background(Color::from_hex(0x0a0a1a));

        for peg in &pegs {
            peg.draw();
        }
        cannon.draw();
        bucket.draw();

        next_frame().await;
    }
}
