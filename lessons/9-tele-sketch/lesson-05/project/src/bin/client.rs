use macroquad::prelude::*;
use std::io::ErrorKind;
use std::net::UdpSocket;
use tele_sketch::event::DrawEvent;

const SERVER: &str = "127.0.0.1:9090";

// TODO: define PALETTE as a const [(u8, u8, u8); 8] with 8 colours

#[macroquad::main("Tele-Sketch")]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    socket.set_nonblocking(true).expect("set_nonblocking failed");

    let mut local_strokes:  Vec<DrawEvent> = Vec::new();
    let mut remote_strokes: Vec<DrawEvent> = Vec::new();
    let mut connected  = false;
    // TODO: declare color_idx: usize = 0
    let mut buf = [0u8; 64];

    loop {
        // TODO: derive (r, g, b) from PALETTE[color_idx]
        // TODO: derive canvas_h = screen_height() - 60.0
        // TODO: derive palette_y = screen_height() - 52.0

        // ── receive ───────────────────────────────────────────────────────
        loop {
            match socket.recv_from(&mut buf) {
                Ok((n, _)) => {
                    if let Some(ev) = DrawEvent::from_bytes(&buf[..n]) {
                        connected = true;
                        remote_strokes.push(ev);
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }

        // TODO: handle palette clicks (is_mouse_button_pressed, hit-test each button)

        // ── stroke input ──────────────────────────────────────────────────
        let (mx, my) = mouse_position();
        // TODO: guard with my < canvas_h so palette clicks don't produce strokes
        if is_mouse_button_down(MouseButton::Left) {
            // TODO: use r, g, b from palette instead of hardcoded 255, 255, 255
            let ev = DrawEvent { x: mx, y: my, r: 255, g: 255, b: 255, size: 8, pen_down: true };
            local_strokes.push(ev);
            let _ = socket.send_to(&ev.to_bytes(), SERVER);
        }

        // ── render ────────────────────────────────────────────────────────
        clear_background(Color::from_rgba(30, 30, 35, 255));

        for ev in local_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 255));
        }
        for ev in remote_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
        }

        // TODO: draw separator line at canvas_h
        // TODO: draw 8 palette buttons; outline the selected one

        let (label, lc) = if connected { ("● LIVE", GREEN) } else { ("○ waiting...", DARKGRAY) };
        draw_text(label, screen_width() - 140.0, 24.0, 20.0, lc);

        next_frame().await;
    }
}
