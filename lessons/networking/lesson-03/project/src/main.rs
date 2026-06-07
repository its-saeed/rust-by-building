use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // Connect to the echo server from lesson 1
    let mut stream = TcpStream::connect("127.0.0.1:7878")
        .expect("could not connect — is the echo server running?");

    // TODO: print peer address

    let message = b"hello from the client!\n";
    stream.write_all(message).expect("send failed");

    // TODO: read and print the echo from the server

    // TODO (exercise): use shutdown(Shutdown::Write) and read_to_end
    //   to read the full echo before the connection closes
}
