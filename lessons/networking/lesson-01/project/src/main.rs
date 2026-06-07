use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle(mut stream: TcpStream) {
    // TODO: print peer address when client connects
    let mut buf = [0u8; 4096];
    loop {
        let n = match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("read error: {e}");
                break;
            }
        };
        // TODO: print how many bytes were received
        if let Err(e) = stream.write_all(&buf[..n]) {
            eprintln!("write error: {e}");
            break;
        }
    }
    // TODO: print when client disconnects
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("failed to bind port 7878");

    println!("listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.expect("incoming connection failed");
        handle(stream);
    }
}
