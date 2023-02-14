### package

当我们用 `cargo new` 的时候就会创建一个 package，即新建的整个文件夹就是一个 package。`Cargo.toml` 里面 `[package]` 下就会声明 这是一个 package，其中的 `name` 字段一般默认情况下与文件夹名称一致，当然你也可以手动更改它，这样就改了 package 的名字



如果想调用本地 crate，只需在 `Cargo.toml` 中声明另一个 package 的路径

```
[dependencies]
pkg = {path = "..."}
```

如果使用相对路径，需要记住当前路径 `./` 表示 package 文件夹的目录，即 `package/`



### crate

crate 是编译的基本单元。

一个 package 至多包含一个库 crate （lib）和多个可执行 crate（bin）。



当有多个可执行 crate 时，默认一个是 `src/main.rs`，其他的可执行 crate 放在 `bin` 文件夹下。`src/main.rs` 这个 crate 的名称与 package 的名称一致，但是 `bin` 文件夹下的 crate 名称就与文件名一致了，比如 `bin/abc.rs` 的 crate 名称是 `abc`。

顺便说一句：`src/main.rs` 并不一定是必要的，我们完全可以将其放入 `bin` 目录下，但是此时 crate 的名称就变成 `main` 而非 package name

同样的，默认的库 crate 是 `src/lib.rs` ，它的名称同样也是与包名（package name）一致



当我们使用关键字 crate 是就是指本 crate，使用该关键字表示从 crate 开始往下查找 module，即**绝对路径方式**，如果要使用其他 crate 则需要写出全名，如 `rand::` 就是指使用 rand crate

这里有个小点：如果自己要使用自己 package 的库 crate，那么要写出 package name 开始绝对路径查找，此时编译器知道你是要使用库 crate 而不是本可执行 crate，也就是说库 crate 和可执行 crate 虽然同名，但编译器不会将他们弄混，编译器依靠 package name 和 `crate` 关键字来区分

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202206071040161.svg" alt="module tree" style="zoom: 80%;" />

### module

![image-20220607110549794](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202206071105877.png)



文件系统和 module 并不是一一对应的，如上图所示，`src/main.rs` 就对应 crate 根，在它下面有三个子 `module`，分别是 `config`、`routes` 和 `models`，在后两者下又继续划分子 `module`。



#### 子module声明方式

1. 直接声明为一个文件，如 `config.rs`，此时 module 名称与文件名称相同
2. 声明为一个文件夹，并且文件夹下必须有一个 `mod.rs`。此种方式 module 名称与文件夹名称相同，此时可将文件夹视为一个 module，在它之下的文件是子 module



#### 最佳实践

如果某个 module 没有子 module，那么直接将其声明为一个文件，否则的话将其声明为一个文件夹，并在 `mod.rs` 中声明子 module 的存在



#### mod 关键字

使用 `mod` 关键字可以将一个文件或文件夹加入到 `module tree` 中，例如只有在 `src/main.rs` 中声明 `mod config`，在 `module tree` 中才有 `config` 这个 module，否则的话即使有这个文件 Rust 也会忽视它的存在



#### use 关键字

- 使用 module 时要么写绝对路径，即从 crate 开始往下查找。

- 另一种就是使用 `use` 关键字，他就类似于其它语言的 `import`，此后要使用该路径的项时可直接省略前面的公用路径



惯用法：

`use` 还有一个用法就是将子模块的内容可以引入到本模块中，换句话说相当于提升了模块内方法的层级

在使用结构体、枚举时习惯用法是指定他们的完整路径，例如 `use std::collections::HashMap`

如果导入时有两个结构体或枚举名称相同（如 `std::fmt::Result` 和 `std::io::Result`），解决方案：

- 使用到父模块即可，即 `std::fmt` 和 `std::io`
- 给某一个模块起一个别名，即 `use ... as ...`





#### 公有结构体、枚举

Rust中声明的数据类型默认是私有的，要想将其公开则必须使用 `pub` 关键字。但即使如此结构体字段仍然是私有的，要想将其再次公开则必须在字段面前使用 `pub`

```rust
pub struct A {	// 公有结构体
    pub a: i32,	// 公有字段
    b: i64,		// 私有字段
}
```

但是枚举却不遵循这种规则，只要枚举体本身是公有的，则枚举字段也必然变公有

```rust
pub enum A {
	Q,	// 公有
    W,	// 公有
    E,	// 公有
}
```

