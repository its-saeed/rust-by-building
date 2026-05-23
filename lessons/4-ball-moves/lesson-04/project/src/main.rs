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

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

// TODO: define a Body struct with fields: position: Vec2, velocity: Vec2, radius: f32
// TODO: impl Body { fn new(position: Vec2, velocity: Vec2, radius: f32) -> Self }
// TODO: impl Body { fn draw(&self)  — call draw_circle using self.position and self.radius }

#[macroquad::main("A Ball Moves")]
async fn main() {
    let position = Vec2::new(400.0, 300.0);

    // TODO: replace this with a Body and call body.draw()
    loop {
        clear_background(BLACK);
        draw_circle(position.x, position.y, 20.0, WHITE);
        next_frame().await;
    }
}
