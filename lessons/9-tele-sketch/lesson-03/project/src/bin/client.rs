use macroquad::prelude::*;
use tele_sketch::event::DrawEvent;

const PALETTE: [(u8, u8, u8); 8] = [
    (255, 255, 255),
    (20,  20,  20 ),
    (220, 60,  60 ),
    (60,  200, 80 ),
    (60,  110, 240),
    (240, 165, 30 ),
    (200, 60,  200),
    (40,  200, 210),
];

#[macroquad::main("Tele-Sketch")]
async fn main() {
    // TODO: declare local_strokes: Vec<DrawEvent>
    // TODO: declare remote_strokes: Vec<DrawEvent>
    let mut color_idx:  usize = 0;
    let mut brush_size: u8    = 8;

    loop {
        let (r, g, b) = PALETTE[color_idx];
        let canvas_h  = screen_height() - 60.0;

        // TODO: handle mouse input
        //   if left button down AND mouse y < canvas_h:
        //     create a DrawEvent with current position, colour, size, pen_down: true
        //     push to local_strokes

        // TODO: handle colour palette
        //   draw 8 colour buttons at the bottom
        //   on left-click inside a button, update color_idx
        //   draw a white border around the selected colour

        // TODO: handle scroll wheel to change brush_size (clamp 2..=40)

        // TODO: clear background (Color::from_rgba(30, 30, 35, 255))

        // TODO: draw local_strokes (pen_down only) as circles
        // TODO: draw remote_strokes (pen_down only) as circles at alpha 200

        // TODO: draw palette separator line
        // TODO: draw brush size preview circle (current colour, current size)

        next_frame().await;
    }
}
