use std::net::UdpSocket;

fn run_server() {
    let socket = UdpSocket::bind("127.0.0.1:7879").expect("failed to bind");
    println!("UDP server listening on 127.0.0.1:7879");

    let mut buf = [0u8; 1472];
    loop {
        // TODO: recv_from, print sender + byte count, send_to echo back
        let (n, from) = socket.recv_from(&mut buf).expect("recv_from failed");
        println!("datagram from {from}: {n} bytes");
        socket.send_to(&buf[..n], from).expect("send_to failed");
    }
}

fn run_client() {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("bind failed");
    let server = "127.0.0.1:7879";

    // TODO: send a message to server with send_to
    // TODO: recv_from the echo and print it
    let message = b"hello UDP!";
    socket.send_to(message, server).expect("send failed");

    let mut buf = [0u8; 1472];
    let (n, from) = socket.recv_from(&mut buf).expect("recv failed");
    println!("echo from {from}: {:?}", std::str::from_utf8(&buf[..n]).unwrap());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("server") => run_server(),
        Some("client") | None => run_client(),
        Some(other) => eprintln!("unknown mode: {other}. Use 'server' or 'client'"),
    }
}
