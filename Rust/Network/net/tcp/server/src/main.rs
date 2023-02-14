use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    loop {
        let bytes = stream.read(&mut buf)?;
        if bytes == 0 {
            break;
        }
        stream.write(&buf[..bytes])?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9999")?;
    let mut thread_vec = Vec::new();
    for stream in listener.incoming() {
        let stream = stream.expect("stream fail");
        let handler = thread::spawn(move || {
            handle(stream).unwrap_or_else(|err| eprint!("{}", err));
        });
        thread_vec.push(handler);
    }

    for handler in thread_vec {
        handler.join().unwrap();
    }

    Ok(())
}
