### 来源

我们知道，函数式编程里面的函数其实是没有名称的，我们对函数的某个命名其实是一个记号

```C
let f = λx.2*x

f (5)
```

其实是编译器提供的语法糖，避免重复复制粘贴。上面的运算相当于是

```C
λx.2*x (5)
```



然而这样的运算还有一个缺陷需要完善，那就是递归函数调用自身，对于类似于 C 的语言而言

```C
int f(int n) {
	return n == 1 ? 1 : n * f(n-1);
}
```

它可以通过定义 `f` 来递归调用自己。

然而我们知道，函数式编程的函数都是匿名函数，既然没有名字，那么该如何调用自身呢

```C
λx. x == 1 ? 1 : x * (??? x-1)	// what is ???
```



### 参数化函数

一个经典的想法就是，将本函数变为参数 `f`，通过 `f` 来表示本函数，进而调用 `f(f)(x-1)`

```C
λf.λx. x == 1 ? 1 : x * (f(f)(x-1))
```

对于上面这个函数，将之记号为 `K`（注意这只是记号，而非定义，换句话说这只是一种简写方式），那么必然有

```C
(λf.λx. x == 1 ? 1 : x * (f(f)(x-1))) (λf.λx. x == 1 ? 1 : x * (f(f)(x-1))) (5)

//简记为 K
K K 5	// = 120 = f(5)
```

对于任何递归函数，我们都可以如此操作。其实到此为止我们就解决了递归的问题，只不过这个方法的抽象性不足，我们希望的是能否找到一个通用函数 Y，使得 **Y 作用于普通函数后将第一个参数代指本函数，以用于递归**

效果如下

```C
// 此处用 f 即可代指本函数的抽象，即f(x)，而非f(f)用法，使之类似于C的用法
// 后面的x是参数，以此类推还可以有 λy, λz 等等
Y λf.λx. x == 0 ? 1 : x * f(x - 1)	
// 将其记号为 F
F = Y λf.λx. x == 0 ? 1 : x * f(x - 1)
F(5) // 120
```

寻找 Y 使得 `f` 可代指本函数 `f(x)` 进而可在 `f(x)` 内递归调用，例如 `f(x) = f(x-1) + f(x-2)`

```C
Y λf.λx. x==1 || x==0 ? 1 : f(x-1) + f(x-2)
```



### 寻找Y组合子

我们从 `K = λf.λx. f(f)(x)` 开始，已知最终的结果函数为 `res = K K`，不妨设 `res = λv. v v K`

将 `f(f)` 提取为 `g` 可得

```c
λf.( λg.λx.g(x) f(f) )
```

我们发现 `λg.λx.g(x)` 的形式就像我们所要求的抽象形式，将之提取为 `h` 可得

```C
λh.λf.( h f(f) ) (λg.λx.g(x))
```

将 `λg.λx.g(x)` 记号为 `G`

```
res = (λv. v v) (λh.λf.( h f(f) ) G)
```

将 `G` 再提出一层，剩下的就是 Y 组合子

```C
res = λs.(λv. v v) (λh.λf.( h f(f) ) s) G
// 化简
res = λs.(λv. v v) (λf.( s f(f) )) G
```

将 `f` 替换为 `x`，`s` 替换为 `f` 由此可得 Y 组合子的定义
$$
Y = \lambda f.(\lambda v.vv)(\lambda x.f(xx))
$$


### 数学验证

假设函数为 `g`，被 `Y` 作用后
$$
\begin{equation*}
    \begin{aligned}
        Y g&=\lambda f \cdot(\lambda x \cdot f(x x))(\lambda x \cdot f(x x)) g\\
        &=\lambda x \cdot g(x x))(\lambda x \cdot g(x x)\\
        &=g((\lambda x \cdot g(x x))(\lambda x \cdot g(x x)))\\
        &=g(Y g)
    \end{aligned}
\end{equation*}
$$


### python验证

使用 python 代码写出 Y 组合子如下

```python
Y = lambda f: (lambda u: u(u))(lambda x: f(x(x)))
```

不过 `f(f)` 有可能会栈递归爆炸，所以将其变化

```python
Y = lambda f: (lambda u: u(u))(lambda x: f(lambda y: x(x)(y)))
```

尝试运用 Y 组合子到一个普通函数上

```python
# f 可代指自己了
fib = Y(lambda f: lambda x: 1 if (x == 0 or x == 1) else f(x-1) + f(x-2))
print(fib(5))	# -> 8
```

