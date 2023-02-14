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

编译器捕获变量有以下四种方式，它们的优先级依次递减：

- 不可变借用(immutable borrow)

- 唯一不可变借用(unique immutable borrow)

- 可变借用(mutable borrow)

- 移动(move)

一旦闭包被定义，那么由该闭包所捕获的环境变量就已经被锁定了，也就是说其他变量再借用它是有规则的（不能一写多读，不能多写）。但是到闭包最后一次调用后就可以再次引用（编译器会在合适的地方插入闭包的 `drop` ）：

```rust
let mut color = String::from("green");
let print = || println!("`color`: {}", color);	// 不可变借用

// let ref mut _reborrow = color;	-> compile error, 因为是可变借用
// let _color_moved = color;		-> compile error，因为是移动

let ref _reborrow = color;			// -> compile ok!
print();
print();

let _color_moved = color;			// -> compile ok!
```



Rust中闭包的实现其实并没有引入特殊的实现，而只是一种编译器的语法糖，这就意味着我们可以手动模拟闭包的实现。

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



众所周知，闭包会可能捕获环境变量，也可能修改环境变量，综合来看有以下三种处理：

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