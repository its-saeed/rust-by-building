use macroquad::prelude::*;
use std::collections::HashMap;
use std::io::ErrorKind::WouldBlock;
use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;
use tele_sketch::event::DrawEvent;

#[macroquad::main("Tele-Sketch Server")]
async fn main() {
    // TODO: bind UDP socket to "0.0.0.0:9090"
    // TODO: call set_nonblocking(true)
    let socket: UdpSocket = todo!("bind and configure socket");

    let mut peers: HashMap<SocketAddr, Instant> = HashMap::new();
    let mut buf      = [0u8; 64];
    let mut relayed: u64 = 0;
    let mut last_active: Option<SocketAddr> = None;

    loop {
        let now = Instant::now();

        // TODO: drain incoming packets in a loop
        //   - recv_from into buf
        //   - if DrawEvent::from_bytes succeeds:
        //       * insert/update peer in map
        //       * relay buf[..n] to all peers except sender
        //       * increment relayed
        //   - break on WouldBlock (or any error)

        // prune peers silent for >5 s
        peers.retain(|_, t| now.duration_since(*t).as_secs() < 5);

        // render
        clear_background(Color::from_rgba(18, 18, 28, 255));
        draw_text("Tele-Sketch  Server", 40.0, 55.0, 36.0, WHITE);
        draw_line(40.0, 68.0, screen_width() - 40.0, 68.0, 1.0, DARKGRAY);

        // TODO: draw peer count (green if >0, gray if 0)
        // TODO: draw packets relayed count
        // TODO: list each peer address (highlight last_active in yellow)

        next_frame().await;
    }
}
