use macroquad::prelude::*;

mod state;
mod level;
mod peg;
mod cannon;
mod bucket;
mod ball;
mod physics;

use cannon::Cannon;
use bucket::Bucket;
use peg::Peg;

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
    // TODO: create PhysicsWorld::new() and call add_walls()
    // TODO: build pegs: iterate level::level_one(), call physics.add_peg_collider(pos)
    //   for each, then Peg::new(pos, kind, handle)
    // TODO: call physics.step() each frame

    // For now, build pegs without physics handles (remove this when TODO above is done)
    let pegs: Vec<Peg> = level::level_one()
        .into_iter()
        .map(|(pos, kind)| Peg::new(pos, kind))
        .collect();

    let cannon = Cannon::default();
    let bucket = Bucket::default();

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
