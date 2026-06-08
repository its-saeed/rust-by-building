use macroquad::prelude::*;
use std::io::ErrorKind;
use std::net::UdpSocket;
use tele_sketch::event::DrawEvent;

const SERVER: &str = "127.0.0.1:9090";

#[macroquad::main("Tele-Sketch")]
async fn main() {
    // TODO: bind a UdpSocket to "0.0.0.0:0"
    // TODO: call set_nonblocking(true)

    let mut local_strokes:  Vec<DrawEvent> = Vec::new();
    let mut remote_strokes: Vec<DrawEvent> = Vec::new();
    let mut connected = false;
    let mut buf = [0u8; 64];

    loop {
        // ── receive ───────────────────────────────────────────────────────
        // TODO: drain loop — recv_from into buf
        //   on Ok: parse DrawEvent, set connected = true, push to remote_strokes
        //   on WouldBlock: break
        //   on other Err: break

        // ── input ─────────────────────────────────────────────────────────
        let (mx, my) = mouse_position();

        if is_mouse_button_down(MouseButton::Left) {
            let ev = DrawEvent { x: mx, y: my, r: 255, g: 255, b: 255, size: 8, pen_down: true };
            local_strokes.push(ev);
            // TODO: send ev.to_bytes() to SERVER (use let _ = to ignore errors)
        }

        // ── render ────────────────────────────────────────────────────────
        clear_background(Color::from_rgba(30, 30, 35, 255));

        for ev in local_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 255));
        }
        for ev in remote_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
        }

        // TODO: draw connection status ("● LIVE" in GREEN or "○ waiting..." in DARKGRAY)

        next_frame().await;
    }
}
