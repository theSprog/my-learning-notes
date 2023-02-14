### env

#### args()获取命令行参数

`args()` 在 `std::env` 模块下，它返回一个迭代器，元素是程序输入参数，元素类型为 `String`

```rust
for cmd in env::args() {
    println!("{}", cmd);
}
```



#### vars()获取环境变量

它返回的也是迭代器，只不过元素是 `(String, String)`元组。该函数返回的是进程的全部工作环境变量

```rust
for var in env::vars() {
    println!("{} = {}", var.0, var.1);
}
```

不过一般不使用这种方式获取全部环境变量，而是使用 `var(name)` 获取 `name` 指定的环境变量

```rust
let ss: Result<String, env::VarError> = env::var("PATH");
```

但是这种方式在运行时还是有损耗，如果用户在编译时就断言环境变量的存在，完全可以把这个操作用于编译时，使用宏 `env!(name)` 代替 `var(name)`

```rust
let ss: &'static str = env!("PATH");

// 还可以定制环境变量错误信息，这样编译时环境变量出错则打印对应信息
let doc: &'static str = env!("doc", "what's that?!");
```

但是这种方式不具备通用性，我们希望在环境变量编译时未知的情况下还可以将其推迟到运行时

```rust
let doc: Option<&'static str> = option_env!("doc");
```



#### current_dir() 获取当前工作目录

#### current_exe() 获取当前可执行文件全路径



### io控制台读取

`std::io::stdin()` 返回控制台标准输入的句柄

`read_line()` 需要一个可变 `String` 的引用作为参数，返回一个 `Result`，`Ok(n)` n 代表读入的字节数。尤其要注意 `\n` 也被算入读入字节数的，而且 `String` 的末尾也不会丢掉 `\n`

同一个字符串重复使用时不会自动清空，`read_line()` 会在内部添加（`appending`）读入的内容到字符串上

```rust
let inp = io::stdin();
let mut input = String::new();
inp.read_line(&mut input);
println!("{}", input);
// 追加字符到 input
inp.read_line(&mut input);
println!("{}", input);	// 此时 input 有两次键入的内容
```



#### 写入控制台

虽然 `println!()` 宏可以在控制台写信息，但我们也可用通过文件操作替代。

```rust
// 控制台句柄必须可变才能写
let mut out = stdout();

// 控制台以 &[u8] 的方式获取
out.write(b"hello\n");
out.write("hello\n".as_bytes());
```



### file文件读取

`std::fs::File` 可以获取各种文件操作，`fs` 是 file system 的简称

文件的大部分操作都返回 `Result`，将其包装打开才得到文件句柄

```rust
let file = File::open("../test")?;
```

默认的文件是没有缓冲区的，但是可以通过 `BufReader` 创建一个有缓冲区的文件 `Reader`。缓冲区默认大小为 8K

同时此时还多了一个 `lines()` 方法可以统计源文件的每一行文本（通过返回一个迭代器），一般用此来处理文本文件

```rust
let reader = io::BufReader::new(file);
for line in reader.lines() {}
```

不必显式地关闭文件，当文件句柄的生命周期结束时默认会关闭文件



### exit退出

`std::process::exit(code)` 可以立即终结进程并且返回状态码 `code`

```rust
std::process::exit(42);
```

