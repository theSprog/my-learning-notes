### libc

[libc](https://docs.rs/libc/latest/libc/) 是 rust 官方维护的一个 crate，包含了与 C 交互的所有基本内容，

包括：

- C 的基本类型，typedefs，枚举，结构体（比如 `Elf32_Ehdr`）等等
- C 常量，比如使用 `#define` 指令定义的那些常量
- C 静态变量
- C 函数（按它们的头文件中定义的函数签名来导出）
- C 宏，在 Rust 中会实现为 `#[inline]` 函数

另外，所有的 struct 都实现了 `Copy` 和 `Clone` 的 trait。



C 中也有一个 libc，但Rust 的 libc crate，不完全等价于 C 的 libc 库的一层封装。具体区别如下：

- Linux （以及其它 unix-like 平台）下，libc crate 导出的是 libc, libm, librt, libdl, libutil 和 libpthread 这几个库的符号。

- OSX 下，libc crate 导出的是 libsystem_c, libsystem_m, libsystem_pthread, libsystem_malloc 和 libdyld 这几个库的符号。

- Windows 下，libc crate 导出的是 VS CRT（VS C RunTime VS C 运行时库）中的符号。

  > 但是这些符号，比前两个平台的符号，数量上要少得多。
  >
  > 因此，可以直接这样说，Rust libc crate 在 Windows 平台上的功能有限。在 Windows 平台上，建议使用 `winapi` 这个 crate 进行开发。



### example

Rust 不能创建进程，我们使用 libc 创建进程

```rust
use libc::fork;
use libc::getpid;
use libc::getppid;

fn main() {
    unsafe {
        let pid = fork();
        if pid > 0 {	// 假如说是父进程
            println!("Hello, I am parent thread: {}", getpid());
        } else if pid == 0 {	// 当前进程是子进程
            println!("Hello, I am child thread: {}", getpid());
            println!("My parent thread: {}", getppid());	// 得到当前进程的父进程 pid
        } else {	// 出错时
            println!("Fork creation failed!");
        }
    }
}
```



### libc 与 std::os::*::raw

在标准库的 os 模块下面，有一些东西与 libc 的重复。例如  `c_char, c_double, c_float, c_int, c_long`。而 libc 中，对这些内容，也重新定义了一份这些变量。

这是因为，std::os::raw 中这些定义，可以用于与一些**简单**的 C 代码进行交互，比如说不存在系统调用的 C 代码。这个时候，就不需要再引入 libc 库了。而一旦产生了系统调用或者 Unix 环境编程，那么就得引入 libc 库来操作。



### `CStr` 和 `CString`

#### CStr 

CStr 是 C 中生成的字符串，供 Rust 使用。它对应于 Rust 的 str，但不拥有所有权。所以`CStr`表示一个以终止符`\0`结尾的字节数组的引用。

如果它是有效的 UTF-8 字符串，你甚至可以将其转换为 Rust 语言中的`&str`。实现从 C 语言到 Rust 语言的字符串传递。 

注意的是 CStr 都不拥有所有权，因此只是一个引用

```rust
// 接受 C 类的字符串
let c_str: &CStr = CStr::from_bytes_with_nul(b"hello\0").unwrap();

// 或者接受一个指针
unsafe {
    let pointer = "abc\0".as_bytes().as_ptr() as *const c_char;
    let c_str = CStr::from_ptr(pointer);
}	
```



#### CString

在 Rust 语言中生成的字符串，供 C 使用。

`CString`以终止符`\0`结尾，并且没有内部`\0`字符，代码可以首先从 Rust 语言的普通字符串创建`CString`类型，然后将其作为参数传递给使用 C-ABI 约定的字符串函数。实现从 Rust 语言到 C 语言的字符串传递

注意 CString 创建的数据结构 Rust 是拥有所有权的。

```rust
let c_string = CString::new("hello").unwrap();
```



### nix

libc 暴露的基本都是 unsafe API，nix 相当于是对 libc 又进行了一次封装，使得暴露的基本都是 safe API。

同时提供了更加 rust 风格的调用方式

```rust
use nix::unistd::*;

// fork 哪怕包装后仍然是 unsafe 的
match unsafe { fork() } {
    // 错误处理的风格已经 Rust 了许多
    Ok(ForkResult::Parent { child }) => {
        // 在父进程中
        println!(
            "Hello, I am parent thread: {} and my child is {}",
            getpid(),
            child
        );
    }
    Ok(ForkResult::Child) => {
        // 在子进程中
        println!("Hello, I am child thread: {}", getpid());
        println!("My parent thread: {}", getppid());
    }
    Err(errno) => {
        // fork 创建子进程失败
        println!("Fork creation failed! {}", errno);
    }
}
```

