use std::str;
use std::{io::stdin, net::UdpSocket};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:9998").expect("couldn't bind to address");
    socket
        .connect("127.0.0.1:9999")
        .expect("couldn't connect to address");
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input);
        let amt = socket.send(input.as_bytes());
        let mut buffer = [0u8; 1024];
        socket.recv(&mut buffer);
        println!(
            "recv: {}",
            str::from_utf8(&buffer).expect("cannot connvert to utf-8")
        );
    }
}
