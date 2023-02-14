### 语法

枚举的本质其实类似于C语言中的union结构体。它用一定的内存表示 `tag`（通常是一个 `u8`）,后接一个 `union` 结构体共用内存空间。如下面这个枚举。

```rust
enum Data {
    Empty,
    Number(i32),
    Array(Vec<i32>),
}

println!("{:#?}", size_of::<Data>());	// 32 (64位机器) = 8 + 24（Vec结构体）
```

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205171645717.png" alt="image-20220517164535624" style="zoom:50%;" />



枚举的出现部分取代了C中的常量，比如不要在 Rust 写这样的代码，而是换成枚举更好

```rust
const NO: bool = false;
const YES: bool = true;

// 换成枚举
enum STATE {
	NO,
	YES,
}
```



#### match

枚举无法使用 `==` 运算符，更无法使用 `<` `>` 之类的运算，因为枚举从类型上不是整型，Rust不会允许类型隐式转换。要得到枚举内部值，可以使用 `match` 模式匹配。`match` 做的事就是**依次**穷举列出的模式，若找到一个模式匹配该枚举则返回，不再向后继续检查。可以看出 `match` 也是一个表达式，这就要求所有：1.必须穷尽所有的枚举情况，2.分支的返回值类型必须相同。

```rust
enum STATE {
    NO,
    YES,
}

let v = STATE::YES;

let a = match v {
    STATE::NO => println!("NO"),	// 枚举结尾使用 , 而非 ;
    _ => println!("YES"),
};
```

上面的语句可以看出，`println!()` 其实也是表达式，但是该表达式的类型是 `()` 单元类型。使用 `_` 表示通用匹配。



#### if let

当枚举内部只有两个值，尤其是我们并不关心另一个值是什么的时候（例如 `Option`），每次都写 `match` 并在分支最后加一个 `_` 十分烦人，因此我们可以使用 `if let` 语法糖

```rust
let v = Some(5);

match v {
    Some(k) => println!("{}", k),
    _ => (),
}

// 语法糖。
// 如果匹配成功则执行后面的语句, 
// 否则什么也不做, 类似于之前的 _ => (),
if let Some(k) = v {
    println!("{}", k); // 此处使用 ;
}
```



#### while let

`while let` 同样也是 `match` 的语法糖，一般用于 `loop`中 `match` 的替换

```rust
let mut v = vec![1,2,3];
loop {
    match v.pop() {
        Some(k) => println!("{}", k),
        None => break,
    }
}

// 语法糖，当不匹配时自动退出 while 循环
while let Some(k) = v.pop() {
    println!("{}", k);
}
```

`vec.pop()` 是 `Option` 枚举，当没有值时（`None`）break退出。有意思的是，`break` 其实是 `never` 类型，表示该表达永远不会完成计算结果，它可以转换成任意类型，当然也可以转为 `()` 类型。

```rust
loop {
    let a: ! = break;	// break 只能用在循环里面
    let b: u32 = a;
}
```



需要注意的是 `if let`  和 `while let` 不仅可以匹配正确的结果，也可以用来匹配 `None` 和错误 `Err`

```rust
if let None = v {
    println!("none");	// 如果是 None 则打印
}

while let Err(k) = v {
    println!("{}", k);	// 打印错误信息
}
```

