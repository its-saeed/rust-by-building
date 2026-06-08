use macroquad::prelude::*;
use tele_sketch::event::DrawEvent;

#[macroquad::main("Tele-Sketch")]
async fn main() {
    // TODO: declare local_strokes: Vec<DrawEvent>
    // TODO: declare remote_strokes: Vec<DrawEvent>

    loop {
        // ── input ─────────────────────────────────────────────────────────
        // TODO: get mouse_position()
        // TODO: if left button is held, create a DrawEvent with:
        //         r: 255, g: 255, b: 255, size: 8, pen_down: true
        //       and push to local_strokes

        // ── render ────────────────────────────────────────────────────────
        // TODO: clear_background with Color::from_rgba(30, 30, 35, 255)
        // TODO: draw a circle for each local_strokes event where pen_down is true
        // TODO: draw a circle for each remote_strokes event where pen_down is true
        //       (use alpha 200 for remote strokes to distinguish them)

        next_frame().await;
    }
}
