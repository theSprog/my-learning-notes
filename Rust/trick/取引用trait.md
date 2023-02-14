### 取引用

取引用也有一对 trait，分别叫做 `AsRef` 和 `AsMut`，分别对应 `&T` 和 `&mut T`。只不过用法不同的就是 `AsRef<T>` 不是在 `&U`  时生效，而是在 `U.as_ref()` 时，将 `U` 取不可变引用转换为 `&T`。



#### 取不可变引用

它的作用就类似于 `Java` 中的 `getter()`

```rust
struct Something<T>(T);

impl<T> AsRef<T> for Something<T> {	// AsRef中指定的类型即为取引用后的类型
    fn as_ref(&self) -> &T {
        &(self.0)
    }
}

fn main() {
    let v = Something(format!("114514"));
    let v2: &String = v.as_ref();	// 取出内部数据
    println!("{}", v.as_ref());	// 114514
}
```



#### 取可变引用

可变取引用不必先实现 `AsRef`，同样 `AsMut<T>`  中指示返回的类型为 `&mut T`。取该可变引用的方法是 `as_mut()`

它的作用就类似于 `Java` 中的 `setter()`

```rust
struct Something<T>(T);

impl<T> AsMut<T> for Something<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

fn main() {
    let mut v = Something(format!("114"));
    let v2: &mut String = v.as_mut();
    v2.push_str("515");
    println!("{}", v2);	// 114515
}
```



### 多态

用这种方法也可以实现多态。

用这种方法 `print_string` 可以接受一切实现了 `AsRef<String>` 的类型。例如我们可以传入自定义类型 `Something`

```rust
struct Something<T>(T);

impl<T> AsRef<T> for Something<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

fn print_string(s: impl AsRef<String>) {
    println!("this is {}", s.as_ref());
}

fn main() {
    let v = Something(format!("114514"));
    print_string(v);
}
```

