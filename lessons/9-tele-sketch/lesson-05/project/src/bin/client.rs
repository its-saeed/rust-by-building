use macroquad::prelude::*;
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

    let mut local_strokes:  Vec<DrawEvent> = Vec::new();
    let mut remote_strokes: Vec<DrawEvent> = Vec::new();
    let mut color_idx:  usize = 0;
    let mut brush_size: u8    = 8;
    let mut connected         = false;
    let mut buf = [0u8; 64];

    loop {
        let (r, g, b) = PALETTE[color_idx];
        let canvas_h  = screen_height() - 60.0;

        // receive
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

        // TODO (lesson 5): handle 'C' key press
        //   clear local_strokes and remote_strokes
        //   send a DrawEvent with pen_down: false, x: -1.0 as a clear sentinel
        //   (in lesson 5 we add a proper clear: bool field to DrawEvent instead)

        // mouse input
        let (mx, my) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) && my < canvas_h {
            let ev = DrawEvent { x: mx, y: my, r, g, b, size: brush_size, pen_down: true };
            local_strokes.push(ev);
            let _ = socket.send_to(&ev.to_bytes(), SERVER);
        }

        // colour palette
        let palette_y = screen_height() - 52.0;
        for (i, &(pr, pg, pb)) in PALETTE.iter().enumerate() {
            let px = 10.0 + i as f32 * 50.0;
            draw_rectangle(px, palette_y, 40.0, 40.0, Color::from_rgba(pr, pg, pb, 255));
            if color_idx == i {
                draw_rectangle_lines(px - 2.0, palette_y - 2.0, 44.0, 44.0, 3.0, WHITE);
            }
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mpx, mpy) = mouse_position();
                if mpx >= px && mpx < px + 40.0 && mpy >= palette_y {
                    color_idx = i;
                }
            }
        }

        // brush size
        let scroll = mouse_wheel().1;
        if scroll > 0.0 { brush_size = (brush_size + 2).min(40); }
        if scroll < 0.0 { brush_size = brush_size.saturating_sub(2).max(2); }

        // render
        clear_background(Color::from_rgba(30, 30, 35, 255));

        for ev in local_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 255));
        }
        for ev in remote_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
        }

        draw_line(0.0, canvas_h, screen_width(), canvas_h, 1.0, DARKGRAY);
        draw_circle(screen_width() - 50.0, screen_height() - 30.0,
                    brush_size as f32, Color::from_rgba(r, g, b, 255));

        let (label, color) = if connected { ("● LIVE", GREEN) } else { ("○ waiting...", DARKGRAY) };
        draw_text(label, screen_width() - 140.0, 24.0, 20.0, color);

        // TODO (lesson 5): add "Press C to clear" hint text at bottom right of palette row

        next_frame().await;
    }
}
