## module pass

Module Pass是针对IR代码中的每个模块 (Module)执行的。它可以对模块内的所有函数、全局变量等进行分析处理时。注意在进行处理时，Pass从模块得到函数是**无序的**。

我们来写一个最简单的Module Pass。该 Pass 会对每一个模块进行处理，即把其模块中的所有全局变量、函数等信息打印出来。



## 准备

同样的，仿照 **02节** 准备 toml

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
llvm-plugin = { version = "0.2", features = ["llvm13-0"] }
```



### 导入 crate

在 `src/lib.rs` 下导入命名空间

```rust
use llvm_plugin::inkwell::module::Module;
use llvm_plugin::utils::{FunctionIterator, GlobalIterator, InstructionIterator};
use llvm_plugin::{
    LlvmModulePass, ModuleAnalysisManager, PassBuilder, PipelineParsing, PreservedAnalyses,
};
```

对于 module pass 而言，使用的是 `inkwell::module::Module` 

另外，如果要迭代 全局变量、指令或者函数，使用 `utils` 下的迭代器



### 注册迭代器

使用 `add_module_pipeline_parsing_callback` 注册，`name` 中填写 pass 名称

```rust
#[llvm_plugin::plugin(name = "SimpleModule", version = "0.1")]
fn plugin_registrar(builder: &mut PassBuilder) {
    builder.add_module_pipeline_parsing_callback(|name, manager| {
        if name == "simple_module" {
            manager.add_pass(SimpleModulePass);
            PipelineParsing::Parsed
        } else {
            PipelineParsing::NotParsed
        }
    });
}
```



### 构建 pass

参照 FunctionPass，只要为一个结构体实现 `LlvmModulePass` trait 即可

```rust
struct SimpleModulePass;

impl LlvmModulePass for SimpleModulePass {
    fn run_pass(&self, module: &mut Module, _manager: &ModuleAnalysisManager) -> PreservedAnalyses {
        // module 名称
        eprintln!("Module: {:?}", module.get_name());
        // 源文件名称
        eprintln!("Source File Name: {:?}", module.get_source_file_name());
		
        // 全局变量迭代
        let globals = GlobalIterator::new(module).collect::<Vec<_>>();
        eprintln!("GloablCount: {:?}", globals.len());
        for global in globals {
            eprintln!("     Global Variable: {:?}", global.get_name());
        }
		
        // 函数迭代
        let funcs = FunctionIterator::new(module).collect::<Vec<_>>();
        eprintln!("FunctionCount: {:?}", funcs.len());
        
        // 统计指令
        let mut instrs = 0;
        for func in funcs {
            eprintln!("     Function: {:?}", func.get_name());
			
            // 从函数中取得基本块，然后使用指令迭代器迭代它
            func.get_basic_blocks().iter().for_each(|bb| {
                // 对于每一个基本块，统计其中的指令
                instrs += InstructionIterator::new(&bb).count();
            });
        }
        eprintln!("Instruction Count: {:?}", instrs);

        PreservedAnalyses::All
    }
}
```



## 测试

假设源文件如下

```c
// Test.c
int globalInt = 0;
short globalShort = 1;
const char* globalString = "This is a global string";

int Foo(int a) {
    int b;
    if (a > 33) {
        b = 66;
    } else {
        b = 77;
    }
    return b;
}

int Bar(int a, int b) { return a + b; }
int Bead(int a, int b) { return a * b; }
```



```shell
# 编译源文件为 LLVM IR
$ clang-13 -O0 -Xclang -disable-O0-optnone -S -emit-llvm test.c -o test.ll

# 测试
$ opt-13 --load-pass-plugin=./libsimple_module.so --passes=simple_module test.ll -disable-output
```

![image-20221205144311485](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20221205144311485.png)