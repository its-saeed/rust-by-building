use macroquad::prelude::*;
use std::ops::{Add, Mul, Neg};

// TODO: define GRAVITY as a const f32 (try 500.0)

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
        // TODO: add gravity — self.velocity.y += GRAVITY * dt
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

// --- World ---
struct World {
    bodies: Vec<Body>,
}

impl World {
    fn new() -> Self { World { bodies: Vec::new() } }
    fn add_body(&mut self, body: Body) { self.bodies.push(body); }

    fn step(&mut self, dt: f32) {
        for body in self.bodies.iter_mut() {
            body.update(dt);
            body.keep_in_bounds();
        }
    }

    fn draw_all(&self) {
        for body in &self.bodies { body.draw(); }
    }
}

#[macroquad::main("Many Bodies")]
async fn main() {
    let mut world = World::new();
    world.add_body(Body::new(Vec2::new(200.0, 100.0), Vec2::new(80.0, 0.0), 20.0));
    world.add_body(Body::new(Vec2::new(400.0, 150.0), Vec2::new(-50.0, 0.0), 15.0));
    world.add_body(Body::new(Vec2::new(600.0, 80.0),  Vec2::new(30.0, 0.0), 25.0));

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);
        world.step(dt);
        world.draw_all();
        next_frame().await;
    }
}
