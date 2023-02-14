## Pass

一般而言 llvm 的 pass 都是使用 C++ 写出的，但是我们也可以通过 rust 进行调用



## 准备

```shell
$ cargo new hello --lib
```

在 toml 文件中加入如下必要准备：

```toml
[package]
name = "hello"	# name 的值就是生成的库的名字，比如这里的库名称就是 libhello.a
version = "0.1.0"
edition = "2021"

# 表示生成动态库供外部某个程序调用
[lib]
crate-type = ["cdylib"]

# 依赖包
[dependencies]
llvm-plugin = { version = "0.2", features = ["llvm13-0"] }
```

`llvm-plugin` 虽然 API 暴露得有些问题，但它是目前我所能发现的唯一的 Rust Pass 构建 crate 了，将就用吧



在 `src/lib.rs`  下编写 hello pass，该 pass 名称为 `hello-world`

```rust
use llvm_plugin::inkwell::values::FunctionValue;
use llvm_plugin::{
    FunctionAnalysisManager, LlvmFunctionPass, PassBuilder, PipelineParsing, PreservedAnalyses,
};

// See https://github.com/banach-space/llvm-tutor/blob/main/HelloWorld/HelloWorld.cpp
// for a more detailed explanation.

#[llvm_plugin::plugin(name = "HelloWorld", version = "0.1")]
fn plugin_registrar(builder: &mut PassBuilder) {
    builder.add_function_pipeline_parsing_callback(|name, manager| {
        // 如果指定的 pass 名称为 hello-world, 则将本 pass 加入处理流程
        if name == "hello-world" {
            manager.add_pass(HelloWorldPass);
            PipelineParsing::Parsed
        } else {
            PipelineParsing::NotParsed
        }
    });
}

struct HelloWorldPass;
impl LlvmFunctionPass for HelloWorldPass {
    fn run_pass(
        &self,
        function: &mut FunctionValue,
        _manager: &FunctionAnalysisManager,
    ) -> PreservedAnalyses {
        // hello-world pass 流程仅仅是对没一个 function 打印出简单的语句即可
        eprintln!("(llvm-tutor) Hello from: {:?}", function.get_name());
        eprintln!(
            "(llvm-tutor)   number of arguments: {}",
            function.count_params()
        );
        PreservedAnalyses::All
    }
}
```

注意：`function.get_name()` 所得到的 `&CStr` 没有实现 `Display` 因此要使用 `Debug` 模式输出



## 构建

注意对于 lib 无法进行 `cargo r`，因为他不是一个可运行文件

使用 `out-dir` 指定生成的最终文件存放的路径。由于这是一个 unstable 特性，所以使用 flag `-Z unstable-options`

```shell
$ cargo b -Z unstable-options --out-dir ./
```

然后在 hello 目录下就可以看见一个 `libhello.so`

```
├── Cargo.lock
├── Cargo.toml
├── libhello.so		<--- 生成的库文件
├── src
│   └── lib.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
```

我们将其拷贝到需要的地方，以供调用



## 测试

假设 test.c 是测试文件

```c
// test.c
void foo() {}
void bar() { foo(); }
void fez() { bar(); }

int main() {
    foo();
    bar();
    fez();

    int ii = 0;
    for (ii = 0; ii < 10; ii++) foo();

    return 0;
}
```

编译为 `.ll` 文件

```shell
# 注意必须开启 -O, 因为如果没有指定优化级别, 或者使用了-O0, clang会添加optnone函数属性
# 如果这样的话 pass 就不会生效
$ clang-13 -O1 -S -emit-llvm test.c -o test.ll

# 如果确实需要使用 -O0 的话，必须指定 -O0 -Xclang -disable-O0-optnone，使得可以使用 pass
# 否则的话，你的 pass 即使被注册，也不会被传递给 opt, 你不会得到任何 pass 效果
$ clang-13 -O0 -Xclang -disable-O0-optnone -S -emit-llvm test.c -o test.ll
```

然后使用 opt 对 `.ll` 文件进行 “优化”（虽然这并不是什么优化，但处理一趟就是 pass，pass 就被称为优化了，就这样理解吧）

```shell
$ opt-13 --load-pass-plugin=./libhello.so --passes=hello-world test.ll -disable-output
```

![image-20221202161138883](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20221202161138883.png)