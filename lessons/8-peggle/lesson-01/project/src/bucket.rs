use macroquad::prelude::*;

pub struct Bucket {
    pub x:   f32,
    pub dir: f32,
}

// TODO: implement Default for Bucket
//   x = WINDOW_W / 2 - BUCKET_W / 2, dir = 1.0
impl Default for Bucket {
    fn default() -> Self {
        todo!()
    }
}

impl Bucket {
    // TODO: implement draw
    //   y = WINDOW_H - 30.0
    //   draw_rectangle(self.x, y, BUCKET_W, BUCKET_H, GREEN)
    pub fn draw(&self) {
        todo!()
    }
}
