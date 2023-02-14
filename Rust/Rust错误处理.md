### assert! 宏

防御性编程，断言某个`bool`表达式为真，如果不为真则`panic`

第一个参数是 `bool`  表达式，之后的参数是用于格式化字符串。

```rust
assert!(true);

fn some_computation() -> bool { true } 
assert!(some_computation());

let x = false;
assert!(x, "x wasn't true!");	// -> 打印 x wasn't true!

let a = 1; let b = 2;
assert!(a + b == 4, "a = {}, b = {}", a, b);	// -> 打印 a = 1, b = 2
```



### Option\<T>

某些行为并不总是有被定义的结果，例如对空向量 `pop()` 操作，应该是返回什么呢，是直接 panic 还是返回 null，还是什么都不做。

不同的语言有不同的处理方式，Rust 选择的的是使用 `Option` 进行映射，他将所有的正常操作映射为 `Some`，所有的未定义操作映射为`None`。

当一个函数返回值是 `Option` 时就意味着该函数并不一定总是成功，我们需要自己提供对 `None` 的处理，而非由库的作者提供，这样更加提高了定制性。



`Option` 本质上是一个枚举，属于和类型，因为一个 `Option` 只有两种情况，要么为 `Some` 要么为 `None`，只可能为两种情况之一，它的内存布局都是一个 tag 加上两者（`Some`和`None`）中内存更大的那个，相当于是C语言的`union`。甚至于如果确定 `Some` 必定不为 0 时，编译器甚至会将 tag 优化掉，例如 `Some` 包裹的是 `Box<T>`.

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171657415.png" alt="image-20220517165720323" style="zoom: 50%;" />



`Option`的关键在于不要立马把它里面的值取出来，而要使用链式调用开火车，当链式调用遇到 `Some` 时会应用函数在其值上，而遇到`None` 时自然会停下来。最后返回的结果仍然会用 `Option` 包装。

```rust
fn square(value: i32) -> i32 {
    value.pow(2)
}

fn is_even(value: &i32) -> bool {
    value % 2 == 0
}

fn main() {
    let v = 6;
    let result = Option::from(v).map(square).filter(is_even); //Some(36)
    let a = None.map(|x: i32| x + 1);	// None
}
```





### Result<T, E>

`Option` 只能够告诉调用者是否成功，但无法提供更多的信息。我们希望失败时不仅告诉我们失败，还要提供更具体地信息，这时候就用 `Result<T,E>`，其中的 `E` 就包含具体的错误值，它一定实现了 `Error trait`，该 `trait` 继承了 `Debug + Display`。这也意味着只要我们能自己实现该 `Trait` 则也可以用于泛型 `E`。

```rust
pub enum Result<T, E> {
	/// Successfully
    Ok(T),
    
    /// Contains the error value
    Err(E),
}
```

当遇到错误是，我们有两种策略：1. 立即处理错误。2. 向上一层报告错误，即传播错误。



#### 自定义错误

有时为了定制化需求，我们需要在自己的项目中自定义错误类型。完成该行为需要实现 `std::error::Error` 的特征，而该特征继承自 `Debug + Display`，因此这两个特征也必须实现

```rust
#[derive(Debug)]
struct A();

// 实现 Display 使得该错误可以打印
impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is here", self.0)
    }
}

impl Error for A {}

fn main() {
    let myres: Result<(), A> = Err(A);
    match myres {
        Ok(_) => todo!(),
        Err(e) => println!("display is \"{}\"", e),
    }
}
```



如果该错误还有嵌套的子错误，那么可以在 `Error` 中实现 `source()` 方法来追踪错误

```rust
#[derive(Debug)]
struct A(i32, ChildError);	// A是一种错误类型，可以放在 Err 中

// 打印本错误
impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is here", self.0)
    }
}

impl Error for A {
    // 实现 source 以实现错误溯源
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.1)
    }
}

// 定义子错误
#[derive(Debug)]
struct ChildError(String);

impl std::fmt::Display for ChildError {
    // 描述子错误
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "child err is caused by {}", self.0)
    }
}

// 子错误已不需要溯源
impl Error for ChildError {}

fn main() {
    let myerr: Result<f64, A> = Err(A(5, ChildError("special err".to_string())));
    match myerr {
        Ok(f64) => println!("no err"),
        Err(e) => {
            // 打印错误	-> 5 is here
            println!("{}", e);	
            
            // 错误溯源 -> child err is caused by special err
            println!("{}", e.source().unwrap());	
        }
    }
}
```





#### 立即处理错误

当一个错误发生的时候，就可以立即开始进行错误处理。但是，我们经常不希望在某个发生错误的位置处理错误。因为那样会十分繁琐，破坏代码的可读性。比如下面这段代码：

```rust
fn caller() {
	// 可能出错，选择立即处理
    match may_fail1() {
        Ok(happy) => println!(":)"),
        Err(sad) => {
            eprintln!(":(");
            /* handle error */
            return;
        }
    }
	
    // 可能出错，选择立即处理
    match may_fail2() {
        Ok(happy) => println!(":)"),
        Err(sad) => {
            eprintln!(":(");
            /* handle error */
            return;
        }
    }
	
    // 可能出错，选择立即处理
    match may_fail3() {
        Ok(happy) => println!(":)"),
        Err(sad) => {
            eprintln!(":(");
            /* handle error */
            return;
        }
    }
	
    println!("I am so happy right now");
}
```

从上面的代码可以看到，本质上就调用了三个可能出错的函数，但是写出如此冗余的代码。因此错误处理逻辑集中起来处理是更清晰的。



#### 向上传播

todo





#### ? 操作符

Rust的异常处理是通过 `Result` 的 `Ok` 或 `Err` 成员来传递操作成功还是失败，然而错误信息的处理一般都是要通过match来对类型进行比较, 所以很多时候代码比较冗余, 通过`?`符号来简化判断

```rust
// 原本代码
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt");
	
	// Ok 则取出，否则直接返回错误
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
	
 	// Ok 则取出，否则直接返回错误
   let res = match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    };
    
    res
}
```

我们可以看到上面两种模式都有共同特点就是需要取出 `Result` 的值，如果出错就传播错误，让上层去处理。这种通用的模式可以用 `?` 来模拟。`?` 的作用就是尝试取出值，一旦发现是错误则直接传播，本层不做处理。

可以将 `?` 是一种 `unwarp()`，唯一不同的是 `unwarp()` 遇到 `Err` 会 `panic`，但是 `?` 遇到 `Err` 会返回它。

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```



`?` 操作符还可以执行隐式的 `From` 操作，可用这种特性将不同的错误抽象为同一类错误

```rust
// 定义抽象的解析错误
#[derive(Debug)]
enum ParseError {
    ChildError1(std::num::ParseIntError),
    ChildError2(std::num::ParseFloatError),
}
// 分派抽象错误给具体错误处理逻辑，为了简便，我们只是简单打印
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ChildError1(e) => e.fmt(f),
            ParseError::ChildError2(e) => e.fmt(f),
        }
    }
}
impl Error for ParseError {}

// 定义转换
impl From<std::num::ParseIntError> for ParseError {
    fn from(s: std::num::ParseIntError) -> Self {
        println!("I was be converted");
        ParseError::ChildError1(s)
    }
}

fn produce_err() -> Result<i32, ParseError> {
    let num = i32::from_str_radix("abc", 10)?;	// ? 在发生错误时会调用 from 进行隐式转换
    Ok(num)	// 顺利 parse 则返回 Ok
}

fn main() {
    let c: Result<i32, ParseError> = produce_err();
    match c {
        Ok(num) => println!("num is {}", num),
        Err(e: ParseError) => println!("{}", e),	// -> invalid digit found in string
    }
}
```

在某个函数处理中往往可能会在各个环节触发错误，如果一个个都就地处理有可能扰乱主干逻辑，降低代码可读性

用 `?` 操作符可以把一段逻辑链上所有的错误都抽象为一个枚举，进而单独处理出现的错误，简化主干逻辑，使代码更加可读



### panic

#### Unwinding





#### Aborting