### 语法

`trait` 类似于接口，可以为某个类型实现 `trait` 从而实现其中的方法，那么这个类型也就能够调用它的方法。传入 `trait` 类型的变量的方法有 3 种，分别是 `impl`、`&impl` 和 `&dyn`。不能够使用 `dyn` 因为他不能在编译期间获得大小

```rust
trait MyTrait {
    fn call(&self);
}

struct A {
    a: i32,
}

impl MyTrait for A {
    fn call(&self) {
        println!("A");
    }
}

fn test_trait1(p: impl MyTrait) {
    p.call();
}

fn test_trait2(p: &impl MyTrait) {
    p.call();
}

// compile error,
// dyn MyTrait doesn't have a size known at compile-time
fn test_trait3(p: dyn MyTrait) {
	p.call();
}

fn test_trait4(p: &dyn MyTrait) {
    p.call();
}

fn main() {
    let s1 = A { a: 1 };
    test_trait1(s1);
}
```



#### impl trait

对于这种用法我们可以传递引用（`&impl`）也可以不传递（`impl`），编译器实际上是为每一个类型各自生成一个函数，例如假如传入的参数类型为 `T` 和 `F`，他们都去调用 `call` 方法，则编译器实际上是 `impl T for A ` 和 `impl F for A`，即对应类型生成对应的函数，然后把生成的函数插入到调用的地方，这一切都是在编译器完成的，这就是所谓的静态派发。换句话说 `impl` 表达的就是静态生成代码。



##### 关联方法与 `trait`

这就引出一个问题：如果我们为一个类型既实现了 `trait` 中的方法，又为它实现了同名关联方法，那么最终调用的方法是哪一个

```rust
trait MyTrait {
    fn call(&self);
}

struct A {
    a: i32,
}

impl MyTrait for A {
    fn call(&self) {
        println!("trait");
    }
}

impl A{
    fn call2(&self) {
        println!("method");
    }
}

fn main() {
    let s1 = A { a: 1 };
    s1.call();	// -> method
}
```

答案是关联方法的优先级更高。关联方法将 `trait` 方法覆盖了



##### 多重 `trait`

另一个问题是，如果两个 `trait` 中有重名函数编译器会调用哪一个。答案是编译不通过，因为编译器找到了多个可调用了方法，此时必须明确告诉程序员否则可能会造成程序员难以理解的行为。

除非我们消除歧义性，将类型主动转换为某一个 `trait` ，此时必须使用引用且转换为 `&dyn trait`

```rust
struct A {
    a: i32,
}

trait MyTrait {
    fn call(&self);
}
trait MyTrait2 {
    fn call(&self);
}

impl MyTrait for A {
    fn call(&self) {
        println!("trait1");
    }
}

impl MyTrait2 for A {
    fn call(&self) {
        println!("trait2");
    }
}

fn main() {
    let s1 = A { a: 1 };
    s1.call();	// compile error
    
    <A as MyTrait>::call(&s1);	// -> trait1
    
    let s2 = &s1 as &dyn MyTrait;	// 手动 cast
    s2.call();	// -> trait1
    
    let s3: &dyn MyTrait2 = &s1;	// 手动标注类型
    s3.call();	// -> trait2
}
```



#### dyn trait

在 2021 版本的Rust中必须使用 `dyn` 来表示传入的是一个 `trait object`。之前的 Rust 每当 `&trait` 时自动将对象包装为 `trait object` 结构体，现在要在引用符号与对象之间加入 `dyn`。`&dyn trait` 整体就表示为一个 `trait object` 而不是它的引用。如下图的 `w` 就是一个 `trait object`，类型为 `&dyn trait`。

之所以要包装为一个 `trait object` ，是因为编译器要生成一份虚表然后通过虚表内的函数指针的方式间接调用，这就使得这个方法可以接受任何实现了该 `trait` 的对象，这些对象的大小不一，也就不可能在编译期间就知道大小，所以只能用 `trait object`的方式传递。

 `trait object`内存布局如下：它的 `data pointer` 字段指向原类型数据，`vtable pointer` 指向通用虚函数表结构。当一个 `trait` 对象取 `&dyn` 时就会生成出下面这个结构。在汇编层面上，每次调用 `trait` 关联方法都会先将一个标识符（标识符其实是一个偏移量，用于索引自己的函数入口）压栈，然后调用通用的一个特殊方法，由这个方法去寻址真正的调用方法。（`w` 即 `trait object`）。

需要注意的是 `dyn trait` 也是存在的（此处没有引用符号），它就是 `data pointer` 所指向的数据，只不过它的大小不一，随每个对象的内部布局而变化，因此才用 `&dyn trait` 

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171750854.png" alt="image-20220517175042778" style="zoom:50%;" />

```rust
 println!("{}", size_of::<&dyn MyTrait>())	// 16 (64位机器)

struct Trade(i32);
let s = Trade(0);
// size_of_val 可以知晓动态类型T的大小, 只需要传入一个 &T 即可
println!("{}", size_of_val(&s));	// -> 4
```

使用 Box 封装编译期不知道大小的类型

```rust
struct Trade1(i32);
struct Trade2(i32, i32);

impl Test for Trade1 {
    fn test(&self) {
        todo!()
    }
}

impl Test for Trade2 {
    fn test(&self) {
        todo!()
    }
}

let s = Trade2(0, 0);
println!("{}", size_of_val(&s));	// -> 8

let k1: Box<dyn Test> = Box::new(Trade1(0));
let k2: Box<dyn Test> = Box::new(Trade2(0, 0));
```



之前三种只是最基本的用法，其实他们的内涵是相同的，就是必须在编译期间知道大小，我们也可以用如下方式来使用 `trait`

```rust
fn test_trait(p: &mut dyn MyTrait)
fn test_trait(p: Box<dyn MyTrait>)
fn test_trait(p: Arc<dyn MyTrait>)
fn test_trait(p: Rc<dyn MyTrait>)
...
```





### 对象安全

**一个 trait 如果能实现自己，就认为它是对象安全的**

为什么必须是对象安全呢？

trait对象，在运行时已经擦除了类型信息，要通过虚表调用相应的方法。不像静态分发那样，trait对象不是为每个类型都实现trait的方法，而是只实现一个副本（自动为其实现自身），结合虚函数去调用。

现在想一个问题： 假如那个类型没有实现这个方法怎么办？
实际上，会有很多种情况下，会出现这个问题。运行时确定的类型和方法应该合法的，保证trait对象在运行时可以安全地调用相关的方法。

比如trait里有泛型函数。这就搞的很复杂了，可能运行时无法确定该调用哪个函数。反正是各种情况吧。所以，为了避免出现这种问题，官方引入了对象安全的概念。
实际上就是引入了一系列的规则，也就是上面列出的那些。编译器根据这些规则，在编译期判断一个你写的trait对象，是不是合法的。

比如：trait对象其实在内部维护两个表：safe_vtable和nonself_vtable，标记有where Self: Sized的会被归类到nonself_vtable，也就是说，不会被trait对象调用。
这样的话，方法标记有where Self: Sized的trait对象自然是安全的，因为这表示 这个方法 只能为 Self: Sized 都类型实现，是有条件的，所以在运行时有可能存在无效（万一有不是Sized的类型调用，就没有该方法）调用。

如果是合法的，则代表了，这个trait对象在运行时调用方法应该是没问题的。
不会出现没有实现，或者不知道该调用哪个的情况。
这就是对象安全的概念。它和内存安全并无直接关系。
所以，对象安全的本质就是为了让trait对象可以安全地调用相应的方法。

如果没有Sized的限定，那么就会很容易写出无用的类型。比如 Box，它用做trait对象即便会编译，但是不能用它做任何事情（后面有演示代码）。
对于更复杂的trait，往往就没有这么明显了，只有在做了大量繁重的工作之后可能会突然发现某个trait对象无法正常调用方法。
所以，为trait增加Sized限定，然后编译器自动为该trait实现自身，就可以在编译期准确排除无效的trait对象。
这就是对象安全。需要注意的是，对象安全和内存安全并无直接的关联，它只是保证trait对象在运行时可以安全准确地调用相关的方法。
