## Rust API

#### llvm-sys 与 inkwell

在 cargo.toml 中加入依赖

```toml
llvm-sys = "130"

# 或者说使用 safe wrapper, 他是对 llvm-sys 的一个包装
inkwell = { git = "https://github.com/TheDan64/inkwell", branch = "master", features = ["llvm13-0"] }
```

注意版本号要和 llvm version 对齐（llvm-13-dev）。之后我们的演示会使用 inkwell 而不是底层的 llvm-sys



#### llvmenv

如果不想手动适配 llvm 版本，可以使用 llvmenv。

```shell
# 注意提前安装必备的库，如果没有这两个依赖可能安装 llvmenv 失败
$ sudo apt install openssl
$ sudo apt install pkg-config

# 然后安装 llvmenv
$ cargo install llvmenv
```

选择配置 llvm 环境

```shell
$llvmenv init

# 显示所有可用 entry
$llvmenv entries

# 选择其中一个 entry, 比如 13.0.0
$llvmenv build-entry 13.0.0
```

但其实这上面的镜像也不是最新



## Context

用 LLVM 创建模块之前，需要首先初始化 Context，然后才能调用LLVM API。需要注意 context 是万物的根本，它的生命周期必须比所有的其他 llvm 组件都要长

我们可以把它理解为一个黑盒，它包含并管理了LLVM中基础的、核心的“全局”数据，如类型(Type)、标准化的常量表等。

### 线程不安全

虽然context可以在多线程中共享使用，但它不是线程安全的，因为它自己不提供数据同步（如加锁）机制。

Context本身以及它所包含的数据都是设计成给一个单独的线程使用的。所以，在多线程中共享使用context，可能会导致竞争、数据不同步等多线程常见问题。

### 线程独有

当然，如果每个线程都创建一份自己独有的context，则不会有这些问题了。当每一个线程都有自己独立的一份context时，那命名冲突、类型冲突等问题就不再存在了。

> 比如，某个线程正在处理模块A中的类型“Foo”，而另一个线程正在处理模块B中的类型“Foo”。那这两个类型不会互相干扰，因为它们处于不同的context下。

> 再举个例子，有这样一个多线程的程序，其中一个线程运行图像处理库(Graphics Library)，另一个线程运行音频处理库(Audio Library)，而这两个库都调用了LLVM。
>
> 既然它们都调用了LLVM，为了避免这两个处理任务相互干扰，我们只需让它们各自创建一份context即可

### Context 融合

> 一般情况下，每一个线程都应该创建一份自己独有的context，以确保数据安全。但如果要合并两个线程的数据，我们不能直接融合两份context对象为一份context。我们可以做的是，把两份context对象转为IR代码，然后合并IR代码




## module

简单来说，LLVM中的模块(Module)，代表了一块代码。

它是一个比较完整独立的代码块，是一个最小的编译单元。需要注意的是，它跟我们平常说的“程序模块”不一样。程序模块是一个更大的集合，包含了很多个编译单元；而LLVM中的模块(Module)，我们可以把它初步理解为一个编译单元。

一个 module 可以包含的基本构件有**全局变量**、**函数**、**数据结构**等。

初始化环境

```rust
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use std::error::Error;

fn init<'ctx>(context: &'ctx Context, module_name: &str) -> (Module<'ctx>, Builder<'ctx>) {
    (context.create_module(module_name), context.create_builder())
}

let context = Context::create();
let (module, builder) = init(&context, "test");
```

打印输出到屏幕或者文件

```rust
module.print_to_file("out.ll").unwrap();
module.print_to_stderr();
```



## Builder

Builder的作用其实比较纯粹，它的存在就是为了让我们方便快捷地创建IR指令（Instruction）。当然在构建指令之前需要先设定 basicblock，即需要在哪一个基本块上构建指令

Builder并不是全能的，它并没有把所有与创建IR有关的API都集成进来。如果要使用其它未集成的API，可以在创建好指令之后，直接对指令进行操作，比如直接调用 LoadInst::setVolatile() 等各种 Instruction 的成员函数

```rust
// 使用 builder 构建指令，返回值是 InstructionValue
let inst = builder.build_store(b_ptr, i32_type.const_int(77, false));
// 该类可以设定指令的一些基本属性，如 volatile
inst.set_volatile(true).unwrap();
```

Builder的工作方式是，首先指定当前基本块，然后逐指令插入。Builder在其内部记录了当前的 BasicBlock，以及当前 BasicBlock 中的当前指令。每次插入新的指令时，总是在当前指令的后面插入。



## type

```rust
use inkwell::AddressSpace;

let i32_type = context.i32_type();	// i32 类型
let i32_ptr_type = i32_type.ptr_type(AddressSpace::Generic);	// i32* 类型
let vector_type = i32_type.vec_type(2);	// <2 * i32>
let arr_type = i32_type.array_type(2);	// [2 * i32]

let vec_size = vector_type.get_size();
let vec_ele_type = vector_type.get_element_type();
assert!(vec_ele_type.is_int_type());
```



## function

要创建函数先创建函数类型，然后依据函数类型创建函数

### function_type

函数类型包含两部分：返回值类型 和 参数类型

```rust
 let fn_type = i32_type.fn_type(
     &[
         i32_type.into(),
         i32_ptr_type.into(),
         vector_type.into(),
         arr_type.into(),
     ],
     false,	// 是否是变参数
);
```



### function

```rust
// 函数名假设为 Test
let function = module.add_function("Test", fn_type, None);
// 将第一个参数名设置为 a
function.get_nth_param(0).unwrap().set_name("a");

// 开启函数验证，他会检查我们的函数是否符合规范
// 参数表示验证失败时是打印还是直接返回
function.verify(true);
```



### attribute

函数可以添加各种属性，然而诡异的是需要用 u32 来代指属性名称。例如参数使用寄存器传递的属性代号是 `8`，创建该属性并将其添加到 `attribute` 中

```rust
use inkwell::attributes::{Attribute, AttributeLoc};

let enum_attribute = context.create_enum_attribute(8, 0);
function.add_attribute(AttributeLoc::Param(0), enum_attribute);
```

幸运的是我们可以使用 `Attribute::get_named_enum_kind_id("xxx")` 来查询每一个属性的代号，需要注意的是字符串是全小写，如果不存在此属性则返回 `0`

```rust
let inreg = Attribute::get_named_enum_kind_id("inreg");	// 8
let enum_attribute = context.create_enum_attribute(inreg, 0);
```



## BasicBlock

一个函数由若干个 bb 组成，每一个 bb 都只能有一个入口和一个出口。bb 的主体即一系列的IR指令。bb 的起点是一个标签，它代表 bb 的名称。标签的下一行就是该 bb 的第一条IR指令。

一个 function 的 bb 组织为一个 List\<bb>，而由 bb 上的标签来决定复杂的控制流关系

```rust
let entry_block = context.append_basic_block(function, "entry");
// 将 builder 放在 bb 的末尾，准备构建指令
builder.position_at_end(entry_block);
```

值得注意的是 builder 放在何处就会在该处构建 bb。这排除了存在多个 bb 时要在何处构建指令的混淆



## Return

ret 构建返回值。当然，函数也可以不返回结果（即返回 void）

```rust
// 返回值也要有类型，而且要和函数声明的返回值类型一致
// 如果不一致的话，在不开启 Verify 时会通过，开启后会报错
 let res = i32_type.const_int(114514, false);	// 常量
builder.build_return(Some(&res));

// 函数也可以不返回
// ret void
builder.build_return(None);
```



## Arithmetic

llvm 可以直接支持基本的算数运算，例如加减乘除

可以使用 BuildAdd，BuildSub 等特殊的构建函数

```rust
// 将函数的第 0 个参数 x 3，结果命名为 add_res
let param1 = function.get_nth_param(0).unwrap().into_int_value();
// 扩展到高位时（如 i64）时以 0 填充
let three = i32_type.const_int(3, false);
builder.build_int_add(x, three, "add_res");
```





## local&global var

### local var

局部变量是存储在栈上，llvm 提供了申请局部变量的函数

```rust
// 申请一个 i32 的栈空间，将之命名为 local_1
let i32_ptr = builder.build_alloca(i32_type, "local_1");
// %local_1 = alloca i32, align 4
```

注意用 `alloca` 指令申明的变量，其实得到是变量的**地址**。

如果要访问它，我们需要用 store 和 load 指令。

```rust
// store 指令可以把变量值写入该地址
builder.build_store(i32_ptr, i32_type.const_int(3, false));
// store i32 3, i32* %local_1, align 4

// load 指令则可以把变量值从该地址的读取出来，假设将其取名为 load_1。
// 默认是存入什么类型就读取什么类型
let pointee = builder.build_load(i32_ptr, "load_1");
builder.build_return(Some(&pointee));
// %load_1 = load i32, i32* %local_1, align 4
// ret i32 %load_1
```



### global var

全局变量是在一个模块(Module)之内全局可见的变量，一个模块内存在许多函数，也就是说模块内所有的函数都能用它。全局变量都是表示为指针

LLVM 提供了创建全局变量以及查找全局变量的方法。创建全局变量之后，我们可以配置它的属性，如链接类型、内存对齐模型等。

```rust
// 在 module 内创建 i32_type 全局变量，命名为 global_var
module.add_global(i32_type, Some(AddressSpace::Generic), "global_var");
// 查找全局变量, 同样的查找到的全局变量也是指针，需要使用时仍需 load
let glb = module.get_global("global_var");
assert!(glb.is_some());

// 加载全局变量，将之命名为 load_global
let loaded_global = builder.build_load(glb.unwrap().as_pointer_value(), "load_global");
```

设置全局变量属性

```rust
glb.unwrap().set_linkage(inkwell::module::Linkage::Common);
glb.unwrap().set_alignment(4);
// ...
```

如果是数组，设置初始化时可以 `zero_initializer`

```rust
glb.set_initializer(&vector_type.const_zero());
```

常量字符串也是全局变量

```rust
unsafe {
    builder.build_global_string("this is global string", "global_str");
}
```



## 字符串

### 非 `\0` 结尾

假设我们有 `"hello world"` 这一个字符串，我们应该怎么在 llvm 中声明

```rust
let string = "hello world";
// 字符串长度是固定的，因此是 i8 array，注意此处不能用 i8 vector
let ty = context.i8_type().array_type(string.len() as u32);
let gv = module.add_global(ty, Some(AddressSpace::Generic), "const_str");
// 内连接
gv.set_linkage(Linkage::Internal);

// 向全局变量中填充内容
// 注意 null_terminated 是 false，代表不是以 \0 结尾
gv.set_initializer(&context.const_string(string.as_ref(), false));
```



### 以 `\0` 结尾

在调用 `printf` 之类的函数时，传入的字符串退化为指针，因此需要以 `\0` 结尾指明字符串终点

```rust
// 需要预留出一个空位
let ty = context.i8_type().array_type((string.len() + 1) as u32);
let glob = module.add_global(ty, Some(AddressSpace::Generic), "const_str");

glob.set_linkage(Linkage::Internal);
// CString 末尾是 \0, 这时的内存布局就和 C 比较像了
let string = CString::new(string).unwrap();
// as_bytes 转为 &u[8], null_terminated 设置为 true
glob.set_initializer(&ctx.const_string(string.as_bytes(), true));
```



## GetElementPtr

GetElementPtr指令其实是一条**指针计算**语句，本身并不进行任何数据的访问或修改，进行是计算指针，修改计算后指针的类型。

GetElementPtr至少有两个参数，第一个参数为要进行计算的原始指针，往往是一个结构体指针，或数组首地址指针。

第二个参数及以后的参数，都称为indices，表示要进行计算的参数，如结构体的第几个元素（以 0 为起点），数组的第几个元素。



### 简单的例子

假设存在如下 C 语言

```c
int a[10];
int b = a[2];
```



拿到一个指针后 a，先是寻址（哪怕就是头元素也要寻址 0），然后再取元素指针

```
%4 = getelementptr inbounds [10 x i32], [10 x i32]* %2, i64 0, i64 2
%5 = load i32, i32* %4, align 8
```

上面的 %4 是一个指针，所以要 load 之后才能取值



### 一个更复杂的例子

```
struct RT {
    char A;
    int B[10][20];
    char C;
};
struct ST {
    int X;
    double Y;
    struct RT Z;
};

int foo(struct ST *s) { return s[1].Z.B[5][13]; }
```

它们的每一次取值都是一次 GetElementPtr 运算：

- `s[1]`

  ```
  %4 = getelementptr inbounds %struct.ST, %struct.ST* %3, i64 1
  ```

- `.Z`

  ```
   %5 = getelementptr inbounds %struct.ST, %struct.ST* %4, i32 0, i32 2
  ```

- `.B`

  ```
  %6 = getelementptr inbounds %struct.RT, %struct.RT* %5, i32 0, i32 1
  ```

- `[5][13]`

  ```
  %7 = getelementptr inbounds [10 x [20 x i32]], [10 x [20 x i32]]* %6, i64 0, i64 5
  %8 = getelementptr inbounds [20 x i32], [20 x i32]* %7, i64 0, i64 13
  
  %9 = load i32, i32* %8, align 4
  ```

  由于追后拿到的还是指针，所以需要 load 取出具体值



### API

在 rust 的 inkwell 中可以使用 `build_gep` 指令

```rust
// 分配一块 [3 x i32] 数组
let array_alloca = builder.build_alloca(i32_type.array_type(3), "array_alloca");

unsafe {
    builder.build_gep(array_alloca, &[i32_type.const_int(2, false)], "a_ptr");
}
```



## 结构体

### 构建结构体

```rust
// {
//	i32
//	i32
// }

//声明域
let field_types = &[i32_type.into(), i32_type.into()];
// 构建结构体
let struct_type = context.struct_type(field_types, false);
```



### 取结构体的值

- #### 如果传入指针

  > struct 也可以使用 gep

  ```rust
  let struct_alloca = builder.build_alloca(struct_type, "struct_alloca");
  builder.build_struct_gep(struct_alloca, 0, "field_0").unwrap();	//如果越界或者指针有误会报错
  ```

- #### 如果传入结构体本身

  > 对于结构体还是数组，都统一用 `build_extract_value`
  >
  > 因为在LLVM IR中，数组或者结构体是存储在**虚拟寄存器**中的值类型，因此不能对数组的元素进行 GEP
  >
  > 当然，如果把数组存储在内存的某个地方，并且有一个指向数组的指针，这时就可以使用GEP，然后再进行加载或存储。
  >
  > extract_element，extract_value 和 gep 并不访问内存，只是对虚拟寄存器进行访问。
  >
  > insert_value 和 extract_value 的索引必须是常数。
  
  ```rust
  // 构建 StructValue
  let struct_ = builder
          .build_load(struct_alloca, "struct")
          .into_struct_value();
  
  // 插入值
  builder.build_insert_value(struct_, const_zero, 0, "filed_0");
  // 取值
  builder.build_extract_value(struct_, 0, "name");
  ```
  
  

## 数组

### 构建数组

```rust
// 数组指针，局部变量需要分配内存
let array_alloca = builder.build_alloca(i32_type.array_type(3), "array_alloca");

// 数组值
let array = builder.build_load(array_alloca, "array").into_array_value();
```

### 查看基本属性

```rust
array.len();	// 数组长度
array.get_element_type();	// 数组元素类型
```

### 取数组值

> 注意此处有两种方式：
>
> - in_bounds_gep：有了inbounds关键字，如果地址在实际的底层分配对象之外，那么GEP的结果值是 undef
> - gep：如果没有inbounds关键字，对计算界外地址没有任何限制。很明显，执行加载或存储需要一个分配好的、充分对齐的内存地址。但是GEP本身只关心计算地址。
>
> 索引参数不需要是常量

```rust
// 指针取值

unsafe {
    let a0_ptr = builder.build_gep(array_alloca, &[i32_type.const_int(0, false)], "a0_ptr");
   	let a0_ptr =  builder.build_in_bounds_gep(array_alloca, &[i32_type.const_int(0, false)], "a0_ptr");
}

// 值取值
let i32_value = builder.build_extract_value(array, 0, "extract").unwrap();
```



## Vector

### 构建 vector

```rust
let vec_type = i32_type.vec_type(3);
// 指针
let vec_alloca = builder.build_alloca(vec_type, "vec_alloca");
// 具体值
let vec_value = builder.build_load(vec_alloca, "vec_val").into_vector_value();
```

### 取值

> extract_element 是专门针对 vector 的，返回在索引处的标量值

```rust
// 取到值后是 BasicValueEnum, 如果之后要使用必须 into 转换, 当然也可以不转保留 BasicValue 属性
let vec_ele1 = builder
        .build_extract_element(vec_value, const_int1, "vec_ele1")
        .into_int_value();

// 始终注意使用 gep 得到的是指针，因此取值必须再次使用 load
let vec_ele1_ptr = unsafe {
    builder.build_gep(vec_alloca, &[const_int1], "vec_ele1_ptr")
}
builder.build_load(vec_ele1_ptr, "vec_ele1");
```





## if-else

if-else 设计对两个 bb 的条件选择，选择的结果依赖于 if 的条件。

假设我们有一个代码，我们为其创建 ir

```c
// Test.c
int Test(int a)
{
    int b;
    if (a > 33){
        b = 66;
    }else{
        b = 77;
    }
    return b;
}
```

仔细观察我们可以发现，为上面这个函数建立 IR 需要这些步骤

> 我们忽略对环境的初始化，因为那会增大篇幅。同时 `i32_type` 也使用之前的定义

- 建立函数

  ```rust
  // 建立函数
  let param_type = vec![i32_type.into()];
  let fn_type = i32_type.fn_type(&param_type, false);
  // int Tets(int a)
  let function = module.add_function("Test", fn_type, None);
  
  // 将第一个参数名称设置为 a
  let mut param1 = function.get_nth_param(0)
  param1.unwrap().set_name("a");
  
  // 在函数中建立基本块，目前他们是没有前驱后继关系的
  let entry_block = context.append_basic_block(function, "entry");
  let then_block = context.append_basic_block(function, "if.then");
  let else_block = context.append_basic_block(function, "if.else");
  let return_block = context.append_basic_block(function, "ret");
  ```

- 申请局部变量 b

  ```rust
  // int b
  builder.position_at_end(entry_block);
  let b_ptr = builder.build_alloca(i32_type, "b_ptr");
  ```

- 判断 a

  ```rust
  // if (a > 33)
  let a = function.get_nth_param(0).unwrap().into_int_value();	// 取得整数 a
  let const_33 = i32_type.const_int(33, false);
  
  // Int SGT : 整数比较 带符号比较 great than
  let cmp = builder.build_int_compare(IntPredicate::SGT, a, const_33, "cmp.result");
  // 建立基本块之间的关系
  builder.build_conditional_branch(cmp, then_block, else_block);
  ```

- 若条件为真

  ```rust
  // b = 66
  // 将 builder 指针指向 then 块
  builder.position_at_end(then_block);
  builder.build_store(b_ptr, i32_type.const_int(66, false));
  // 建立无条件跳转
  builder.build_unconditional_branch(return_block);
  ```

- 若条件为假

  ```rust
  // b = 77
  // 将 builder 指针指向 then 块
  builder.position_at_end(else_block);
  builder.build_store(b_ptr, i32_type.const_int(77, false));
  // 建立无条件跳转
  builder.build_unconditional_branch(return_block);
  ```

- 返回 b

  ```rust
  builder.position_at_end(return_block);
  // 读取局部变量的值，将其返回
  let b = builder.build_load(b_ptr, "b");
  builder.build_return(Some(&b));
  ```



完成以上步骤后，我们将 llvm ir 打印到文件中

```rust
module.print_to_file("out.ll").unwrap();
```

 

编译运行，查看文件可得

```
; ModuleID = 'test'
source_filename = "test"

define i32 @Test(i32 %a) {
entry:
  %b_ptr = alloca i32, align 4
  %cmp.result = icmp sgt i32 %a, 33
  br i1 %cmp.result, label %if.then, label %if.else

if.then:                                          ; preds = %entry
  store i32 66, i32* %b_ptr, align 4
  br label %ret

if.else:                                          ; preds = %entry
  store i32 77, i32* %b_ptr, align 4
  br label %ret

ret:                                              ; preds = %if.else, %if.then
  %b = load i32, i32* %b_ptr, align 4
  ret i32 %b
}
```



## if-else-phi

在很多情况下，控制流只是为了给某一个变量赋值，而phi 指令，则可以根据控制流来选择合适的值

> 比如说上面的例子，一个 if-else 语句包含了一个条件判断以及两个逻辑分支。最终会运行哪个分支的代码，取决于条件判断的结果为真还是假。而 “条件” 则一般是一个比较表达式
>
> ```assembly
> %value = phi i32 [66, %branch1], [77, %branch2]
> ```

可以看到 phi 指令可以接收多个输入参数，参数的个数也不是固定的。

- 第一个参数表示的是 phi 指令的返回值类型，如在以上示例中为 i32。

- 接下来的每一个参数都是一个数组，代表了每一个分支及其对应的返回值。

  > 例如，如果前一步执行的是 branch1 分支，则返回值为 66；当执行的是 branch2，则返回值为 77；



仍然以之前的 if 为例子

其他语句不变：

- 若条件为真

  ```rust
  builder.position_at_end(then_block);
  let const_66 = i32_type.const_int(66, false);
  builder.build_unconditional_branch(return_block);
  ```

- 若条件为假

  ```rust
  builder.position_at_end(then_block);
  let const_66 = i32_type.const_int(77, false);
  builder.build_unconditional_branch(return_block);
  ```

- 返回 b

  注意 phi 需要把到来值都转化为 basic_value。而取值时也是先转化为 basic_value，然后转为具体类型

  ```rust
  // 建立局部变量，它来自之前的两个 bb。
  // 参数规定来值是 i32_type
  let phi = builder.build_phi(i32_type, "phi_node");
  // 它来自 then_block 和 else_block
  phi.add_incoming(&[
      (&const_66.as_basic_value_enum(), then_block),
      (&const_77.as_basic_value_enum(), else_block),
  ]);
  
  // 读取值，将其返回
  let phi_value = phi.as_basic_value().into_int_value();
  builder.build_return(Some(&phi_value));
  ```



## for loop

假设有如下源码，我们为其生成 IR

```c
// 其中 b 是局部变量
for (int i = 0; i < a; i++){
    b = b + i;
}
```

可以看出，for 循环主要是由 for 中的语句，以及循环体构成，我们只需要为其分别生成 BB，然后用 label 表达控制流关系即可：

- `for (int i = 0; ...)`

  ```rust
  // 显然，这是一个局部变量的声明
  builder.position_at_end(for_entry_block);
  let i_ptr = builder.build_alloca(i32_type, "i_ptr");
  builder.build_store(i_ptr, i32_type.const_zero());
  // 无条件到达 cond
  builder.build_unconditional_branch(for_cond_block);
  ```

- `for (... i < a; ...)`

  ```rust
  builder.position_at_end(for_cond_block);
  let i = builder.build_load(i_ptr, "i").into_int_value();
  let a = function.get_nth_param(0).unwrap().into_int_value();
  let cond = builder.build_int_compare(IntPredicate::SLT, i, a, "cond.res");
  // 条件跳转
  builder.build_conditional_branch(cond, for_body_block, return_block);
  ```

- `for (... i++)`

  ```rust
  builder.position_at_end(for_action_block);
  let i = builder.build_load(i_ptr, "i").into_int_value();
  let i_inc = builder.build_int_add(i, i32_type.const_int(1, false), "i.inc");
  builder.build_store(i_ptr, i_inc);
  // 无条件到达 cond
  builder.build_unconditional_branch(for_cond_block);
  ```

- `b = b + i`

  ```rust
   builder.position_at_end(for_body_block);
  let b = builder.build_load(i_ptr, "b").into_int_value();
  let i = builder.build_load(i_ptr, "i").into_int_value();
  let add_res = builder.build_int_add(b, i, "add.res");
  builder.build_store(b_ptr, add_res);
  // 无条件到达 cond
  builder.build_unconditional_branch(for_action_block);
  ```

- 其他代码参考 `if-else`



## 可视化

llvm 没有自带的可视化工具，但是可以依赖 graphviz 来辅助可视化

```shell
# 生成函数调用图，该指令会生成一个 dot 文件，每一个元素都是一个函数
$ opt -dot-callgraph test.ll

# 生成控制流图, 该指令会生成若干个 dot 文件，每个文件都是单个函数内的控制流图
$ opt -dot-cfg test.ll

# 根据 dot 绘图
$ dot xxx.dot -Tpng -o xxx.png
```

以上面的 for loop 为例，png 如下：

<img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/cc.png" alt="cc" style="zoom: 80%;" />



## 调用 C API

假设我们要在 llvm 中生成调用 C 中的 `printf` 的IR，如何使用 llvm 生成指令？

首先构建常量字符串

```rust
let string = "hello world %d\n";
let ty = context.i8_type().array_type(string.len() as u32);
let gv = module.add_global(ty, Some(AddressSpace::Generic), "const_str");
gv.set_linkage(Linkage::Internal);

// 需要注意的是传入的 &u[8] 不是以 \0 结尾的
gv.set_initializer(&context.const_string(string.as_ref(), false));

// 将其转换为指针，作为参数传入 printf 中
let str_type = context.i8_type().ptr_type(AddressSpace::Generic);
let pointer_value = builder.build_pointer_cast(gv.as_pointer_value(),str_type,"hello_str_ptr");
```

构建 `printf` 函数，我们只需要声明即可，因为 llvm 默认链接 libc

```rust
// int printf (const char * __fmt, ...)
// 由于该函数是变参，所以 is_var_args 设置为 true
let printf_type = i32_type.fn_type(&[str_type.into()], true);
// 声明此链接来自于外部：Linkage::External
let printf = module.add_function("printf", printf_type, Some(Linkage::External));

// call, 第一个参数是函数，第二个是参数，第三个是函数返回结果的名称
let three = i32_type.const_int(33, false);
builder.build_call(printf, &[pointer_value.into(), three.into()], "call_hello");
```





## 构建 lli 可执行程序

要想构建 `lli` 可执行程序其实很简单，我们只需要构建出 `main` 函数即可，因为 `lli` 解释器默认将 main 作为程序入口

```rust
// char**
let str_ptr_type = context
        .i8_type()
        .ptr_type(AddressSpace::Generic)
        .ptr_type(AddressSpace::Generic);

let main_fn_type = i32_type.fn_type(&[i32_type.into(), str_ptr_type.into()], false);
let main_fn = module.add_function("main", main_fn_type, None);
let param1 = main_fn.get_nth_param(0);
let param2 = main_fn.get_nth_param(1);
param1.unwrap().set_name("argc");
param2.unwrap().set_name("argv");
```

然后构建基本块，entry 和 ret 即可。可使用 `lli` 解释该程序。我们将之前的调用 printf 加入其中可得

```assembly
; ModuleID = 'test'
source_filename = "test"

@const_str = internal global [15 x i8] c"hello world %d\0A"

define i32 @main(i32 %argc, i8** %argv) {
entry:
  %call_hello = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @const_str, i32 0, i32 0), i32 33)
  br label %ret

ret:                                              ; preds = %entry
  ret i32 0
}

declare i32 @printf(i8*, ...)
```

解释执行：

```
$ lli test.ll
```

