### Rust 方

首先创建Rust库工程，我们需要将该库导出供 C 调用。

我们将该名称取为 demo，后面可以看到，该名称其实无关紧要。真正有用的是 toml 中声明的导出 name

```shell
$cargo new --lib demo
```



#### 静态库

然后再在 toml 文件中声明依赖和导出：

```toml
[dependencies]
libc = "*"		# 我们需要 C 中的数据类型来与 C 交互

[lib]
crate-type = ["staticlib"]	# staticlib 表示生成的是静态库
name = "my_demo_lib"		# my_demo_lib 是我们库的名字，它在链接时有用
```



一切准备好后，在 src 下的 `lib.rs` 编写我们需要的函数，需要注意的是最好 c 中的一切类型都来自 libc 这个包，否则可能发生类型跨语言调用时的不匹配

需要注意的是，对于C的结构体类型和其他自定义类型，Rust必须使用`#[repr(C)]`重新定义该类型（重新在Rust中定义一个结构体），才能在Rust中使用。

```rust
// lib.rs
extern crate libc;
use libc::int32_t;	// 使用libc里面的C语言类型


// 因为 Rust 会修改函数名称，使用 no_mangle 属性关闭 Rust 的名称修改，以便更容易链接到
// extern "C" 表示该函数可以被外部使用
#[no_mangle]	
pub extern "C" fn myFunc(a: int32_t, b: int32_t) -> int32_t {
    return a+b;
}
```

完成后我们编译 `cargo build`,该命令在`./target/debug/` 生成了库，名称为 `libmy_demo_lib.a`，以 `.a` 为后缀，以 `lib` 为前缀，我们将其复制到C工程对应库目录，以便调用。

```shell
$ cargo build
# 或者 release 版本
$ cargo build --release
```



#### 动态库

动态库与静态库差别不大，唯一的区别是 `toml` 文件中

```toml
[lib]
crate-type = ["cdylib"]	# cdylib 表示生成的是动态库
```

注意是 `cdylib`，而非 `dylib`。前者是为 C 调用的，而后者只能为 Rust 调用。

同样结果在 `target/debug` 或者 `target/release` 下



### C 方

然后再编写C测试函数,在C中调用Rust库

```c
// test.c
#include<stdio.h>
#include<stdint.h>

// 声明外部符号
extern int32_t myFunc(int32_t, int32_t);

int main(){
    int32_t c = myFunc(1,2);
    printf("c = %d\n", c);
}
```

完成后编译：

```shell
# 静态
$gcc -g -o test test.c libmy_demo_lib.a -lpthread -ldl
```

其中 `pthread` 和 `dlsym` 是必须的，`pthread` 是 rust 运行时需要，`dlsym` 是符号查找需要（dlsym 的链接选项是 -ldl）

库虽然是静态库，但我们生成的文件却是动态链接文件 `test` 

`-g` 选项是便于gdb调试

