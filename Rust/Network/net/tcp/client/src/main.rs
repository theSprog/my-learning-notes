use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    str,
};

fn main() -> io::Result<()> {
    let mut conn = TcpStream::connect("127.0.0.1:9999")?;
    let mut input = String::new();
    let mut buf = Vec::new();
    loop {
        io::stdin().read_line(&mut input).expect("read failed");
        if input.eq("exit\n") {
            println!("bye!");
            break;
        }

        conn.write(input.as_bytes()).expect("write error!");
        let mut reader = BufReader::new(&conn);
        reader.read_until(b'\n', &mut buf).expect("read to buf err");
        println!("recieve: {}", str::from_utf8(&buf).unwrap());

        input.clear();
        buf.clear();
    }

    Ok(())
}
