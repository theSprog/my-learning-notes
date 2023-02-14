### 非词法作用域（NLL）

从外观入手，下面这个代码会提示错误，因为在存在可变借用 `vv` 的同时 `println!()` 又尝试不可变借用 `v`：

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let vv = &mut v;
    println!("{:?}", v);
    vv;
}
```

但是将 `vv` 注释就会编译通过

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let vv = &mut v;
    println!("{:?}", v);
    // vv;
}
```



这里的问题在于其实第二段代码也不应该编译通过的，因为 `vv` 可变借用按理来说应该一直到 `}` 时才结束，而这期间又不可变借用了 `v`，因此应该报错，但 Rust 编译器非常智能得识别出了你并未在之后使用此 `vv` 变量，因此它的生命周期在 `let` 语句结束时就结束了，所以你才能在 `println!()` 中继续使用 `v`。换句话说编译器更精准地识别出了变量真正的生命周期。

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let vv = &mut v;		// ------> vv 生命周期开始
    vv;						// ------> vv 生命周期结束
    println!("{:?}", v);

}
```



这个东西的实现是由非词法作用域实现的。传统的词法作用域就是作用域由静态上下文确定，但这种确定作用域的方法的缺点就是粒度太大，有些借用明明已经结束，但由于还未到词法作用域 `}` 处，所以编译器还是认为该借用存在效果，进而对后面的代码产生约束。

而有了非词法作用域（NLL）后，对借用的具体作用范围有了更精细的控制，也进一步放宽了用户的创作边界。





### 生命周期参数

只需要注意这一点，生命周期的最大的作用就是**避免出现悬垂指针**，换句话说Rust在编译器就检查出悬垂指针并将其扼杀在摇篮中。



#### 函数

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

在上面的代码中，编译器能够立马的出的生命周期信息是：

1. `string1` 的生命周期是到 `main` 的  `}` 结束
2. `string2` 引用的值（即原 `str` ）的生命周期是静态的，也就是说会一直存在

但问题在于 `result` 所引用的值（不是 `result`,  它的生命周期到 `}` 也结束了）的生命周期是什么，编译器无法得知，因为它是使用一个函数来得出的，而这个函数究竟会返回什么编译器不知道，因此就必须通过一种手段告诉编译器，这种手段就是在函数上标注生命周期

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

生命周期 `'a` 是由编译器推断出来的：当编译器看到`x: &'a str`的时候，`'a`会被编译器推断为 `x` 的生命周期，当编译器看到`y: &'a str`的时候，编译器会将`'a`推断为 `y` 的生命周期，但是此时有冲突，于是编译器会将`'a`推断为`x`和`y`的生命周期中最小的那个，并且编译器也看到返回值也是 `'a`，这就意味着编译器可以得到一个断言：**返回值（是个引用）的引用值的生命周期绝不会小于 `'a`**，如果实际的生命周期违反了此断言，编译器就会报错。

这一切都是为了保证这一个原则：**引用的生命周期绝不能大于值的生命周期**，否则就会产生悬垂指针。



对于下面这个例子

```rust
fn main() {
    let string1 = String::from("abcd");	// 值到 } 结束
    let string2 = "xyz";				// 值是静态

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

编译器通过之前的规则得出 `'a` 为两者中较短的那个，即到 `}` 结束，它就是返回值的引用值（或许是 `string1`，或许是 `"xyz"` ）的推定生命周期，而 `result` 的生命周期也是到 `}` 结束，满足之前提到的规则，即**返回值的引用值的生命周期（至少能和 `string1` 活得一样长）绝不会小于该引用（即 `result` ）的生命周期 **，编译通过。



而对于下面这个例子

```rust
fn main() {
    let result;
    let y = "world";
    {
        let x = String::from("hello");
        result = longest(x.as_str(), y);
    }// here
    println!("r: {}", result);
}// there
```

编译器推断返回值的生命周期是到 `// here` 结束（取 `x` 和 `y` 的引用值较短的那一个，即 `x`），而 `result` 的实际生命周期却是到 `there`，这就违反了之前的规则：**返回值的引用值的生命周期（即 `x` ）绝不会小于该引用（即 `result` ）的生命周期** ，编译报错。





#### 结构体

当我们定义的struct的里面有对象引用的时候，我们需要在struct的模板参数中增加生命周期声明。

跟函数生命周期声明类似，当一个生命周期参数修饰多个字段的时候，编译器会将这个生命周期参数推断出这几个字段生命周期最小的那个。这是因为一个包含引用成员的结构体，必须保**证结构体本身的生命周期不能超过任何一个引用成员的生命周期**。否则就会出现成员已经被销毁之后，结构体还保持的那个成员引用就会变成悬垂引用。所以 **Rust 把引用成员的最短生命周期推断为结构体的生命周期，并且断言结构体不小于该生命周期**

```rust
struct Foo<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn main() {
    let x = 6;
    let m;
    let f;
    {
        let y = 6;
        f = Foo { x: &x, y: &y };
        m = f.x;
    }
    f;
}
```

在上述例子中，`Foo`包含一个生命周期参数，这个生命周期参数修饰了`x`和`y`结构体字段，`'a`将会被编译器推断为`x`和`y`的生命周期中最小的那个（即 `y` 的生命周期）这也是编译器推断的结构体的生命周期。可以看出，结构体 `Foo` （即 `f`）超出 `y` 的生命周期，断言不成立，编译报错。



但是在下面这个例子中：

```rust
struct Foo<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn main() {
    let x = 6;
    let m;                     
    {                          
        let y = 6;            
        let f = Foo { x: &x, y: &y };  
        m = f.x;             
    }                          
    println!("{}", m);   // error
}
```

在上述例子中，`Foo`包含一个生命周期参数，这个生命周期参数修饰了`x`和`y`结构体字段，`'a`将会被编译器推断为`x`和`y`的生命周期中最小的那个（即 `y` 的生命周期），可以看出，结构体 `Foo` 确实没有超出 `y` 的生命周期，断言成立。

**但是**因为`x`的生命周期被声明为和`y`的生命周期一样，所以当打印 `m` 的时候，`x`会被编译器认为已经无效。这也提醒我们，尽管 x 的实际生命周期一直要到 `main` 的 `}`，但是手工标注的生命周期会覆盖编译器推断的生命周期。



当你觉得 Rust 的借用检查一直在给你使绊子时，请记住：

**Rust的借用检查器对程序的生命周期标记只要求到能够以静态的方式验证程序的内存安全。 Rust会爽快地编译一个程序，即使它的生命周期标记有语义上的错误， 这带来的结果就是程序会变得过于受限**

换句话说，你只要能够合理的给编译器**细粒度**的生命周期，编译器并不会干扰你写代码，如果你想当甩手掌柜， Rust就会用他那捉急的智商给你一个粒度极粗糙的生命周期。





#### early bound 和 late bound

需要提前说明的是，生命周期参数其本质不过是一种泛型，对于泛型所指的具体类型无非有两种手段：

1. 在调用前手动指定。
2. 在调用时由编译器自动推断。

前者就对应 `early bound`，后者就对应 `late bound`.在 Rust 中普通泛型都只能 `early bound`，而只有生命周期泛型允许 `late bound`，这是因为函数可能会被多次调用，而每次调用时传入参数的生命周期很可能不同，这就导致编译器放松了对生命周期泛型的限制，允许它调用时再推断泛型。而且编译器一旦判定该泛型是 `late bound` 后就不允许再 `early bound`了。



生命周期泛型允许晚绑定，但以下**两种情况**编译器会将其早绑定（`early bound`）：

1. 生命周期被指定超过某个生命周期，如 `'a: 'b` 这种形式，就是对 `'a`  和 `'b` 的 `early bound`
2. 生命周期在函数签名之外声明，例如在结构体的关联方法中，它的生命周期就可能来自结构体



##### 情况1

```rust
fn f<'a>() {}
fn g<'a: 'a>() {}

fn main() {
    let pf = f::<'static> as fn();	// error
    let pg = g::<'static> as fn();	// ok
}
```

在上面这段代码中，`f` 的绑定属于晚绑定，它的生命周期参数要到函数具体调用时才确定，而 `g` 则属于早绑定，因为它符合之前提到的两种情况的情况 1。`pg` 的手动绑定会被编译器允许，因为 `g` 本来就是早绑定，现在只是传递了泛型参数而已，编译通过。但是对于 `pf` 编译器却不允许传递泛型参数，因为它是一个晚绑定，不允许提前绑定生命周期参数。

```rust
fn f<'a: 'a, 'b>() {}
fn g<'a: 'b, 'b>() {}

fn main() {
    let pf = f::<'static, 'static> as fn();		// error
    let pg = g::<'static, 'static> as fn();		// ok
}
```

上面的 `f` 生命周期 `'a'` 被绑定，但 `'b` 没有，因此报错。但是 `g` 同时绑定了两个参数，因此通过编译



##### 情况2

对于下面这段代码，编译不通过

```rust
struct Buffer {
    buf: Vec<u8>,
    pos: usize,
}

impl Buffer {
    fn new() -> Buffer {
        Buffer {
            buf: vec![1,2,3, 4, 5,6],
            pos: 0,
        }
    }

    fn read_bytes<'a>(&'a mut self) -> &'a [u8] {
        self.pos += 3;
        &self.buf[self.pos-3..self.pos]
    }
}

fn print(b1 :&[u8], b2: &[u8]) {
    println!("{:#?} {:#?}", b1, b2)
}

fn main() {
    let mut buf = Buffer::new();
    let b1 = buf.read_bytes(); 	// --b1
    let b2 = buf.read_bytes();	// 	 |
    print(b1,b2)				// --b1
}
```

原因在于 `read_bytes` 属于一个晚绑定，只有调用时才会传递生命周期参数，而 `b1` 的生命周期如图所示，由于它指定了 `&mut self` 的生命周期也是 `'a`，因此这就说明当 `b1` 存在时那么这个可变借用也存在，问题的关键就在这里，`b2` 那一行也出现了可变借用，但是此时 `b1`还存活着，所以 `b1` 那一行所引入的可变借用也活着，由于不能有两个可变借用，因此此时报错。

如果将其变成下面这样，显然由于 `b1` 的生命周期结束时（**NLL判断**）可变借用的生命周期也可以结束，因此 `b2` 再次引入可变借用不会报错。

```rust
fn main() {
    let mut buf = Buffer::new();
    let b1 = buf.read_bytes();	// --b1
    print(b1, b1);				// --b1
    let b2 = buf.read_bytes();
}
```



然而其实我们知道，`b1` 和 `b2` 的可变借用其实是没有关系的，当返回 `b1` 时引入的那个可变借用其实就应该消亡了，因此这就是 Rust 还不够智能的地方，我们将源代码进行一些更改

```rust
fn read_bytes<'a: 'b, 'b>(&'b mut self) -> &'a [u8] {
    self.pos += 3;
    &self.buf[self.pos - 3..self.pos]
}
```

这样表达我们的意图，表示 `&mut self` 的生命周期更短，而返回值的生命周期更长。看起来好像很完美，但它是错误的，原因在于这违反了生命周期规则，可能会引入悬垂指针，用户完全可能写出这种符合生命周期规则的代码，这其中 `b1` 的生命周期确实比 `&mut self` 要长，但它长过头了，以至于结构体都消亡了这个引用还没消亡：

```rust
fn main() {
    let b1: &[u8];
    {
        let mut buf = Buffer::new();
        let b1 = buf.read_bytes();
    }
    b1;	// dangling pointer
}
```

————————————————————————————————————



至此我们开始接触之前提到的早绑定情况2。在结构体中，为该结构体声明的生命周期是在 `impl`，如果在函数上的话单指为函数声明生命周期：

```rust
struct Buffer<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'b, 'a: 'b> Buffer<'a> {
    fn new(buf: &'a [u8]) -> Buffer {
        Buffer { buf: buf, pos: 0 }
    }

    fn read_bytes_early(&'b mut self) -> &'a [u8] {
        self.pos += 3;
        &self.buf[self.pos - 3..self.pos]
    }
}


fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let mut BUF = Buffer::new(&v);
    let b1 = BUF.read_bytes_early();
    let b2 = BUF.read_bytes_early();
    print(b1, b2);
}
```

在上面这种结构体中，我们声明了 `'a` 和 `'b` 并将其早绑定，注意早绑定的生命周期参数可以运行时再传递，但晚绑定的就不能反过来提前传递，见**情况1**。

在 `let mut BUF= Buffer::new(&v);` 中编译器提前绑定 `'a` 为 `&v` 的生命周期参数，并且指定 `'b` 可以短于 `'a`，即 `read_bytes_early` 中输入参数的生命周期 `&mut self` 允许短于输出的引用生命周期，这就类似于下面这种编程：

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let mut BUF = Buffer::new(&v);
    let b1: &[u8];
    let b2: &[u8];
    {
        let s = &mut BUF;	// local mut ref	--s
        b1 = read_bytes(s);	//					  |
    }// drop local mut ref						--s
    {
        let s = &mut BUF;	// local mut ref
        b2 = read_bytes(s);
    }// drop local mut ref
    print(b1, b2)
}
```

由于 `s` 的生命周期确实满足 `'a: 'b`（`s`的生命周期就是 `'b'`），因此编译器允许调用。退出一个作用域时 `s`这个可变借用已经消亡，自然允许下一次可变借用（即 `b2`）。



Q：这样做为什么不会引起悬垂指针？

A：因为我们把 **引用`buf`**，**结构体`BUF`** 和 **函数`read_bytes_early` 返回的引用** 都关联在一起，换句话说他们的生命周期应该至少是一致的（总的原则是不能出现悬垂指针， 所以 `&v` 允许活得更久一点）

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let b1: &[u8];
    let b2: &[u8];
    {
        let mut BUF = Buffer::new(&v);
        b1 = BUF.read_bytes_early();
        b2 = BUF.read_bytes_early();
    }// drop BUF
    print(b1, b2);
}
```

如果违反上面的条件则编译器报错。

```rust
fn main() {
    let mut BUF: Buffer;
    {
        let v = vec![1, 2, 3, 4, 5, 6];
        BUF = Buffer::new(&v);
    }//drop v
    BUF;	// `v` does not live long enough
}
```





### 高阶生命周期

若一个函数是late bound，当它被**当做变量**传递时，我们就可以用`for`语法把它内部的生命周期参数暂时的悬置起来

```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let f = |p: &Person| &p.name;	// compile error
}
```

闭包会自动转换为一个函数指针，但是这个函数没有指定生命周期，所以可能传入的结构体比它的字段活得短，这样就会外面还有变量持有结构体字段的引用但是结构体已经消亡的情况，造成悬垂引用。手动标注生命周期如下，我们需要用到 `for<'a>` 来悬置生命周期：

```rust
fn main() {
    let f: for<'a> fn(&'a Person) -> &'a String = |p: &Person| &p.name;
    
    // 现在 Rust 也支持这样写了
    let f: fn(&Person) -> &String = |p: &Person| &p.name;
}
```

