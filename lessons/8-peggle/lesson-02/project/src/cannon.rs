use macroquad::prelude::*;

pub struct Cannon {
    pub pos:   Vec2,
    pub angle: f32,
}

impl Default for Cannon {
    fn default() -> Self {
        Cannon {
            pos:   Vec2::new(crate::WINDOW_W / 2.0, 30.0),
            angle: std::f32::consts::FRAC_PI_2,
        }
    }
}

impl Cannon {
    pub fn draw(&self) {
        let tip = self.pos + Vec2::new(
            self.angle.cos() * crate::CANNON_LEN,
            self.angle.sin() * crate::CANNON_LEN,
        );
        draw_line(self.pos.x, self.pos.y, tip.x, tip.y, 6.0, GRAY);
        draw_circle(self.pos.x, self.pos.y, 10.0, DARKGRAY);
    }
}
