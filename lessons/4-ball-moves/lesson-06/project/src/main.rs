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

    // TODO: add fn keep_in_bounds(&mut self)
    //       use screen_width() and screen_height() for the boundaries
    //       check all four edges: left, right, top, bottom
    //       when the ball crosses an edge: correct position and flip the relevant velocity component

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);
    }
}

#[macroquad::main("A Ball Moves")]
async fn main() {
    let mut ball = Body::new(
        Vec2::new(400.0, 300.0),
        Vec2::new(200.0, 150.0),
        20.0,
    );

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        ball.update(dt);
        // TODO: call ball.keep_in_bounds()
        ball.draw();

        next_frame().await;
    }
}
