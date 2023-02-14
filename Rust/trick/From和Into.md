### From

`From` 和 `Into` 都是值类型到值类型的转换，这意味着他会消耗某个资源

`From` 将某个外界提供的值转换为我们需要的类型，如名称所示

```rust
struct Something<T>(T);

impl<T> From<T> for Something<T> {
    fn from(t: T) -> Self {
        Something(t)
    }
}

fn main() {
	let a: Something<bool> = Something::from(true);
}
```



### Into

如果你实现了 `From`，那你会自动获得一个对偶的 `Into`，而不必手动实现。**但需要注意的是这时你必须为编译器手动提供类型，因为一个类型可以Into为很多个类型**

```rust
struct Something2<T>(T);		// 假设我们又实现了另一个类型 Something2
impl<T> From<T> for Something2<T> {
    fn from(t: T) -> Self {
        Something2(t)
    }
}

fn main() {
	let a: Something<bool> = let b: Something<bool> = false.into();	// 此时必须手动为编译器提供信息
}
```

