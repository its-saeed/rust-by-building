use macroquad::prelude::*;
use crate::peg::PegKind;

// Returns (position, kind) pairs — Peg is constructed in main after
// adding the collider to PhysicsWorld (which gives us the ColliderHandle).
pub fn level_one() -> Vec<(Vec2, PegKind)> {
    let mut pegs = Vec::new();
    for row in 0..5_u32 {
        for col in 0..9_u32 {
            let x = 100.0 + col as f32 * 75.0;
            let y = 180.0 + row as f32 * 70.0;
            let kind = if (row * 9 + col) % 3 == 0 {
                PegKind::Orange
            } else {
                PegKind::Blue
            };
            pegs.push((Vec2::new(x, y), kind));
        }
    }
    pegs
}
