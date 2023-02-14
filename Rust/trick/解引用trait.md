### 为对象实现解引用

#### 不可变解引用

Rust 中引用本来是一个指针，`*` 操作符本质上就是对一个地址进行间接跳转。但有时候我们也能够对一个对象进行取引用，解引用。就比如对一个 `String` 取引用会得到 `&str`

```rust
struct Packed<P, Q> {
    obj1: P,
    obj2: Q,
}

impl<P, Q> Deref for Packed<P, Q> {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        let r = Box::new(2);
        Box::leak(r)
    }
}

fn main() {
    let p = Packed::<u8, u8> { obj1: 1, obj2: 2 };
    println!("{:?}", *p);	// -> 2
}
```

可以看出，我们的解引用操作符对一个自定义的结构体发生了作用，而且返回一个常量引用 `&2`，`*` 再对它发生作用。换句话说 Rust 允许在真正的解引用之前添加额外的操作，并将解引用放在最后一步



#### 可变解引用

可变解引用必须要首先实现 `Deref`，并且它内部的 ` type Target ` 也是 `DerefMut` 的返回值类型，区别在于多了一个可变修饰符号

```rust
static mut R2: i32 = 0;

impl<P, Q> DerefMut for Packed<P, Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            println!("there is {}", R2);
            R2 += 1;
        }
        Box::leak(Box::new(0))
    }
}

fn main() {
    let mut p = Packed::<u8, u8> { obj1: 1, obj2: 2 };
    *p = 5;	// -> there is 0
    *p = 5;	// -> there is 1
    *p = 5;	// -> there is 2
    *p = 5;	// -> there is 3
    *p = 5;	// -> there is 4
}
```

同样的，我们可以在解引用之前做出许多操作，并且最后返回的类型都不一定要是 `Packed` 中的 `P` 或者 `Q`.