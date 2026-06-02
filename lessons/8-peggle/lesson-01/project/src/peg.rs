use macroquad::prelude::*;

// TODO: add #[derive(Debug, Clone, Copy, PartialEq)] to PegKind
pub enum PegKind {
    Blue,
    Orange,
}

// TODO: add #[derive(Debug, Clone)] to Peg
pub struct Peg {
    pub pos:  Vec2,
    pub kind: PegKind,
    pub hit:  bool,
}

impl Peg {
    // TODO: implement new(pos: Vec2, kind: PegKind) -> Self
    pub fn new(pos: Vec2, kind: PegKind) -> Self {
        todo!()
    }

    // TODO: implement draw — blue pegs draw as BLUE, orange as ORANGE
    //   use draw_circle(self.pos.x, self.pos.y, crate::PEG_RADIUS, color)
    pub fn draw(&self) {
        todo!()
    }
}
