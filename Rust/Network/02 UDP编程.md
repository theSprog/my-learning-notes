UDP编程主要需要 `UdpSocket` 这个类

### Server

bind 方法可以帮定一个端口，用来提供 UDP 服务

对于无连接服务，必须使用 `recv_from` 才能返回源地址，若使用 `recv` 则只返回接收到的字节数。若不想回复收到的消息，也可使用 `recv` 代替 `recv_from`

由于目标地址未知，所以需要 `send_to` 去指定目标地址。若直接使用 `send` 发送信息会报 `Destination address required` 错误

```rust
use std::{io, net::UdpSocket};

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:9999").expect("couldn't bind to address");
    loop {
        let mut buf = [0u8; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, src)?;
    }
}
```



### Client

`connect` 函数用于连接远程的服务器，只不过使用 UDP 协议，连接后目标地址就已知了，可以直接使用 `send` 函数：

> `send` 在目标地址已知时使用，前提是需要先 `connect`，而之前服务器端 `UDP` 回复时目标地址未知，所以需要 `recv_from` 获取地址后再 `send_to`

```rust
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
```

