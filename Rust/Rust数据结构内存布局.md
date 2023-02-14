### 总则

在 Rust 中，编译器能否确定大小是一件非常重要的事情，只有编译期就能确定大小的数据结构才会放在栈上，反之就必须放在堆上



### 数组

数组必须在编译器就已知大小，这就要求编译器必须知道两件事情：1.单个元素的大小，2.数组长度。这也是为什么Rust数组必须是定长数组的原因

```rust
let a:[i32;3] = [1,2,3];

// 元素个数必须匹配，否则就会报错
// 为了方便也可以不写类型，让编译器自动推导
let a:[i32;4] = [1,2,3];	// compile error
let a = [1,2,3];		// 编译器推断 a 的类型为 [i32;3]
```

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171526750.png" alt="image-20220517152618622" style="zoom: 50%;" />





### 向量(Vec)

向量相当于是变长数组，由于编译期间大小不能确定，因此元素不能放在栈上，必须放在堆上，只在栈上放一个大小已知的结构体.

Vec结构体包含三个域，每一个域大小是一个机器字长（32位机器是4字节，64位机器是8字节）。

第一个域存放实际数组的首地址，第二个域存放数组的总容量（类型 `usize`），第三个域存放当前数组所存放元素的个数（类型 `usize`）。

```rust
let mut v: Vec<i32> = vec![4, 5, 6];		// v 指向的真实数据可变

println!("{}", size_of::<Vec<i32>>());	// -> 24 (64位机器)
```

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171528109.png" alt="image-20220517152818008" style="zoom: 50%;" />



### 切片（&[T]）

一般切片都是使用引用形式，因为切片和数组的区别在于不必指定大小，这使得非引用形式的切片编译期无法知道多少字节表示一个切片，因此无法保存在一个变量中,解决办法是用切片的引用。



切片引用是一个结构体，包含两个域，每一个域是一个机器字长，因此一共两个机器字长。



第一个域存放目标数组首地址。需要注意的是该首地址是可以变化的；第二个域存放切片长度。只要有了这两个变量就能够确定对某一个数组的唯一的视图。这种引用方式也称为胖指针，即存储了指针之外的一些信息。切片引用大小固定，可以存储在栈上。需要记住的是被引用的真实数据既可以在栈上（s1），也可以在堆上（s2）



必须注意如果要更改切片中某个元素，必须满足两个条件：1.真实数据原本就是可修改的（`mut`）;2.借用的时候采用可变借用（`&mut`）。只有可变数据才有可变借用。

```rust
// compile error:
// error[E0277]: the size for values of type `[{integer}]` cannot be known at compilation time
let s1:[i32] = a[0..2];

// it's ok
let s1:&[i32] = &a[0..2];		// 不可变借用
let s2:&[i32] = &mut v[0..2];	// 可变借用，前提是 v 可变

println!("{:#?}", size_of::<&[i32]>());	// 16 (64位机器)
```

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171553840.png" alt="image-20220517155356746" style="zoom:50%;" />





### 字符串

字符串其实也是一个`Vec<u8>` 类型的向量，内存布局与其相似。Rust对字符串唯一的限制是字符串必须用UTF-8编码。直接将字面量赋值给一个变量无法得到字符串，而是得到一个字符串切片的引用，这是因为字符串字面量会存储在 `.data` 节的只读数据区中，它是只读的，并且生命周期与整个程序的生命周期一样长（`'static`）

```rust
// String 是一个Vec
let s1: String = String::from("hello");

// &[T] 是一个胖指针
let s2: &str = "hello";
```

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171620172.png" alt="image-20220517162041077" style="zoom:50%;" />







### 结构体

Rust共有三种结构体：常规结构体、元组结构体、单元结构体。结构体内部元素依次排列，必要时会有填充

```rust
// 常规结构体
struct Data {
	nums: Vec<usize>,
	dim: (usize, usize),
}

// 元组结构体
struct Data (Vec<usize>, usize, usize);
println!("{:#?}", size_of::<Data>());	// -> 40

// 单元结构体，Rust不会为其分配内存
struct Data;
```

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171631294.png" alt="image-20220517163111204" style="zoom:50%;" />



### 枚举

枚举相当于一个 union，Rust会选择内部最长的变量的大小作为枚举的所占内存大小

```rust
enum HttpStatus{
	Ok = 1,
	NotFound = 255,
}

println!("{:#?}", size_of::<HttpStatus>());	// 1

enum HttpStatus{
	Ok = 1,
	NotFound = 256,	// 256至少需要两个字节
}
println!("{:#?}", size_of::<HttpStatus>());	// 2
```



而对于复杂的枚举，其长度也取决于最大的那个类型。

对于下面这个枚举，Empty不存储数据，Number存储一个 `i32`，占 4 字节，Array 存储一个 `Vec<i32>`，占24字节。选择24，再加上必须的8字节（后面解释），一共32字节。所以枚举可以放在栈上。**不过必须注意的是，并不一定会用 8 字节来表示成员个数**

```rust
enum Data {
    Empty,
    Number(i32),
    Array(Vec<i32>),
}

println!("{:#?}", size_of::<Data>());	// 32 (64位机器)
```

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171645717.png" alt="image-20220517164535624" style="zoom:50%;" />



前面 8 字节表示是哪一个成员，例如 0 代表 `Empty`, 1 代表 Number等等。在 8 字节之后就是成员存储区，例如 Array 在这24字节中存放了一个 `Vec`，完全用完；而 Number 在这 24 字节中存放了一个 `i32`，只用了 8 字节；以此类推



这也是为什么 `Option` 必须要给出类型的原因，否则的话无法计算该枚举所占的空间大小。至于一个 `Option` 具体大小，还与 `T` 长度的范围有关，甚至编译器优化后若 `Option<T>`  中 `T` 类型为智能指针，由于智能指针不为 0，所以只需要一个 8 字节就可以存放该 `Option`。解析时若 `Option` 为 0 则表示 `Node`，否则表示 `Some(pointer)` 

```rust
enum Option {
	None,
	Some(T),
}

Option<Box<i32>>;
```

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171657415.png" alt="image-20220517165720323" style="zoom:50%;" />





### Rc和Arc

#### Rc

`Rc`是一个指针，所以它的大小是固定的，64位机器则 8 字节，32位机器则 4 字节。它所指向的是一个结构体，该结构体第一部分是一个引用计数器，第二部分才是类型为 `T` 的数据

```rust
let v: Rc<Vec<String>> = Rc::new(vec![
    String::from("Odin"),
    String::from("Thor"),
    String::from("Loki"),
])

println!("{:#?}", size_of::<Rc<Vec<String>>>());	// 8(64位机器)

// 可以看出Rc的作用就是给 Vec<String> 的前部安了一个 ref count
```

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171710468.png" alt="image-20220517171049364" style="zoom:50%;" />



这是就允许两个指针指向同一个对象，而不是像之前的所有权模型。只有当所有指针都失效时，具体指向的数据才会析构

对`Rc`的`clone`只会对栈上的指针进行一次按位复制，并且把引用计数加1。

```rust
let v2 = v.clone()
```

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171715660.png" alt="image-20220517171544563" style="zoom:50%;" />



#### Arc

如果一个类型实现了 `Send`，则该类型的数据可以在线程之间传递；如果一个类型实现了 `Sync`，则该类型的数据可以在线程之间共享。

`Rc`没有实现 `Send` 和 `Sync`。所以在两个线程共享同一个数据时，若两个线程同时 `clone` ，则他们都试图去增加引用计数，这会产生数据竞争

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171723898.png" alt="image-20220517172318812" style="zoom:50%;" />

要避免这种情形，需要使用 `atomic Rc` 即 `Arc`。`Arc` 和 `Rc` 的作用几乎相同，唯一的区别在于 `Arc` 会消除引用计数的数据竞争，可以在多线程之间安全地共享。

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171724495.png" alt="image-20220517172438416" style="zoom:50%;" />



默认的`Arc`只允许改变引用计数，不允许改变数据本身，若要获取这个功能，需要 `Mutex` ，他会允许访问的数据被改变，者可以用于多线程之间地互斥锁。

```rust
let lock: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
```



### Cell和RefCell

#### Cell

`Cell` 提供了一个变量的内部可变性。一般把 `Cell` 用在结构体的内部。



内部可变性：

变量的可变性继承自包含这个变量的容器，比如下面的 `s.name` 可变是因为包含它的容器s被声明为可变变量

```rust
let mut s = Foo {name: "22"};
s.name = "33";
```

而有的时候，我们不希望把整个结构体设置为可变或者不可变，这时候就需要更加细粒度的控制方式：

```rust
use std::cell::Cell;
struct SomeStruct {
    regular_field: u8,
    special_field: Cell<u8>,
}
let my_struct = SomeStruct {
    regular_field: 0,
    special_field: Cell::new(1),
};

my_struct.regular_field = 100;	// error：my_struct 是不可变的
my_struct.special_field.set(new_value);	// compile ok
```

上面的示例中虽然 `my_struct` 是不可变的，但 `Cell` 修饰了 `special_field`，因此这个域是可变的。所以说 `Cell` 本质上就相当于一个提供了可变性的变量，分配在栈上

![image-20220519134824728](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205191348762.png)

```rust
println!("{:#?}", size_of::<Cell<()>>());	// -> 0
println!("{:#?}", size_of::<Cell<i8>>());	// -> 1
println!("{:#?}", size_of::<Cell<i64>>());	// -> 8
println!("{:#?}", size_of::<Cell<String>>());	// -> 24
```



需要注意的是，`Cell<T>`  中 `T` 必须实现 `Copy`，否则无法 `get()`。这很好理解，没有实现 `Copy` 就只能 `move` 了，移动之后 `Cell` 内部就没有值，这是不允许的

```rust
let c = Cell::new(String::from("asdf"));
let one = c.get();	// compile error: doesn't satisfy `String: Copy`
```





#### RefCell



```rust
pub struct RefCell<T: ?Sized> {
    borrow: Cell<BorrowFlag>,
    value: UnsafeCell<T>,
}
```

`Cell<T>`  中 `T` 必须实现 `Copy`，否则无法 `get()`。但是 `RefCell` 不必考虑这一点，即使没有实现 `Copy` 也能存放，但唯一的限制在于不能存在多个可变、可变与不可变的情形：

```rust
let s = RefCell::new(String::from("hello, world"));
let s1 = s.borrow();
let s2 = s.borrow_mut();	// runtime error

println!("{},{}", s1, s2);	
```



`RefCell` 相比于 `Cell`，多了一个借用标记 `BorrowFlag`。它占 8 个字节。不过有趣的是，它的数据 `T` 是按照 8 字节对齐的。

```rust
println!("{:#?}", size_of::<RefCell<()>>());	// -> 8, 即 () 未分配空间
println!("{:#?}", size_of::<RefCell<i8>>());	// -> 16
println!("{:#?}", size_of::<RefCell<i64>>());	// -> 16
println!("{:#?}", size_of::<RefCell<i128>>());	// -> 24, 即 8 + 16


// 4 + 1
struct Point {
    y: i32,
    z: i8,
}
println!("{:#?}", size_of::<RefCell<Point>>());		// -> 16, 即 8 + 8
```



![image-20220519134851531](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205191348566.png)







### trait对象

为对象加一个 `trait` 方法：

```rust
use std::io::Write;

let mut buf: Vec<u8> = vec![];
// w 具有 Write 方法
let w: &mut dyn Write = &mut buf;

w.write(b"abcdefg");	// 前面加 b 使得 str 变成 u8 序列
println!("{:#?}", buf);
```

`w`具有两个域，而非平常的单独一个 `pointer` 指针指向 `buf`，如下图所示。

`vtable` 在编译期间生成，指向实现该 `trait` 必须实现的方法，例如 `write()`、`write_all()`和`flush()`.由于 `dyn Write` 也是动态类型大小，所以我们也用它的引用。即下图的 `vtable pointer`

`Vec<u8>` 转换为 `dyn Write` 是合法的，因为 `Vec<u8>` 时实现了 `Write` trait

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171750854.png" alt="image-20220517175042778" style="zoom:50%;" />



```rust
println!("{:#?}", size_of::<&mut dyn Write>());	// 16(64位机器)
```



这种实现与 C++ 的实现有一些区别，C++ 是直接在类的内存布局中（通常是起始处）添加一个 `vatble` 指针，用于指向虚表，换句话说 `vtable` 是和其他数据凑在一起的。而 Rust 则是将数据字段和虚表字段完全分隔开并且用两个指针连接两个内存区域，这就使得 `trait` 对象的大小编译期完全可以确定下来。





### 闭包

Rust中有三种函数：普通函数，关联函数和方法。其中关联函数和方法都和结构体绑定，关联函数类似于OO语言的静态方法，与结构体实例无关,调用方式是 `::`；方法类似于OO语言的成员函数，第一个参数是结构体实例，调用方式是 `.`。

```rust
struct A(i32);
impl A {
	fn add(a: i32) -> i32 {
		a + 1
	}
}

A::add(5)	// -> 6
```



Rust中闭包的实现其实并没有引入特殊的实现，而只是一种编译器的语法糖，这就意味着我们可以手动模拟闭包的实现。

众所周知，闭包会可能捕获环境变量，也可能修改环境变量，综合来看有以下三种处理：

使用提示: 自定义闭包特性尚未稳定，因此需要做以下两个工作：

1. 将 rust 切换到 `nightly`

```shell
$rustup default nightly
```

2. 然后设定特性门

```
#![feature(unboxed_closures, fn_traits)]
```

需要特别留心的是如果闭包没有使用 `move` 关键字默认是捕获引用，使用了 `move` 后则获得所有权，环境变量从而拥有了和闭包一样的生命周期。



#### 未捕获环境变量

```rust
struct Myclosure<T> {
    env: T,
}

impl<T> FnOnce<(i32,)> for Myclosure<T> {
    type Output = ();
    extern "rust-call" fn call_once(self, args: (i32,)) -> Self::Output {
        println!("hello {}", args.0);
    }
}

fn main() {
    let f = Myclosure { env: () };
    f.call_once((42,));
}
```

如上面的代码所示，闭包就是先定义一个结构体，然后为其实现 `trait`。其中 `FnOnce`接受一个泛型参数，它的类型是一个元组，每一个元素的类型写入其中（示例代码只接受一个 `i32` 参数，因此写为 `(i32,)`)，之所以类型是一个元组，是因为传入参数可能有多个。`FnOnce`  必须实现 `call_once` 方法，他会获取闭包结构体的所有权，即消耗一个闭包结构体，因此只能调用一次。同时在此实现中还需要定义返回值类型 ` type Output`，本例中没有返回值，因此为 `()`。在 `call_once` 方法中通过 `args` 获取传入参数。



在使用闭包时，由于未捕获环境变量，因此手动为其 `env` 域填入 `()`。这一步本来是在编译期由编译器完成的。然后在传入参数时将参数组成一个元组传入。`Myclosure`接受一个泛型 `T`，但在本章讨论的情况下最终 `T` 是 `()`，因为未捕获环境



#### 捕获且修改环境变量

```rust
let arr = [1,2,3];
let clos = |i|{
	a[0] = i;
	println!("hello {:?}", self.env);   
}
clos(43);
```

它的去糖之后的实现类似于下面这种:

```rust
struct Myclosure {
    env: [i32; 3],
}

impl FnOnce<(i32,)> for Myclosure {
    type Output = ();
    extern "rust-call" fn call_once(mut self, args: (i32,)) -> Self::Output {
        self.env[1] = args.0;			// 修改第 1 个参数
        println!("hello {:?}", self.env);
    }
}

impl FnMut<(i32,)> for Myclosure {
    extern "rust-call" fn call_mut(&mut self, args: (i32,)) -> Self::Output {
        self.env[0] = args.0;			// 修改第 0 个参数
        println!("hello {:?}", self.env);
    }
}

fn main() {
    let arr = [1, 2, 3];
    let mut clos = Myclosure { env: arr };
    clos.call_mut((43,));
    clos.call_once((42,));
}
```

这段代码难度特别高，一不小心就不会通过编译。

- 对于任何要修改环境变量的闭包，都必须用 `mut` 修饰 `self`.

- 如果要实现 `FnMut`，就必须先实现 `FnOnce`，而且两者的参数类型必须一致，代码中都是 `(i32,)`。这是因为 `FnMut<Args>` 继承自 `FnOnce<Args>`

  ```rust
  pub trait FnMut<Args>: FnOnce<Args>
  ```

- 由于要修改环境变量，因此闭包必须声明为 `mut`

- 一旦调用 `call_once` 后闭包就会被消耗掉，因此该方法只能调用一次，而 `call_mut` 能调用多次

- 其实可以注意到 `call_once` 和 `call_mut` 内部的实现其实不一样，但闭包调用的是 `call_mut`



#### 捕获但未修改环境变量

```rust
let arr = [1,2,3];
let clos = |i|{
	println!("hello {:?} and {}", self.env, i);   
}
clos(44);
```

它的去糖实现如下，相比于情况2，它的实现倒并未太复杂

```rust
struct Myclosure {
    env: [i32; 3],
}

impl FnOnce<(i32,)> for Myclosure {
    type Output = ();
    extern "rust-call" fn call_once(mut self, args: (i32,)) -> Self::Output {
        self.env[0] = args.0;
        println!("hello {:?}", self.env);
    }
}

impl FnMut<(i32,)> for Myclosure {
    extern "rust-call" fn call_mut(&mut self, args: (i32,)) -> Self::Output {
        self.env[1] = args.0;
        println!("hello {:?}", self.env);
    }
}

impl Fn<(i32,)> for Myclosure {
    extern "rust-call" fn call(&self, args: (i32,)) -> Self::Output {
        println!("hello {:?} and {}", self.env, args.0);
    }
}

fn main() {
    let arr = [1, 2, 3];
    let mut clos = Myclosure { env: arr };
    clos.call_mut((43,));
    clos.call((44,));
    clos.call_once((42,));
}
```

要点如下:

- 由于 `trait` 继承关系，要实现 `Fn` 就必须实现之前的 `FnOnce` 和 `FnMut`，并且泛型类型必须一致
- `Fn `必须实现 `call` 方法，有趣的是，它的 `FnOnce` 和 `FnMut` 不必和他实现一样，编译器实现的闭包调用的是 `call` 方法，而不会去调用另外两个方法。
- 由于不用修改环境变量，因此闭包不必声明为 `mut`，但这样就不能调用 `FnMut`。但是手动声明为 `mut` 也可以。注意的是虽然 `call_mut` 需要可变引用，但闭包不声明为 `mut` 也是可以通过编译的



#### 逃逸闭包

所谓逃逸闭包，即是能被函数返回，不会被函数完结而销毁的闭包就被称为逃逸闭包（一般而言函数完结放在其栈上的闭包也会被销毁，而Rust编译器可以检测出这些销毁。从而将闭包移动到堆内存上）

```rust
fn c_mut() -> impl FnMut(i32) -> i32 {
    let mut arr = [1, 2, 3];
    move |i| {
        arr[0] = arr[1] + arr[2];
        arr[0] + i
    }
}

fn main() {
    let mut clos = c_mut();
    println!("{}", clos(5));	// 10
}
```

这段代码的注意之处：

- 返回值语法是 `impl FnMut(i32) -> i32`，这是因为它捕获了环境变量而且修改了它，对应上面的第二种情况，因此用 `FnMut`，而且使用 `()` 而非 `<>`，编译器用它推断参数元组的类型（即后面的 `i` 的类型）
- 由于该闭包会返回，而局部变量 `arr` 在函数返回后便销毁，因此必须用 `move` 关键字令闭包获取 `arr` 的所有权，否则的话返回的闭包引用了栈上被销毁的变量
- 由于闭包捕获了环境变量并修改了它，因此闭包必须声明为可变的，如此才能够将其引用传递给 `FnMut` 的 `call_mut(&mut self, Args)`



##### 一个坑

我们知道，结构体的生命周期必须短于或等于其内部成员的生命周期，否则会发生报错。用到闭包就是闭包的生命周期必定不能大于环境变量的生命周期，这也是为什么下面的代码是错误的

```rust
fn c_mut() -> impl FnMut(&str) -> String {
    let mut s = String::from("hello");
    move |i| {
        s.push_str(i);
        s
    }
}
```

我们使用 `FnMut` 并且使用了 `move` 捕获环境变量 `s`.然而在闭包中我们最后将 `s` 的所有权移送到外部。这个实现是错误的，因为由于转移了闭包内环境变量的所有权，因此这个闭包只能被调用一次，再次调用时闭包内已经没有环境变量可供转移了，而 `FnMut` 必须支持多次调用 `call_mut`，这就导致矛盾.

可以将上述代码改为

```rust
fn c_mut() -> impl FnOnce(&str) -> String {
    let mut s = String::from("hello");
    move |i| {
        s.push_str(i);
        s
    }
}
```

闭包在销毁之前转移了捕获到的 `s` 的所有权



#### 唯一不可变借用

编译器捕获变量有以下四种方式，它们的优先级依次递减：

- 不可变借用(immutable borrow)

- 唯一不可变借用(unique immutable borrow)

- 可变借用(mutable borrow)

- 移动(move)

  

**唯一不可变借用**是一种特殊的借用捕获，这种借用不能在语言的其他任何地方使用，也不能显式地写出。唯一不可变借用发生在修改可变引用的引用对象(referent)时（即解引用一个 `mut` 对象时）

```rust
let mut b = [1, 2, 3];
let x = &mut b;
{
    let mut c = || {
        (*x)[0] = 4;
    };
    let y = &x; // compile error
    c();
}
let z = &x;
```

上面代码中 `c()` 调用必须存在，否则不会触发报错。这里闭包使用了**唯一不可变借用**：它采用了不可变的方式借用了 `x`，但是又拒绝其他变量再次借用被它捕获的变量（即 `x` ）.

