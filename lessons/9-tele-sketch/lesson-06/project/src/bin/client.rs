use macroquad::prelude::*;
// TODO 2: use macroquad::rand::gen_range;
use std::io::ErrorKind;
use std::net::UdpSocket;
use tele_sketch::event::DrawEvent;

const SERVER: &str = "127.0.0.1:9090";

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
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    socket.set_nonblocking(true).expect("set_nonblocking failed");

    // TODO 2: let id_r = gen_range(80u8, 220u8);
    // TODO 2: let id_g = gen_range(80u8, 220u8);
    // TODO 2: let id_b = gen_range(80u8, 220u8);

    let mut local_strokes:  Vec<DrawEvent> = Vec::new();
    let mut remote_strokes: Vec<DrawEvent> = Vec::new();
    let mut connected   = false;
    let mut color_idx:  usize = 0;
    // TODO 1: let mut brush_size: u8 = 8;
    let mut buf = [0u8; 64];

    loop {
        // ── derived values ────────────────────────────────────────────────
        let (r, g, b)  = PALETTE[color_idx];
        let canvas_h   = screen_height() - 60.0;
        let palette_y  = screen_height() - 52.0;

        // ── receive ───────────────────────────────────────────────────────
        loop {
            match socket.recv_from(&mut buf) {
                Ok((n, _)) => {
                    if let Some(ev) = DrawEvent::from_bytes(&buf[..n]) {
                        // TODO 3: if ev.clear { local_strokes.clear(); remote_strokes.clear(); }
                        //         else { connected = true; remote_strokes.push(ev); }
                        connected = true;
                        remote_strokes.push(ev);
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }

        // TODO 3: if is_key_pressed(KeyCode::C) {
        //     local_strokes.clear(); remote_strokes.clear();
        //     let ev = DrawEvent { x: 0.0, y: 0.0, r: 0, g: 0, b: 0, size: 0,
        //                          pen_down: false, clear: true };
        //     let _ = socket.send_to(&ev.to_bytes(), SERVER);
        // }

        // ── palette clicks ────────────────────────────────────────────────
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mpx, mpy) = mouse_position();
            for (i, _) in PALETTE.iter().enumerate() {
                let px = 10.0 + i as f32 * 50.0;
                if mpx >= px && mpx < px + 40.0 && mpy >= palette_y {
                    color_idx = i;
                }
            }
        }

        // TODO 1: let scroll = mouse_wheel().1;
        //         if scroll > 0.0 { brush_size = (brush_size + 2).min(40); }
        //         if scroll < 0.0 { brush_size = brush_size.saturating_sub(2).max(2); }

        // ── stroke input ──────────────────────────────────────────────────
        let (mx, my) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) && my < canvas_h {
            // TODO 2: use id_r, id_g, id_b instead of r, g, b (identity colour for remote players)
            // TODO 1: use brush_size instead of 8
            // TODO 3: add clear: false to the struct literal (required after adding the field)
            let ev = DrawEvent { x: mx, y: my, r, g, b, size: 8, pen_down: true };
            local_strokes.push(ev);
            let _ = socket.send_to(&ev.to_bytes(), SERVER);
        }

        // ── render ────────────────────────────────────────────────────────
        clear_background(Color::from_rgba(30, 30, 35, 255));

        // TODO 2: draw local strokes using PALETTE[color_idx] colour instead of ev.r/g/b
        //         let (pr, pg, pb) = PALETTE[color_idx];
        //         draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(pr, pg, pb, 255));
        for ev in local_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 255));
        }
        for ev in remote_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
        }

        draw_line(0.0, canvas_h, screen_width(), canvas_h, 1.0, DARKGRAY);

        for (i, &(pr, pg, pb)) in PALETTE.iter().enumerate() {
            let px = 10.0 + i as f32 * 50.0;
            draw_rectangle(px, palette_y, 40.0, 40.0, Color::from_rgba(pr, pg, pb, 255));
            if color_idx == i {
                draw_rectangle_lines(px - 2.0, palette_y - 2.0, 44.0, 44.0, 3.0, WHITE);
            }
        }

        // TODO 1: draw_circle(screen_width() - 50.0, screen_height() - 30.0,
        //                     brush_size as f32, Color::from_rgba(r, g, b, 255));
        // TODO 3: draw_text("C = clear", 10.0, 24.0, 18.0, DARKGRAY);

        let (label, lc) = if connected { ("● LIVE", GREEN) } else { ("○ waiting...", DARKGRAY) };
        draw_text(label, screen_width() - 140.0, 24.0, 20.0, lc);

        next_frame().await;
    }
}
