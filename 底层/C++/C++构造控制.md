### C++ 中拷贝构造的时机

```cpp
#include<iostream>

using namespace std;
class A {
  public:
	A(){ cout << "A cons" << endl;}
	~A() {cout << "A decons" << endl;}
	A(const A&) {cout << "A copy cons" << endl;}
};

// 返回值有两种选择：A 和 A&&
A getTemp()	
{
    return A();	// A() 是一个右值（临时变量）
}


int main() {
	cout << "begin" << endl;
    
    // 左值有两种选择：A 和 A&&
	A a = getTemp();   //getTemp()的返回值是右值（临时变量）
    
	cout << "end;" << endl;
}
```



在编译参数加入 `-fno-elide-constructors`，禁用构造优化

```shell
$ g++ main.cpp -fno-elide-constructors -o main
```



运行结果如下：

- 函数 getTemp() 使用A，左值类型 A

  ```cpp
  // begin
  // A cons
  // A copy cons		-> 函数 return 时调用拷贝构造，构造临时对象
  // A decons			-> 临时对象A()析构
  // A copy cons		-> 构造 A a 时调用拷贝构造
  // A decons			-> getTemp() 临时对象析构
  // end;
  // A decons
  ```

- 函数getTemp() 使用A&&，左值类型 A&&

  ```cpp
  // begin
  // A cons		
  // A decons		-> 临时对象A()析构, 同时相当于 A a 在该句结束时就已经析构了，不能再使用
  // end;
  ```

  

- 函数getTemp() 使用A，左值类型 A&&（**推荐**）

  ```cpp
  // begin
  // A cons
  // A copy cons
  // A decons		-> 临时对象A()析构，由于临时对象已经在上一步被拷贝，所以可以放心析构
  // end;
  // A decons
  ```

  

- 函数getTemp() 使用A&&，左值类型 A

  ```cpp
  // begin
  // A cons	
  // A decons			-> 临时对象A()析构
  // A copy cons		-> A a 时调用拷贝构造, 但 A() 已经析构了，这里其实是错误的构造，没报错是没有使用other
  // end;
  // A decons
  ```



在本实验的演示中推荐使用第三种，即 getTemp() 逃逸对象，但引用时却不拷贝构造，而是直接右值引用。

第二和第四种直接就是错误的用法，而第一种会额外多一次拷贝操作





