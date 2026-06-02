use macroquad::prelude::*;

pub struct Bucket {
    pub x:   f32,
    pub dir: f32,
}

impl Default for Bucket {
    fn default() -> Self {
        Bucket {
            x:   crate::WINDOW_W / 2.0 - crate::BUCKET_W / 2.0,
            dir: 1.0,
        }
    }
}

impl Bucket {
    pub fn draw(&self) {
        let y = crate::WINDOW_H - 30.0;
        draw_rectangle(self.x, y, crate::BUCKET_W, crate::BUCKET_H, GREEN);
    }
}
