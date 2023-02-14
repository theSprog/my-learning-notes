## take

假设我们只有一个可变引用，但想要获取它的值的所有权，可以使用 `take`

```rust
let mut v = vec![1, 2, 3];
let a = mem::take(&mut v);
println!("{:?}", a);	//	[1, 2, 3]
println!("{:?}", v);	//	[]	default value
```

注意必须要使用可变引用，因为 `take` 的行为其实是将 v 的值返回并且在 v 的内存处放置类型默认值（这也侧面暗示了类型必须实现 `default` 才能使用 take）。



## swap

如果要交换可变引用的值可以使用 `swap`

```rust
let mut v1 = vec![1, 2, 3];
let mut v2 = vec![3, 4, 5];
mem::swap(&mut v1, &mut v2);
println!("{:?}", v1);	// [3, 4, 5]
println!("{:?}", v2);	// [1, 2, 3]
```

同样的，必须是可变引用，因为只有可变引用的语义才是能改变所引用的值。



## replace

如果我们想将某个内存处的值进行替换，可以使用 `replace` ，该方法会返回被替换的旧值。

```rust
let mut v1 = vec![1, 2, 3];
let v2 = vec![3, 4, 5];
let v3 = mem::replace(&mut v1, v2);	// v2 被 move, 之后不能再使用
println!("{:?}", v1);	// [3, 4, 5]
println!("{:?}", v3);	// [1, 2, 3]
```

