必备 crate

但凡涉及到与 C 交互的 FFI 在 `dependencies` 都会加入这个 crate

```toml
libc = "0.2"
```



### C 方

#### 静态库

Rust 默认会链接 libc 和 libm。

当然也可以自己准备外部库。

外部库分为两种：

- 存在源文件 `xxx.c`

假设它的内容如下

```c
// 文件在工作目录下 share.c
int double_input(int input) { return input * 2; }
```

先在 toml 中添加构建必备的工具 `cc`

```toml
# 声明 build 脚本所在
build = "build.rs"

[build-dependencies]
cc = "1.0"
```

在构建脚本中预处理

```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("share.c")
        .compile("share");	// 表示将外部库编译为 libshare.a
}
```



- 只存在 `.a` 库文件

假设 C 中已经准备好外部库 `libxxx.a`，我们需要先构建 build.rs 构建脚本，通知 cargo 静态库所在

```rust
// build.rs
fn main() {
    // 声明静态库所处目录，此处简单的就在本工作空间下
    let staticlib_dir = "./";
    // 声明静态库名称，注意不要加 lib 和 .a 前后缀
    let staticlib_name = "share";
    
    // 将这两项告知 cargo
    println!("cargo:rustc-link-search=native={}", staticlib_dir);
    println!("cargo:rustc-link-lib=static={}", staticlib_name);
}
```



#### 动态库

假设 C 中已经准备好外部库 `xxx.so`，它的内容是

```c
int sub(int a, int b){
   return a-b;
}
```

假设该动态库名叫 `share.so`



### Rust 方

#### 静态库

与C语言类似，Rust使用`extern`关键字可实现对外部函数的声明，不过在调用的代码需要以[unsafe](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)关键字包成代码块

```rust
extern crate libc;
extern "C" {
    fn double_input(input: libc::c_int) -> libc::c_int;
    // rust 默认连接 libc 和 libm, 因此可以直接使用这两个库的函数
    fn write(fd: i32, data: *const u8, len: usize) -> isize;
}

fn main() {
    unsafe {
        let data = b"hello world\n";
        // 向标准输出(1) 输出 hello world
        write(1, data.as_ptr(), data.len());
    }
    
    unsafe {
        // 使用自定义函数
        let b = double_input(1);
        println!("{}", b);
    }
}
```



#### 动态库

动态调用需要 toml 中加入 crate

```toml
libloading = "0.7.4"
```



通过它就能在 rust 中动态加载 lib 并且在其中查找符号。

需要注意的是符号类型要和 C 函数库的符号类型一致

```rust
fn call_dynamic() -> Result<i32, Box<dyn std::error::Error>> {
    unsafe {
        // 注意路径要么是库的绝对路径，要么是相对于当前工作目录的相对路径
        let lib = libloading::Library::new("./share.so")?;
        let func: libloading::Symbol<
        	// 需要注意的是现在 libc 的某些类型已经和 rust 的内置类型合并, 例如 int32_t 和 i32
            unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
        // 注意符号名是 &[u8] 类型，所以要么加前缀 b, 要么在 str 后加 .as_bytes()
        > = lib.get(b"sub")?;
        Ok(func(10, 1))
    }
}

fn main() {
    let res = call_dynamic().unwrap();
    println!("{}", res);
}
```





### 注意

由于Rust只对C-ABI有稳定版本，在面对C++时我们不得不把C++函数的接口改为C版本。不过好在 C++ 全面兼容 C，这一步可以比较容易的做到