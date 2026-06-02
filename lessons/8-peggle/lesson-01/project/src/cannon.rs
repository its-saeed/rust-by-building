use macroquad::prelude::*;

pub struct Cannon {
    pub pos:   Vec2,
    pub angle: f32,
}

// TODO: implement Default for Cannon
//   pos = (WINDOW_W / 2, 30), angle = FRAC_PI_2 (pointing straight down)
impl Default for Cannon {
    fn default() -> Self {
        todo!()
    }
}

impl Cannon {
    // TODO: implement draw
    //   compute tip = pos + vec2(angle.cos(), angle.sin()) * CANNON_LEN
    //   draw_line from pos to tip, width 6, color GRAY
    //   draw_circle at pos, radius 10, color DARKGRAY
    pub fn draw(&self) {
        todo!()
    }
}
