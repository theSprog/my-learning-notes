### FFI调用约定

任何调用约定都必须要考虑这三个问题：

- 参数是采用栈传递还是寄存器传递，如果是栈传递，参数是顺序压栈还是逆序压栈，如果是寄存器传递，寄存器如何和参数列表对应
- 是谁负责清栈，调用方还是被调用方
- 返回值是通过什么方式传递



在 GNU C 编译器中，默认采用 cdecl 调用约定，该约定优点在于支持不定参数。除此之外还有 stdcall 调用，fastcall 调用。

stdcall 只支持 32 位机，64位下无法胜任。

```c
// gnu C 下更改调用约定, 需要加 __attribute__((__xxx__))
int __attribute__((__stdcall__)) func()
```



### 跨语言调用实例：

#### C调用Rust

首先创建Rust库工程,我们将该名称取为 demo，后面可以看到，该名称其实无关紧要

```shell
$cargo new --lib demo
```

然后再在 toml 文件中声明依赖和导出：

```toml
[dependencies]
libc = "*"		# 我们需要 C 中的数据类型来与 C 交互

[lib]
crate-type = ["staticlib"]	# staticlib 表示生成的是静态库
name = "my_demo_lib"		# my_demo_lib 是我们库的名字，它在链接时有用
```



一切准备好后，在 src 下的 lib.rs 编写我们需要的函数，需要注意的是最好 c 中的一切类型都来自 libc 这个包，否则可能发生类型跨语言调用时的不匹配

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

完成后我们编译 `cargo build`,该命令在`./target/debug/` 生成了库，名称为 `libmy_demo_lib.a`，以 `.a` 为后缀，以 `lib` 为前缀，我们将其复制到C工程对应库目录，以便调用



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
$gcc -g -o test test.c libmy_demo_lib.a -lpthread -ldl
```

其中 `pthread` 和 `dlsym` 是必须的，`pthread` 是 rust 运行时需要，`dlsym` 是符号查找需要（dlsym 的链接选项是 -ldl）。库虽然是静态库，但我们生成的文件却是动态链接文件. `-g` 选项是便于gdb调试





#### Rust调用C

