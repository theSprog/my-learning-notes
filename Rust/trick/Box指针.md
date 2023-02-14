### Box 指针与生指针

`Box`指针本质上是一个类型安全的不可能为空的指针，这也就意味着我们可以将其转为生指针。

但是我们对生指针的操作将被标记为 `unsafe`，因为我们可以更改本来被标记为不可变的数据

```rust
let box1 = Box::new(1);
let raw_ptr1 = Box::into_raw(box1);
unsafe {
    *raw_ptr1 = 2;	// box1 不可变但是被更改了
}
```

`into_raw` 是移动操作，不可再使用 `box1`。



也可以将某个生指针强行转换为`Box`类型指针，但这同样是不安全的行为，因为可能会发生这样的事情：

```rust
unsafe {
    let raw_ptr2 = 0 as *mut i32;	// 将 0 赋给指针
    let box2 = Box::from_raw(raw_ptr1);	// unsafe: box2 为 Box<i32> 类型
    println!("{}", box2);			// 对指针 0 解引用
}	
```

编译器会根据 `*mut T` 中的 `T` 来推断 `Box<T>` 指针的类型



### Box 指针转引用

本来 `Box<T>` 是指向的内存不可修改，但是我们可以把它 "泄漏（leak）" 给某个引用，从而让它变得可以修改

```rust
let box1 = Box::new(1);
let ref1 = Box::leak(box1);	// ref1: &mut i32
```

`leak()` 之后的引用默认就是可修改的（即 `&mut T`）。

注意该方法是移动操作，`box1` 已经被移动了，不可再使用