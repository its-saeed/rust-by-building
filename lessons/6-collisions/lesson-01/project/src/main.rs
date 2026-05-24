use macroquad::prelude::*;
use std::ops::{Add, Mul, Neg};

const GRAVITY: f32 = 500.0;

// --- Vec2 ---
#[derive(Clone, Copy)]
struct Vec2 { x: f32, y: f32 }

impl Vec2 {
    fn new(x: f32, y: f32) -> Self { Vec2 { x, y } }
    // TODO: add fn length(self) -> f32
    //   Hint: (self.x * self.x + self.y * self.y).sqrt()
}

impl Add for Vec2 { type Output = Vec2; fn add(self, r: Vec2) -> Vec2 { Vec2::new(self.x + r.x, self.y + r.y) } }
impl Mul<f32> for Vec2 { type Output = Vec2; fn mul(self, r: f32) -> Vec2 { Vec2::new(self.x * r, self.y * r) } }
impl Neg for Vec2 { type Output = Vec2; fn neg(self) -> Vec2 { Vec2::new(-self.x, -self.y) } }

// --- Body ---
struct Body {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Color,
}

impl Body {
    fn new(position: Vec2, velocity: Vec2, radius: f32, color: Color) -> Self {
        Body { position, velocity, radius, color }
    }
    fn update(&mut self, dt: f32) {
        self.velocity.y += GRAVITY * dt;
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
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }
}

fn random_color() -> Color {
    Color::new(
        macroquad::rand::gen_range(0.4, 1.0),
        macroquad::rand::gen_range(0.4, 1.0),
        macroquad::rand::gen_range(0.4, 1.0),
        1.0,
    )
}

// TODO: write fn overlapping(a: &Body, b: &Body) -> bool
//   1. Compute the vector from a.position to b.position:
//        let delta = b.position + (-a.position);
//   2. Get its length using Vec2::length()
//   3. Return true if distance < a.radius + b.radius

// --- World ---
struct World { bodies: Vec<Body> }

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

#[macroquad::main("Balls Collide")]
async fn main() {
    let mut world = World::new();
    world.add_body(Body::new(Vec2::new(200.0, 150.0), Vec2::new(150.0, 0.0), 30.0, random_color()));
    world.add_body(Body::new(Vec2::new(600.0, 150.0), Vec2::new(-150.0, 0.0), 25.0, random_color()));
    world.add_body(Body::new(Vec2::new(400.0, 300.0), Vec2::new(0.0, 0.0), 20.0, random_color()));

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            world.add_body(Body::new(Vec2::new(mx, my), Vec2::new(0.0, 0.0), 20.0, random_color()));
        }

        world.step(dt);
        world.draw_all();

        // TODO: check all pairs for overlap and draw a red line between overlapping centers:
        // for i in 0..world.bodies.len() {
        //     for j in (i+1)..world.bodies.len() {
        //         if overlapping(&world.bodies[i], &world.bodies[j]) {
        //             let a = &world.bodies[i];
        //             let b = &world.bodies[j];
        //             draw_line(a.position.x, a.position.y, b.position.x, b.position.y, 2.0, RED);
        //         }
        //     }
        // }

        draw_text(&format!("Bodies: {}", world.bodies.len()), 10.0, 24.0, 24.0, WHITE);

        next_frame().await;
    }
}
