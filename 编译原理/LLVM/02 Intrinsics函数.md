## Intrinsic函数

Intrinsic函数相当于是 LLVM 提供的内联函数，与普通的内联不同的是，编译器提供的内联更懂得如何利用硬件，效率更高。这些函数往往保留了前缀 `llvm.`



例如说

```c
int foo(void){
    char str[10] = "str";
    return 0;
}
```

编译为 IR 是：

```
define dso_local i32 @foo() #0 {
  %1 = alloca [10 x i8], align 1
  %2 = bitcast [10 x i8]* %1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %2, i8* align 1 getelementptr inbounds ([10 x i8], [10 x i8]* @__const.foo.str, i32 0, i32 0), i64 10, i1 false)
  ret i32 0
}
```

`llvm.memcpy.p0i8.p0i8.i64` 就相当于 Intrinsic 函数。如果没有这种优化，那么反应到 IR 中就是大量的 store/load。