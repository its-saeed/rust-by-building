use macroquad::prelude::*;
use std::ops::{Add, Mul, Neg};

const GRAVITY: f32 = 500.0;

// --- Vec2 ---
#[derive(Clone, Copy)]
struct Vec2 { x: f32, y: f32 }

impl Vec2 {
    fn new(x: f32, y: f32) -> Self { Vec2 { x, y } }
    fn length(self) -> f32 { (self.x * self.x + self.y * self.y).sqrt() }
    fn normalize(self) -> Vec2 { let len = self.length(); Vec2::new(self.x / len, self.y / len) }
    fn dot(self, other: Vec2) -> f32 { self.x * other.x + self.y * other.y }
}

impl Add for Vec2 { type Output = Vec2; fn add(self, r: Vec2) -> Vec2 { Vec2::new(self.x + r.x, self.y + r.y) } }
impl Mul<f32> for Vec2 { type Output = Vec2; fn mul(self, r: f32) -> Vec2 { Vec2::new(self.x * r, self.y * r) } }
impl Neg for Vec2 { type Output = Vec2; fn neg(self) -> Vec2 { Vec2::new(-self.x, -self.y) } }

// --- Collision ---
struct Collision {
    normal: Vec2,
    penetration: f32,
}

fn detect_collision(a: &Body, b: &Body) -> Option<Collision> {
    let delta = b.position + (-a.position);
    let distance = delta.length();
    let radii_sum = a.radius + b.radius;
    if distance >= radii_sum { return None; }
    Some(Collision {
        normal: delta.normalize(),
        penetration: radii_sum - distance,
    })
}

// --- Body ---
struct Body {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Color,
    mass: f32,
}

impl Body {
    fn new(position: Vec2, velocity: Vec2, radius: f32, color: Color) -> Self {
        let mass = radius * radius;
        Body { position, velocity, radius, color, mass }
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
        self.resolve_collisions();
    }

    // TODO: extract detect_collisions into its own &self method:
    //
    // fn detect_collisions(&self) -> Vec<(usize, usize, Collision)> {
    //     let mut result = Vec::new();
    //     for i in 0..self.bodies.len() {
    //         for j in (i+1)..self.bodies.len() {
    //             if let Some(col) = detect_collision(&self.bodies[i], &self.bodies[j]) {
    //                 result.push((i, j, col));
    //             }
    //         }
    //     }
    //     result
    // }

    fn resolve_collisions(&mut self) {
        // TODO: call self.detect_collisions() here once you've extracted it
        let mut collisions: Vec<(usize, usize, Collision)> = Vec::new();
        for i in 0..self.bodies.len() {
            for j in (i+1)..self.bodies.len() {
                if let Some(col) = detect_collision(&self.bodies[i], &self.bodies[j]) {
                    collisions.push((i, j, col));
                }
            }
        }

        for (i, j, col) in &collisions {
            let correction = col.normal * (col.penetration / 2.0);
            self.bodies[*i].position = self.bodies[*i].position + (-correction);
            self.bodies[*j].position = self.bodies[*j].position + correction;

            let vrel = self.bodies[*i].velocity + (-self.bodies[*j].velocity);
            let vn = vrel.dot(col.normal);
            if vn <= 0.0 { continue; }

            let inv_mass_sum = 1.0 / self.bodies[*i].mass + 1.0 / self.bodies[*j].mass;
            let j_val = -(1.0 + 1.0) * vn / inv_mass_sum;

            let impulse = col.normal * j_val;
            self.bodies[*i].velocity = self.bodies[*i].velocity + impulse * (1.0 / self.bodies[*i].mass);
            self.bodies[*j].velocity = self.bodies[*j].velocity + (-impulse) * (1.0 / self.bodies[*j].mass);
        }
    }

    fn draw_all(&self) {
        for body in &self.bodies { body.draw(); }
    }
}

#[macroquad::main("Balls Collide")]
async fn main() {
    let mut world = World::new();
    world.add_body(Body::new(Vec2::new(200.0, 150.0), Vec2::new(150.0, 0.0), 40.0, random_color()));
    world.add_body(Body::new(Vec2::new(600.0, 150.0), Vec2::new(-150.0, 0.0), 15.0, random_color()));
    world.add_body(Body::new(Vec2::new(400.0, 300.0), Vec2::new(0.0, 0.0), 25.0, random_color()));

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            world.add_body(Body::new(
                Vec2::new(mx, my),
                Vec2::new(0.0, 0.0),
                macroquad::rand::gen_range(10.0, 40.0),
                random_color(),
            ));
        }

        world.step(dt);
        world.draw_all();
        draw_text(&format!("Bodies: {}", world.bodies.len()), 10.0, 24.0, 24.0, WHITE);

        next_frame().await;
    }
}
