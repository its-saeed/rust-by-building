use macroquad::prelude::*;
use std::ops::{Add, Mul, Neg};

#[derive(Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }
}

// TODO: impl Add for Vec2
//       type Output = Vec2
//       fn add(self, rhs: Vec2) -> Vec2  — add components

// TODO: impl Mul<f32> for Vec2
//       fn mul(self, rhs: f32) -> Vec2  — scale both components

// TODO: impl Neg for Vec2
//       fn neg(self) -> Vec2  — negate both components

#[macroquad::main("A Ball Moves")]
async fn main() {
    let position = Vec2::new(400.0, 300.0);

    loop {
        clear_background(BLACK);
        draw_circle(position.x, position.y, 20.0, WHITE);
        next_frame().await;
    }
}
