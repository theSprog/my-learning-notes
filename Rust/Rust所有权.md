Rust中一份资源只能有一个所有者，由资源所有者完成资源的析构



### Copy与Move

实现了 `Copy` trait 的类型在赋值时就是复制语义，否则就是移动（`move`）语义。在 `Copy` 时会复制一份资源，而移动时会销毁原资源而创建一个新资源



按照经验原则，一般动态增长的类型就是 `move`，因为这种一般都是一个指针指向堆上的资源，如果实现 `Copy` 就会导致两个指针同时指向堆内存。而固定大小的资源一般分配在栈上的，这种就可以实现 `Copy` trait



Rust 中的引用其实就是一个安全指针，它是一根指针但必须保证安全。

```rust
let mut a = String::from("hello");
let r = &mut a;
let d = *r;	// compile error
```

上述代码是非法的，因为 `*r` 就是 `a`，但 `a` 没有实现 `Copy` trait，所以只能是移动语义。如果编译器允许它通过的话，那么 `r` 所指向的 `a:String` 就由于移动而无效了，换句话说 `r` 指向了一个非法地址，这是不允许的。

如下图所示，如编译器允许通过则 `a` 内存非法，`d` 是一个`String`结构体（`Vec<u8>`）,它的成员包含一个指针指向堆内存上的连续字符

![image-20220518224312155](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205182243229.png)



然而下面的代码确实可以通过，我们修改了 `*r` 的内容，这是因为我们声明 `a` 是可变的，如果是不可变的那自然 `a` 不可以被修改

```rust
let mut a = String::from("hello");
let r = &mut a;
*r = String::from("hello2");
println!("{}", a);		// hello2
```



#### Copy与Clone

发生 `Copy` 时所有权不会移动。`Copy` 虽然是编译器实现的按位复制，但是不一定是发生在栈上。

```rust
let a = Box::new(RefCell::new(1));
let b = Box::new(RefCell::new(2));
*(b.borrow_mut()) = *(a.borrow());
println!("{:?} {:?}", a, b);	// RefCell { value: 1 } RefCell { value: 1 }
```

如上所示，两个 `RefCell` 都在堆上，而在栈上保留两个指针 `a` 和 `b`，但是第三行的复制却发生在堆上，而非栈上



##### 基本类型

所有的基本类型，像integer，float、`'static str`和`char`都是`Copy`类型。`String` 是没有 `Copy` 的。

`&T` 实现了 `Copy` 因为反正它也是不可变引用，允许存在多个不可变引用。



##### 结构体和枚举

结构体（struct）、枚举（enum）默认不是`Copy`但是你可以派生(derive)自一个`Copy` trait。

**ps：单元结构体和元组结构体也不会实现 `Copy`**

```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
enum SignedOrUnsignedInt {
    Signed(i32),
    Unsigned(u32),
}
```

有意思的是，这种操作必须满足两个条件：

- 必须成员都实现 `Copy` 才能给结构体派生 `Copy`
- 派生 `Copy` 也必须派生 `Clone`。因为 `pub trait Cpoy: Clone {}`。这也启发我们，可以给一个对象实现 `Clone` 而不实现 `Copy`，从而保留编译器的默认 `Copy`行为（即按位复制）

`Copy` 只是一个标记，用户无法实现该 trait，因为它是只能给编译器使用，即按位复制 ；用户的复制工作其实是交给 `clone` 完成。当结构体成员都实现了 `Copy` 时如果进行复制操作，结构体会自动调用每一个成员的 `Copy` 来进行复制操作，也就是按位复制。当然你也可以手动完成这个工作来达到定制化需求，这就是 `Clone`（调用时必须手动指定`clone()`而不是依靠编译器隐式调用，**编译器的`Copy`唯一会做的只有按位复制**（你看这个编编它就是逊啦~~））：

```rust
struct Point {
    x: i32,
    y: i32,
}

// Copy 只是一个标记
impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Point {
        x: 10,
        y: 20,
    }
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a;			// Copy，编译器调用
    let c = a.clone();	// Clone，用户调用
    println!("{} {}", a.x, a.y);	// 1, 2
    println!("{} {}", b.x, b.y);	// 1, 2
    println!("{} {}", c.x, c.y);	// 10, 20
}

```

`clone`方法不总是会创建一个深拷贝，类型可以以任意想要的方式自由实现`clone`，换句话说你可以以任意姿势实现 `Clone` （比如上面代码我们永远返回一个固定的 `Point`）。但是语义上，它应该足够接近复制一个对象的含义。



一般如果我们要在 `Clone` 中也实现按位复制，就需要也实现 `Copy` 然后解引用返回

```rust
impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}

impl Copy for Point {}
```

上面的解引用返回，如果你没有实现 `Copy`，解引用之后就会发生 `move`。而如果你实现了 `Copy`，编译器解引用后发现你的 `Point` 是复制语义，就会按位复制，而非移动所有权



##### 数组和元组

只有当每一个成员都实现了 `Copy` 编译器才会默认为其实现 `Copy`



##### 可变引用

不可变引用实现了 `Copy`，即允许对一个对象存在多个不可变引用。



##### 不可变引用

可变引用没有实现 `Copy`，他只能 `move`。很好理解，因为若它实现了 `Copy` 则由于按位复制那么此时就会出现两个对某个对象的可变引用，这是不允许的。







### move

#### 解引用移动

所谓解引用移动，本质上就是将一个引用解开，而其引用的数据又没有实现 `Copy` 时，就会触发移动行为。能够解引用移动的在 Rust中只有 `Box`

```rust
let s = Box::new(String::from("hello"));
let s2 = *s;			// value moved here
println!("{:?}", s);	// compile error
```

上述代码中 `s` 是一个指针，将其解引用后拿到存储在堆上的 `String`，而它又没有实现 `Copy` 因此发生移动行为。



#### move的本质

一个变量被`move`本质就是编译器将他重新设置为**未初始化状态**。但是这个说法也不严谨，这种未初始化状态其实已经绑定过一个值了，所以不能在绑定另一个值，因此不是完全意义上的未初始化。

```rust
let a = String::from("hello");
let b = a; // move

a = "123".to_string();	// compile error
println!("{}", a);
```

上面代码中如果 `a` 真的变成未初始化，那么之后的赋值应该是成功的，但编译器拒绝重复赋值（cannot assign twice to immutable variable），如果将 `a` 设置为 `mut1` 修饰，那么它又可以通过编译



#### move与析构

##### 普通析构

当一个变量发生 `move` 行为后，析构资源的责任也转移到了接受资源的那个变量手中。总体来说，先定义的变量先析构，后定义的变量后析构

```rust
struct Point {
    y: i32,
    z: i8,
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("drop {} {}", self.y, self.z);
    }
}

fn main() {
    let a = Point { y: 1, z: 2 };
    let b = Point { y: 3, z: 4 };

    let c = a;
}

// 输出
// drop 1 2
// drop 3 4
```

可以看出，由于一次所有权的转移，后定义的 `c` 先析构，之后才是 `b` 析构，而 `a` 则不会析构，因为他已经被 `move` 给 `c` 了



##### 元组析构

在元组中定义的成员变量析构顺序是 `FIFO`，因此先定义的成员先析构

```rust
let a = (Point { y: 1, z: 2 }, Point { y: 3, z: 4 });	

// drop 1 2
// drop 3 4
```



但如果元组中如果存在 `painc!()`， 则会析构顺序会倒转，并且 `panic!()` 之后的数据不会析构

```rust
let a = (Point { y: 1, z: 2 }, Point { y: 3, z: 4 }, panic!());
// drop 3 4
// drop 1 2

let a = (Point { y: 1, z: 2 }, panic!(), Point { y: 3, z: 4 });
// drop 1 2
```



##### 结构体

结构体的析构顺序是 ：1. 析构结构体自身，2. 按定义成员顺序析构成员

```rust
struct PrintDrop(&'static str);

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0)
    }
}

struct Foo {
    bar: PrintDrop,
    baz: PrintDrop,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo")
    }
}

fn main() {
    let foo = Foo {
        bar: PrintDrop("bar"),
        baz: PrintDrop("baz"),
    };
}

// 输出
// Dropping Foo		-> 析构自身
// Dropping bar		-> 先定义的成员先析构
// Dropping baz		-> 后定义的成员后析构
```



##### 闭包

闭包如果按所有权捕获了变量，则析构顺序是变量的捕获顺序，而不是变量的定义顺序

```rust
struct PrintDrop(&'static str);
impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0)
    }
}

fn main() {
    let z = PrintDrop("z");
    let x = PrintDrop("x")
    let y = PrintDrop("y");
    let closure = move || {
        x;
        y;
        z;
    };
}

// 输出
// Dropping x
// Dropping y
// Dropping z
```

虽然变量定义顺序是 `z -> x -> y`，但是捕获顺序是 `x -> y -> z`。这是因为闭包生成的匿名结构体成员是按照捕获顺序存放的