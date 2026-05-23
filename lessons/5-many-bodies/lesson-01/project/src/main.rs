use macroquad::prelude::*;
use std::ops::{Add, Mul, Neg};

// --- Vec2 ---
#[derive(Clone, Copy)]
struct Vec2 { x: f32, y: f32 }
impl Vec2 { fn new(x: f32, y: f32) -> Self { Vec2 { x, y } } }
impl Add for Vec2 { type Output = Vec2; fn add(self, r: Vec2) -> Vec2 { Vec2::new(self.x + r.x, self.y + r.y) } }
impl Mul<f32> for Vec2 { type Output = Vec2; fn mul(self, r: f32) -> Vec2 { Vec2::new(self.x * r, self.y * r) } }
impl Neg for Vec2 { type Output = Vec2; fn neg(self) -> Vec2 { Vec2::new(-self.x, -self.y) } }

// --- Body ---
struct Body {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
}

impl Body {
    fn new(position: Vec2, velocity: Vec2, radius: f32) -> Self {
        Body { position, velocity, radius }
    }

    fn update(&mut self, dt: f32) {
        self.position = self.position + self.velocity * dt;
    }

    fn keep_in_bounds(&mut self) {
        let (w, h) = (screen_width(), screen_height());
        if self.position.x - self.radius < 0.0 { self.position.x = self.radius; self.velocity.x = -self.velocity.x; }
        if self.position.x + self.radius > w   { self.position.x = w - self.radius; self.velocity.x = -self.velocity.x; }
        if self.position.y - self.radius < 0.0 { self.position.y = self.radius; self.velocity.y = -self.velocity.y; }
        if self.position.y + self.radius > h   { self.position.y = h - self.radius; self.velocity.y = -self.velocity.y; }
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);
    }
}

// TODO: define a World struct with a `bodies: Vec<Body>` field
// TODO: impl World {
//     fn new() -> Self
//     fn add_body(&mut self, body: Body)
//     fn draw_all(&self)
// }

#[macroquad::main("Many Bodies")]
async fn main() {
    // TODO: create a World and add at least three bodies to it
    // TODO: call world.draw_all() in the loop

    loop {
        clear_background(BLACK);
        next_frame().await;
    }
}
