use macroquad::prelude::*;
// TODO: add: use rapier2d::prelude::ColliderHandle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PegKind {
    Blue,
    Orange,
}

#[derive(Debug, Clone)]
pub struct Peg {
    pub pos:  Vec2,
    pub kind: PegKind,
    pub hit:  bool,
    // TODO: add field: pub handle: ColliderHandle
}

impl Peg {
    // TODO: add handle: ColliderHandle parameter and store it
    pub fn new(pos: Vec2, kind: PegKind) -> Self {
        Peg { pos, kind, hit: false }
    }

    pub fn draw(&self) {
        let color = match self.kind {
            PegKind::Blue   => BLUE,
            PegKind::Orange => ORANGE,
        };
        draw_circle(self.pos.x, self.pos.y, crate::PEG_RADIUS, color);
    }
}
