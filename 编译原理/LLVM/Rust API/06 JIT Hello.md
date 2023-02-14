## ORC

ORC (On Request Compilation) 是LLVM提供的新一代JIT引擎。JIT (Just In Time) 也是一个程序，它在运行时创建并执行一些新的代码，而这些新代码并不是属于JIT程序本身的



假设我们要生成以下代码，他是一个简单的加法器

```
; ModuleID = 'Add.c'
source_filename = "Add.c"

define i32 @Add(i32 %a, i32 %b) {
entry:
  %result = add i32 %a, %b
  ret i32 %result
}
```

它来自于如下函数

```c
int Add(int a, int b) {
    return a + b;
}
```

在编译型语言中，我们需要去解释执行该代码，虽然 C 并不是解释性语言。

有了 JIT 我们可以不去解释执行，而是直接将其编译为本地机器码，从而获得最佳性能体验





## 准备

生成一个二进制 crate，注意我们不是在搞 pass，不要生成 lib crate

```shell
$ cargo new hellojit
```



在 `toml` 文件中加入 `inkwell`

```toml
[dependencies]
inkwell = { git = "https://github.com/TheDan64/inkwell", branch = "master", features = [
    "llvm13-0",
] }
```



### use

在 `src/main.rs` 中开始导入依赖

```rust
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::{Linkage, Module};
use inkwell::OptimizationLevel;

use std::error::Error;
```

声明我们要编译的函数类型

```rust
type Add = unsafe extern "C" fn(i32, i32) -> i32;
```



### struct

然后声明代码生成（codegen）的结构体

```rust
struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
 fn new(
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
        execution_engine: ExecutionEngine<'ctx>,
    ) -> Self {
        Self {
            context,
            module,
            builder,
            execution_engine,
        }
    }
}
```

注意我们可以接受 module 值，可以接受 builder 值，但最好不要接受 context 而要使用引用。因为 context 是万物之源，他最好放在最顶层保证其他组件生命周期内它都存在



### main

在 rust 中建立 JIT 引擎

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("Add.c");

    // 从 module 中生成执行引擎，首先关闭优化等级
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
	
    // 生成 codegen 结构体
    let codegen = CodeGen::new(&context, module, context.create_builder(), execution_engine);

    // 获取编译后的函数
    // ok_or 对 Result 的 Err 进行映射
    let add = codegen
        .jit_compile_add()
        .ok_or("Unable to JIT compile `add`")?;

    let x = 114514;
    let y = 1919810;

    unsafe {
        println!("{} + {} = {}", x, y, add.call(x, y));
        assert_eq!(add.call(x, y), x + y);
    }

    Ok(())
}
```



### jit_compile_add

接下来是重头戏，编写二进制执行过程中生成代码的逻辑

```rust
impl<'ctx> CodeGen<'ctx> {
    // 我们要生成的是已知的 add 函数, 将它的类型放入 JitFunction 中
	fn jit_compile_add(&self) -> Option<JitFunction<Add>> {
        let i32_type = self.context.i32_type();
        let add_fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
        
        // 向 module 中添加函数
        let add_func = self
            .module
            .add_function("Add", add_fn_type, Some(Linkage::External));

        add_func.get_nth_param(0)?.set_name("a");
        add_func.get_nth_param(1)?.set_name("b");

        let basic_block = self.context.append_basic_block(add_func, "entry");
        self.builder.position_at_end(basic_block);

        let a = add_func.get_nth_param(0)?.into_int_value();
        let b = add_func.get_nth_param(1)?.into_int_value();
		
        // build add 指令
        let added = self.builder.build_int_add(a, b, "add.result");
        self.builder.build_return(Some(&added));
		
        // 验证该函数是否正确，不正确时打印信息（print 被置为 true）
        add_func.verify(true);
        self.module.print_to_stderr();

        // 返回刚才生成的 jit function
        unsafe { self.execution_engine.get_function("Add").ok() }
    }
}
```



## 测试

当所有都准备好后，整个项目的 tree 如下

```shell
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
```

运行代码

```shell
$ cargo r
```

![image-20221206195922578](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20221206195922578.png)