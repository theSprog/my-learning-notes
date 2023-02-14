### 命名方式

- 函数、方法、命名空间采用蛇形命名法（snake_case）
- 自定义的类采用驼峰命名（CamelCase），成员变量加前缀 m_
- 全局变量加前缀 g_
- 宏定义常量采用蛇形全大写（SNAKE_CASE），



### 预处理

预处理本身只是文本替换，# 需要写在行首

- #### #include

  #include 不仅可以包含头文件，还可以包含任意文件，因为他只是简单的文本替换。可以用这种方法处理常量存储

  ```cpp
  // xxx.cpp
      
  static uint32_t abc[] {
  #include "xxx.inc"	// -> 会被替换为 1,2,3,4,5,6
  }
  ```

  ```cpp
  // xxx.inc
  1,2,3,4,5,6
  ```

- #### 用宏代替namespace

  ```cpp
  #define BEGIN_NAMESPACE(x) namespace x {
  
  #define END_NAMESPACE }
  
  BEGIN_NAMESPACE(abc)
      ...	// 命名空间内
  END_NAMESPACE
  ```

- #### __cplusplus

  这个宏定义了C++版本号，如果不是C++编译则该宏未定义

  ```cpp
  #ifdef __cplusplus		// 是否是C++环境
  
  
  #if __cplusplus	>= 201402	// C++ 14 版本号为 201402
  	...
  #elseif __cplusplus	>= 201103	// C++ 11 版本号为 201103
      ...
  #else 
      ...
  #endif
  ```

- #### 错误定位相关的宏

  ```cpp
  // 相关的宏有三个:__FUNCTION__, __FILE__, __LINE__
   
  int main(int, char**)
  {
      printf("%s:This fake error is in \"%s\" on line %d.\n", __FUNCTION__, ___FILE___, ___LINE___);
      return 0;
  }
  
  //运行结果
  main: This fake error is in "C:\test\test.cpp" on line 3.
  ```

- #### 查看是否支持某种特性

  ```cpp
  #if defined(__cpp_decltype_auto)	// 支持 delctype 特性
  	...
  #else
  	...
  #endif
  ```

- #### 根据操作系统不同来定制化配置

  ```cpp
  // 使用 shell 脚本检测操作系统，然后生成宏
  
  // 然后在选择性的包含头文件
  #if (FREEBSD)
  	...
  #elseif (LINUX)
  	...
  #elseif (DARWIN)
  	...
  #else
  	...
  #endif
  ```

- #### 使用宏禁用或启用某些代码段（尤其是DEBUG）

  ```cpp
  # define DEBUG 1	// 更改DEBUG的值来决定是否开启DEBUG
  
  #if DEBUG			// DEBUG 改为 0 关闭 DEBUG
  	...
  #endif
  ```



### 属性

属性是C++11引入的，之前的用法是 \__attribute__()

```cpp
extern void exit(int)   __attribute__((noreturn));
extern void abort(void) __attribute__((noreturn));

#include <stdio.h> 
__attribute__((constructor)) void before(void)
{
	printf("run first\n");
}
__attribute__((destructor)) void after(void)
{
	printf("run after\n");
}
int main(void)
{
	printf("I am main function\n");
	
	return 0;
}

// 输出结果
run first
I am main function
run after
```



C++11 的用法是两个中括号: [[...]]，C++11 中只定义了两个属性 `noreturn` 和 `carries_dependency`，C++14增加了 `deprecated`，C++17 又增加了几个，但仍然不够用。不过属性也支持非标准扩展，例如 gnu下就有许多属性

```cpp
[[deprecated("don‘t use me")]]	//声明该方法被废弃,编译时会给出 warning
int old_func();

[[gnu::deprecated]]	// gnu 下的deprecated可以用在 C++11 里
[[gnu::hot]]		// 热点代码，要求编译器优化
[[gnu::constructor]]	// 在 main() 之前执行
[[gnu::destructor]]		// 在 mian() 之后执行
...
    
```





### assert

- 动态assert，只有当程序运行到此处时才检查，assert里面的内容必须为 true，指针必须非空。否则就会 abort() 终止

  ```cpp
  assert(a > 0);
  ```

- 静态assert，编译时期编译器进行的检查，常用于模板元编程。需要注意的是，这个断言只能适用于编译器，无法判断只有运行时才能确定的事，如判断指针是否为空

  ```cpp
  #include <iostream>
  using namespace std;
   
  template<int N>
  struct fib
  {
      static_assert(N>=0, "N must be greater than 0");
      static const int value = fib<N-1>::value + fib<N-2>::value;
  };
  
  // 尾递归
  template<>
  struct fib<0>
  {
       static const int value = 1;
  };
   
  template<>
  struct fib<1>
  {
       static const int value = 1;
  };
   
  int main()
  {
      // 编译时就会求值，而不会等到运行时
      cout<<fib<3>::value<<endl;
      cout<<fib<4>::value<<endl;
      return 0;
  }
  ```



### 常用技巧

#### 委托构造

当你有很多构造函数，并且其中有大量重复的初始化代码时，为了避免重复，可以在使用委托构造函数，即在一个构造函数中调用另一个更全面的构造函数

```cpp
class Test final {
private:
    int data;
public:
    // 被委托的函数，即构造函数的公共部分
    Test(int data): data(data) {}
    
    Test(): data(0) {}
    
    Test(const string& s): data(stoi(s)) {}
}
```



#### 成员变量初始化

可以声明类的成员变量时，同时也给他赋值

```cpp
class Test {
private:
    int a = 0;
    string b = "hello";
    vector<int> c{1,2,3};
}
```



#### 类型别名

using关键字，相当于 typedef，但更加简单直观。当某个类型名字特别长，或者带有命名空间时，利用别名缩减信息长度

```cpp
using u8 = unsigned int;
```





### auto与decltype

- auto 可以自动推导赋值语句右侧的表达式的类型，更重要的是可以配合 `const` 、`violate` 、`&` 、`*` 来使用。**// todo 待补充**

- auto 暂时不能使用在类定义内部，不过可以用 decltype 代替。但是静态成员变量可以用 auto

- auto 会将字符串（例如“hello”）推导为 const char[...]，但在 C++14 可以使用 `auto s = "hello"s`(即后缀加上s)表示推导为 `string`

- `auto&` 什么变量都可以捕获，可以称得上是万能引用

  ```cpp
  int a = 10;
  int &b = a;
   
  auto c = b;//c的类型为int而非int&（去除引用）
  auto &d = b;//此时d的类型才为int&
  ```

- decltype 向编译器索取类型，可用于定义变量的时候，**//todo待补充**

  ```cpp
  int x = 0;
  
  decltype(x) x1;
  decltype(x)& x2;	// x2 为 int&
  decltype(x)* x3;	// x3 为 int*
  ```

- C++14新增了 `decltype(auto)`，已经相当于自动推测右侧类型用于使用，即使是引用也可以推导出来

  ```cpp
  int x = 0;
  
  decltype(auto) x1 = (x);	// (expr) 相当于引用类型
  decltype(auto) x2 = &x;  	// x2 为 int*
  decltype(auto) x3 = x1;		// x3 为 int&
  ```

- range时最佳实践是 `auto& ` 

  ```cpp
  for(auto& e : collection) {	// 如果不改变集合内容，甚至可以使用 const auto&
  	...
  }
  ```

- C++14 支持函数返回值使用 `auto`

  ```cpp
  auto func(){
  	...
  	return complex_value;
  }
  ```

- decltype推导函数指针

  ```cpp
  // 假设已有某个函数 func
  // 使用下面这种方式可以避免C语言中的繁杂的函数指针定义
  using func_ptr = decltype(&func);
  ```






### const

const并非常量，严格来说应该叫只读变量。与宏定义不同，宏在预处理阶段就会文本替换，然而`const`常量需要在运行时阶段才存在，而且我们可以通过获取该常量的指针，进而强行改变该常量的值

```cpp
#include<iostream>

using namespace std;

int main(){
	const volatile int max = 1024;	// volatile 是必须的
	auto ptr = (int*)(&max);
	*ptr = 2048;
	cout << max << endl;
}
```

volatile 是必须的，否则的话编译器会识别出 max 是常数并且将它优化掉，从而失去 max 是“只读变量”的性质。volatile 禁止编译器做任何的优化，每次需要取值时必须老老实实去取 max 的值



#### const 与指针的结合：

- const 在 * 的左边，代表指向的内容不能修改，但是指针可以指向别处

  ```cpp
  int a = 8;
  int b = 9;
  
  const int *p = &a;
  *p = 9;	// error
  p = &b;
  ```

- const 在 * 的右边，表示指针不能变，但是指向的内容可以变

  ```cpp
  int a = 8;
  int b = 9;
  
  int * const p = &a;
  *p = 9;	
  p = &b;	// error
  ```

- const 在 * 两边表示都不能修改



#### const 成员函数

const成员函数即在成员函数之后加上 const 关键字，本质是传入了一个 const 的 this 对象

```cpp
class A {
	...
	
    int func() const {
		...
    }
}

```

在C++中只有被声明为 const 的成员函数才能被一个 const 类对象调用，而 const 类对象不能调用非const成员函数，因为这可能会修改该类对象，从而破坏常量语义。

声明为 const 的成员函数不能改变类的成员变量，除非该变量是 mutable 的



#### mutable

mutable只能修饰类里面的成员对象，表示成员变量即使是在const对象里面时也可以被修改，因为程序员保证改变它不会修改类的对外状态。

比如说类内部需要一个 mutex 互斥量，但它是类的内部实现细节，改变与否不会破坏对外状态，所以可以声明为 mutable



### explicit

`explicit` 关键字只能用于修饰**只有一个参数**的类构造函数, 它的作用是表明该构造函数是显示的, 而非隐式的, 跟它相对应的另一个关键字是implicit, 意思是隐藏的,**类构造函数默认情况下即声明为implicit(隐式)**

```cpp

class CxString  // 没有使用explicit关键字的类声明, 即默认为隐式声明  
{  
public:  
    char *_pstr;  
    int _size;  
    CxString(int size=0)  
    {  
        _size = size;                // string的预设大小  
        _pstr = malloc(size + 1);    // 分配string的内存  
        memset(_pstr, 0, size + 1);  
    }   
    // 析构函数这里不讨论, 省略...  
};  
  
// 下面是调用:  
CxString string1(24);     // 这样是OK的, 为CxString预分配24字节的大小的内存  
CxString string2 = 10;    // 这样是OK的, 为CxString预分配10字节的大小的内存  
```

`CxString string2 = 10;`  虽然让人迷惑，但却是编译器允许的行为，因为在C++中, 如果的构造函数**只有一个参数**（**除了第一个参数以外的其他参数都有默认值的时候，也算只有一个参数**）时, 那么在编译的时候就会有一个缺省的转换操作 : 将该构造函数对应数据类型的数据转换为该类对象. 也就是说 "`CxString string2 = 10;`" 这段代码, 编译器自动将 `int` 转换为 `CxString` 类对象，实际上等同于下面的操作

```cpp
CxString string2(10); 
```

而使用 `explicit` 关键字可以禁止这种转化。





### 智能指针

- #### unique_ptr

  - unique_ptr 实现了 * 与 -> 操作，正常使用的话和指针一模一样。但是它其实不是一个指针，而是一个对象，所以不要对它调用 delete 操作，而且也不能进行加减运算.

  - unique_ptr 不能直接引用一个字面量，但是在 C++ 14 时可以使用 make_unique()
  - unique_ptr 需要独占一个资源，也就是不能有另一个指针同时指向该对象，否则报错
  - unique_ptr 由于是单个变量独享资源，所以资源只能转让，不能共享。转让时使用 move 函数，转让后原来的指针就变成了空指针

  ```cpp
  #include <memory>
  
  int a = 10;
  
  // 原始写法
  int* p = &a;
  
  // 智能指针写法
  unique_ptr<int> p(new int(10));
  
  int b = 20;
  unique_ptr<int> p2(&b);
  
  auto p3 = make_unique(42);	//	C++ 14
  
  auto p4 = make_unique(42);
  auto p5 = move(p4);	// p4 的资源转移到 p5
  ```

- #### share_ptr

  - share_ptr 与 unique_ptr 最大的不同就是 share_ptr 可以多个变量共享资源，从这一点上 share_ptr 完全可以代替裸指针，share_ptr 可以直接赋值（=）给另一个指针，不需要像 unique_ptr 那样使用 move

  - share_ptr 内部使用引用计数，可以调用 unique() 查看是否独占，用 use_count() 查看引用计数

  - 使用引用计数，就必然会面临循环引用的问题，即两个对象互相引用，导致引用计数不归零，内存泄露

    ```cpp
    auto p = make_share(42);
    ```





### 异常类

C++的异常继承体系如下

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201122022582.png)



可以自己定义异常，只需要去继承其中的一个类即可

在抛出异常时，不要直接 `throw`，可以使用一个中间层（例如 raise）来获得正好的安全性和灵活性



#### function-try

即把整个函数体视为一个块，在编写完代码逻辑后再编写异常逻辑

```cpp
void func() 
try {
    ...
}catch (exception& e) {
    ...
}
```



#### noexcept

对编译器保证我不会抛出异常，我也不会去处理异常，编译器应该利用这个信息做优化。如果真的有异常发生，我会直接崩溃（crash）。

可以在 `noexcept` 加上条件表达式，指定只有在某些条件成立时我才不会抛出异常，而直接使用 `noexcept`  相当于 `noexcept(true)`  

一般而言构造函数尽量声明为 `noexcept`，析构函数必须保证不抛出异常



### lambda

语法：

```cpp
[捕获变量列表](参数列表){代码块}

auto f = [&abc] (int n) {
      cout << abc << n << endl;
 }
```

- 捕获变量前要么用 `&`引用捕获，要么用 `=`值捕获，也可以只写一个 `[=]` 表示全部按照值捕获，`[&]` 表示全部按照引用捕获

- 变量的类型使用 auto 推断

- 当引用捕获的时候必须考虑变量的生命周期，如果变量已经死亡，再调用 `lambda` 函数就会发生错误

- C++14 可以使用泛式 `lambda`，捕获任意类型的变量

  ```cpp
  auto triple = [](auto& s) {	// 如果 s 不可变还可以加 const 修饰
  	return s + s + s;
  }
  ```

