## Pass

Pass大体上可以理解为一个“处理”，它处理的对象是IR代码。

LLVM对代码的分析、转换和优化等处理工作都是由Pass来做的。LLVM以流水线的方式把各个Pass组合起来，让它们成为一个有序的流程。

LLVM Pass 可以处理的对象有模块(Module)、函数(Function)、循环(Loop)，甚至函数调用栈(Function Call Graph)等等。



## Out of source tree

传统的做法是将工程建立在 `lib/Transforms` 下。其实完全不必这样做，我们可以在源码树外建立一个工程，并且同时将其加入到 llvm pass 中。

新建 pass 工程如下

```
mypass
├── CMakeLists.txt	---(1)
└── src
    ├── CMakeLists.txt	---(2)
    └── mypass.cpp
```



### CMakeList

在第一个 CMakeLists.txt 中添加如下列内容。这其中的 src 就是源码目录 `src`

```
cmake_minimum_required(VERSION 3.16)
project(MyFirstPass)

add_subdirectory(src)
```



然后在第二个 CMakeLists.txt 中添加如下内容，`add_library` 第一个参数是 pass 的名字，我们将其命名为 MyPass

由于我们是在 source tree 外部建立 llvm pass，所以在依赖 llvm 头文件时必须显式指明 llvm 头文件的位置。因此使用 `INCLUDE_DIRECTORIES` 说明

```
add_library(MyPass MODULE mypass.cpp)

# 使用c++11
target_compile_features(MyPass PRIVATE cxx_range_for cxx_auto_type)
# 不使用C++ RTTI.
set_target_properties(MyPass PROPERTIES
    COMPILE_FLAGS "-fno-rtti"
)

INCLUDE_DIRECTORIES(/usr/include/llvm-13)
INCLUDE_DIRECTORIES(/usr/include/llvm-c-13)
```



### mypass.cpp

两个 CMakeList 完成后，在 `mypass.cpp` 中写入 pass 逻辑：

```cpp
#include "llvm/ADT/Statistic.h"
#include "llvm/IR/Function.h"
#include "llvm/Pass.h"
#include "llvm/Support/raw_ostream.h"

using namespace llvm;

namespace {
// Hello - The first implementation, without getAnalysisUsage.
struct Hello : public FunctionPass {
    static char ID;  // Pass identification, replacement for typeid
    Hello() : FunctionPass(ID) {}

    bool runOnFunction(Function &F) override {
        errs() << "Hello: ";
        errs().write_escaped(F.getName()) << '\n';
        
        // 没有修改源，返回 false
        // 如果修改了源，则要返回 true
        return false;
    }
};
}  // namespace

char Hello::ID = 0;
// 注意此处我们将该 pass 命名为 myhello, 该名称会在之后用到
static RegisterPass<Hello> X("myhello", "Hello World Pass");
```



### 编译运行

在 mypass 内部建立 build 文件夹，并进入其中，生成 makefile 文件

```shell
$ mkdir build && cd build
$ cmake ../

# 若无报错则 makefile 文件已生成在 build 目录下
$ make
# 生成 libMyPass.so, lib 的后缀 MyPass 是因为我们用 add_library 将库设置为 MyPass
```

将 `libMyPass.so` 放入某个文件夹下，并且在该文件夹下同时准备好 `Test.ll` 测试文件。假设该测试文件如下

```
define i32 @Foo() {
  %a = add i32 2, 3
  ret i32 %a
}

define i32 @Bar() {
  ret i32 0
}
```



使用 `opt` 命令将该文件交予我们的 pass 处理

```shell
$ opt-13 -load ./libMyPass.so -myhello test.ll -disable-output -enable-new-pm=0
```

使用 load 加载放在该文件夹下的 `libMyPass.so`，同时指定使用的 pass 名称为 `myhello`（见之前的 mypass.cpp ）

**注意**：必须加上 `-enable-new-pm=0`。因为我们使用的是旧版 pass 管理器，而 llvm 现在（我使用的是 13 ）默认使用新版管理器管理 pass，如果不加的话会报错 `opt: unknown pass name 'myhello'`

`-disable-output` 禁用 bitcode 输出，否则的话会警告 bitcode 是不可读的（除非使用 `-f` 强制打印）。其实这个命令加不加都没有大的关系，为了输出美观我们加上。

![image-20221201154237539](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20221201154237539.png)
