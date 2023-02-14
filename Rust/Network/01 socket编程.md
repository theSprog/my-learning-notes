### TCP Server

#### bind

`TcpListener::bind()` 方法创建一个 `TcpListener`，他已经准备好去接受连接。如果传入的端口号为0，他将要求操作系统分配一个端口号，

```rust
 let listener = TcpListener::bind("127.0.0.1:0")?;
 let sock = TcpListener::local_addr(&listener).unwrap();
 println!("{}", sock.ip());
 println!("{}", sock.port());
```



#### income

接受请求，`income()` 方法返回一个迭代器，该迭代器的每一个元素都是该 `listener`  接收到的请求，注意该迭代器绝不会返回 `None`。该方法相当于在 `loop` 循环中调用 `listener.accept()`。

但 `accept()` 所返回的是 `(TcpStream, SocketAddr)` 的 Result 枚举类。而 `income()` 返回的就是 `TcpStream` 枚举类

```rust
for stream in listener.incoming() {
    let stream = stream.expect("stream fail");
    
    // 使用一个线程处理请求
    let handler = thread::spawn(move || {
        handle(stream).unwrap_or_else(|err| eprint!("{}", err));
    });
}
```



#### TcpStream

TcpStream 实现了 Read 和 Write 两个 traits，因此具有 `read()` 和 `write()` 两个方法

```rust
// Read traits
// 读入到 buf 中
fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

// Write traits
// 从 buf 中写入到 self 中
fn write(&mut self, buf: &[u8]) -> Result<usize>;
```

因此可以直接对 `TcpStream ` 使用 `read()` 和 `write()` 方法

```rust
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
```





### Tcp Client

#### connect

使用 connect 方法去获取与服务器的连接。

同样的，conn 也是一个 TcpStream，

```rust
let mut conn = TcpStream::connect("127.0.0.1:9999")?;
```



读取用户输入，将其传送给服务器，并用 `buf` 监听服务器响应

注意，Rust 的 `read_line` 包含末尾的换行符，这一点不同的编程语言可能实现不一样

```rust
let mut input = String::new();
let mut buf = Vec::new();
loop {
    // read_line 包含末尾的 \n 换行符
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
```



