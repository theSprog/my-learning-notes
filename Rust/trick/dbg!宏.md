### `dbg!`宏

该宏是一个表达式，表达式的结果就是 `dbg!(expr)` 中 `expr` 的值。

```rust
let a = dbg!(5*2+6);	
```

上述代码执行完后 `a` 的值为 16。



`dbg!`宏的作用就是会将 `expr` 转化为一个字符串，例如表达式 `3 + 5` 就转换为 `"3 + 5"`，注意如果表达式是字符串还会再加上 `"` 引号

并且将 `expr` 的结果求出，在`expr字符串`后拼接上该表达式的结果

```rust
let s = String::from("hello");
let res = dbg!(s + " world");	// -> s + "world" = "helloworld"
println!("{}", res);			// -> helloworld
```

