`matches!()` 是一个简便的模式匹配宏，

```rust
matches!(expr,pattern)
```

- `expr`:是指条件判断的入参
- `pattern` 是期待为`true`的匹配模式
- 其中 `pattern` 还支持 **匹配守卫** （match guard）



```rust
let exp = 's'
matches!(exp, 'A'..='Z' | 'a'..='z' | '0'..='9')	// -> true
```

```rust
let bar = Some(4);
assert!(matches!(bar, Some(x) if x > 2));	// if x > 2 是一个 match guard
```



#### `matches!()` 枚举变量

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 50 };
matches!(msg, Message::Hello { id: 3..=70 });	// -> true
```

以上写法相当于普通的 `match`

```rust
match msg {
    Message::Hello {
        id: id_variable @ 3..=70,	// 由于后面需要使用 id 变量, 可以用 id_variable @ 3..=70 这种模式
    } => println!("Found an id in range: {}", id_variable),
    
    _ => {
        println!("Found an id in another range")
    }
}
```

