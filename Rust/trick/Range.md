### 普通范围

Rust 中的 Range 在 `std::ops::Range` 标准库中

```rust
let a: Range<i32> = 1..5;	// [1,5) 开区间
let a: Range<i32> = 1..=5;	// [1,5] 闭区间
```



### 开放范围

开放式范围在 `std::ops` 中



#### RangeFrom

`RangeFrom<T>`  只有起点没有终点，形式为 `n..`

```rust
let a: RangeFrom<i32> = 1..;
println!("{}", size_of_val(&a));	// -> 4 (i32)
```

`RangeFrom ` 的大小为 4（i32），因为只需要存储一个起点即可.

`RangeFrom` 可以直接用于 for 循环，如果 `for` 内部没有 `break` 则无限循环。

 

#### RangeTo

`RangeTo<T>`  只有起点没有终点，形式为 `..n`

```rust
let a: RangeTo<i64> = ..5i64;
println!("{}", size_of_val(&a));	// -> 8 (i64)
```

`RangeTo ` 的大小为 8（i64），因为只需要存储一个终点即可.

但有意思的是，`RangeTo` 却不能够直接用于 `for`，因为编译器不知道从何处开始，即使类型是 `RangeTo<u32>` 也不行，最关键的原因是因为它没有实现 `Iterator`，所有能够用于 `for`  循环的都必须实现该 `trait`。



#### RangeFull

`RangeFull` 代表一个全切片，它不会存储任何信息，因此其大小为0。

它的作用就是用于指定与基础序列完全相同的范围，因为我们有时候并不知道序列的确切范围但又想要得到全切片。

```rust
let a: RangeFull = ..;
println!("{}", size_of_val(&a));	// -> 0
```



### Bound

如果想获取范围的起点和终点可以用 `std::ops::Bound`，该库只对 `Range` 、`RangeFrom` 和 `RangeTo ` 有效。使用关联方法 `start_bound()` 获取起点，使用关联方法 `end_bound()` 获取终点

范围一共有三种：

- `Unbounded` 表示无界
- `Excluded(num)` 表示不包含 `num`
- `Included(num)` 表示包含 `num`

```rust
let a: Range<i32> = 1..5;
a.end_bound()	// ->Excluded(5)
a.start_bound()	// ->Included(1)

let a: Range<i32> = ..=5;
a.end_bound()	// ->Included(5)
a.start_bound()	// ->Unbounded
```

