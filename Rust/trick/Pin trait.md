### Pin概念

**Pin**是一个这样的智能指针，他内部包裹了另外一个指针**P**，并且只要**P**指针指向的内容（我们称为**T**）没有实现**Unpin**，则可以保证**T**永远不会被移动（move）。

之所以要使用 Pin 是为了防止自引用结构体 move 时发生逻辑错误

<img src="C:\Users\fan\AppData\Roaming\Typora\typora-user-images\image-20220622121613787.png" alt="image-20220622121613787" style="zoom:50%;" />

如上图所示，在 move 之后 `name_ref` 本来是指向自身的一个域，但此时却指向了失效内存，这样很容易引发内存错误



### 自引用结构体

自引用结构体是一个这个样的结构体，它内部某个成员是对另外一个成员的引用。

```rust
#[derive(Debug)]
struct Test {
    name: String,
    name_ref: *const String, // 引用自身域
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            name: String::from(txt),
            name_ref: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.name;
        self.name_ref = self_ref;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn name_ref(&self) -> &String {
        unsafe { &*(self.name_ref) }
    }
}

fn main() {
    let mut data1 = Test::new("Tyr");
    data1.init();
    let mut data2 = Test::new("Lindsey");
    data2.init();
}
```

初始化后内存布局如下：

<img src="C:\Users\fan\AppData\Roaming\Typora\typora-user-images\image-20220622121918156.png" alt="image-20220622121918156" style="zoom: 67%;" />

假设经过交换之后

```rust
println!(
    "data1 {{name: {}, name_ref: {}}}",
    data1.name(),
    data1.name_ref()
);	// -> data1 {name: Tyr, name_ref: Tyr}

// 使用swap()函数交换两者
std::mem::swap(&mut data1, &mut data2);

println!(
    "data2 {{name: {}, name_ref: {}}}",
    data2.name(),
    data2.name_ref()
);	// -> data2 {name: Tyr, name_ref: Lindsey} 
```

<img src="C:\Users\fan\AppData\Roaming\Typora\typora-user-images\image-20220622122256311.png" alt="image-20220622122256311" style="zoom:67%;" />



可以看出 `name_ref` 不再指向 `Tyr`，而是之前的内存地址，现在该内存指向 `Lindsey`



### Pin的作用 ???

Pin<P\<T>> 拿住的是一个可以解引用成 T 的指针类型P，而不是直接拿原本的类型T。因此我们一般看到 Pin<&T>、Pin<Box\<T>>等等，因为 Pin 可以把 T 内存位置锁住，move 时只是 移动了指针 P，而 T 是一直不动的。

如下图所说，`Test` 结构体从未移动

<img src="C:\Users\fan\AppData\Roaming\Typora\typora-user-images\image-20220622122949423.png" alt="image-20220622122949423" style="zoom: 67%;" />



Rust 十分宽容，他默认给你们所有类型都实现了 `Unpin`，要想声明一个类是 `Pin` 住的，有两种方式

- 使用 **PhantomPinned**

  ```rust
  struct Test {
      name: String,
      name_ref: *const String, 
      marker: PhantomPinned,	// 声明标记
  }
  ```

- 给自己手动`impl !Unpin`。前提要使用nightly版本，并且需要引入 **#![feature(negative_impls)]**

满足以上任意两个条件之一的话，Rust 就会保证你没办法在Safe Rust下拿到可变借用`&mut T`（`get_mut()` 要求Pin里面的数据都是 `Unpin`，而 `PhantomPinned` 正是官方提供的实现了 `Pin` 的类型）。

拿不到`&mut T`你就没办法作用到`std::mem::swap()`上，因为这会使得类型不匹配从而编译不过



当数据结构满足`Unpin`时，创建`Pin`以及使用`Pin`都可以使用安全接口，否则，需要使用 unsafe 接口：

```rust
// Target 满足 Unpin
impl<P: Deref<Target: Unpin>> Pin<P> {
    // 安全的接口，用户可以放心使用
    pub const fn new(pointer: P) -> Pin<P> {
        unsafe { Pin::new_unchecked(pointer) }
    }

    pub const fn into_inner(pin: Pin<P>) -> P {
        pin.pointer
    }
}

// Target 满足 Pin
impl<P: Deref> Pin<P> {
    // 不安全的接口，用户需要自己为安全负责
    pub const unsafe fn new_unchecked(pointer: P) -> Pin<P> {
        Pin { pointer }
    }
    
    pub const unsafe fn into_inner_unchecked(pin: Pin<P>) -> P {
        pin.pointer
    }
}

```

