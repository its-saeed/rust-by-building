use macroquad::prelude::*;
use std::collections::HashMap;
use std::io::ErrorKind::WouldBlock;
use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;
use tele_sketch::event::DrawEvent;

#[macroquad::main("Tele-Sketch Server")]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:9090").expect("failed to bind :9090");
    socket.set_nonblocking(true).expect("set_nonblocking failed");

    let mut peers: HashMap<SocketAddr, Instant> = HashMap::new();
    let mut buf      = [0u8; 64];
    let mut relayed: u64 = 0;
    let mut last_active: Option<SocketAddr> = None;

    loop {
        let now = Instant::now();

        loop {
            match socket.recv_from(&mut buf) {
                Ok((n, from)) => {
                    if DrawEvent::from_bytes(&buf[..n]).is_some() {
                        peers.insert(from, now);
                        last_active = Some(from);
                        for (&addr, _) in &peers {
                            if addr != from {
                                let _ = socket.send_to(&buf[..n], addr);
                                relayed += 1;
                            }
                        }
                    }
                }
                Err(e) if e.kind() == WouldBlock => break,
                Err(_) => break,
            }
        }

        peers.retain(|_, t| now.duration_since(*t).as_secs() < 5);

        clear_background(Color::from_rgba(18, 18, 28, 255));
        draw_text("Tele-Sketch  Server", 40.0, 55.0, 36.0, WHITE);
        draw_line(40.0, 68.0, screen_width() - 40.0, 68.0, 1.0, DARKGRAY);

        let peer_color = if peers.is_empty() { GRAY } else { GREEN };
        draw_text(&format!("peers connected: {}", peers.len()), 40.0, 110.0, 28.0, peer_color);
        draw_text(&format!("packets relayed: {relayed}"), 40.0, 142.0, 20.0, DARKGRAY);

        for (i, (addr, _)) in peers.iter().enumerate() {
            let color = if last_active == Some(*addr) { YELLOW } else { LIGHTGRAY };
            draw_text(&format!("  ●  {addr}"), 40.0, 185.0 + i as f32 * 30.0, 20.0, color);
        }

        next_frame().await;
    }
}
