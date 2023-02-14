### 1、打印优化级别对应的选项开关

使用`gcc -Q --help=optimizers -O` 可以查看对应的优化级别的优化选项是否打开

```shell
$ gcc -Q --help=optimizers -O
$ gcc -Q --help=optimizers -O1
$ gcc -Q --help=optimizers -O2
$ gcc -Q --help=optimizers -O3
$ gcc -Q --help=optimizers -Os
$ gcc -Q --help=optimizers -Ofast
```



### 2、打印编译时搜索头文件的顺序

```shell
$ gcc -V a.c

ignoring nonexistent directory "/usr/local/include/x86_64-linux-gnu"
ignoring nonexistent directory "/usr/lib/gcc/x86_64-linux-gnu/9/include-fixed"
ignoring nonexistent directory "/usr/lib/gcc/x86_64-linux-gnu/9/../../../../x86_64-linux-gnu/include"
#include "..." search starts here:
#include <...> search starts here:
 /usr/lib/gcc/x86_64-linux-gnu/9/include
 /usr/local/include
 /usr/include/x86_64-linux-gnu
 /usr/include
End of search list.
```



### 3、根据链接库名称找库的路径

```shell
$ gcc -print-file-name=<lib_name>

i.e.
$ gcc -print-file-name=libc.a
/usr/lib/gcc/x86_64-linux-gnu/9/../../../x86_64-linux-gnu/libc.a
```



### 4、使用命令行定义宏，取消宏

有时我们会根据是否定义一个宏来决定程序执行版本，例如定义`DEBUG`来进行调试编译，我们可以在命令行中而不是在文件中定义该宏(使用`-D`选项)

```shell
$ gcc -D <MACRO> <file>
$ gcc -D <MACRO>=xxx <file>
```

例如

```c
// test.c
#include <stdio.h>

int main(){
	#ifdef DEBUG
		printf("DEBUG");
	#else
		printf("NODEBUG");
	#endif
	return 0;
}
```

```shell
$ gcc -D DEBUG test.c
DEBUG
# -D 之后也可以不用空格
$ gcc -DDEBUG test.c
```



类似的，我们可以用 -U 命令取消某一个宏定义。需要注意的是，这个`-U`是用来取消某个`-D`宏定义的，不能取消文件内定义的宏

```shell
$ gcc -D DEBUG -U DEBUG test.c
```





### 5、将选项传递给汇编器，链接器

可以用一行命令，而非多行，控制汇编器、链接器行为。`-Wa,<option>`和 `-Wl,<option>`，注意逗号与选项之间不能有空格

```shell
$ gcc -Wa,<option> <file>
$ gcc -Wl,<option> <file>

i.e.
$ gcc -c -Wa,-L test.c
```



### 6、生成带有详细信息的汇编

有时我们想要在汇编中知道哪些语句对应原文件的什么变量，在哪一行之类的信息，即天生带有注释的汇编文件，这时我们就可以选项使用 `-fverbose-asm`

```shell
$ gcc -S -fverbose-asm <file> # -S 是生成汇编文件的意思，verbose:罗嗦的，冗长的
```





### 7、使用Sanitizer检测越界、内存泄漏和内存访问错误

gcc从`4.8`版本起，集成了`Address Sanitizer`工具，可以用来检查内存访问的错误（编译时指定“`-fsanitize=address`”）

当我们执行程序时，如果发生了数组越界、内存泄漏或者非法内存访问（野指针），就会报错

```shell
$ gcc -fsanitize=address <file>
```

例如

```c
// test.c
#include <stdio.h>
#include <stdlib.h>

int* callLeak(){
	int* a = (int*)malloc(sizeof(int));
	return a;		
}

int main() {
    int* a = callLeak();	// fotget to free!!
	return 0;
}
```

```shell
gcc -fsanitize=address test.c && ./a.out

=================================================================
==326==ERROR: LeakSanitizer: detected memory leaks

Direct leak of 4 byte(s) in 1 object(s) allocated from:
    #0 0x7f79636afbc8 in malloc (/lib/x86_64-linux-gnu/libasan.so.5+0x10dbc8)
    #1 0x55614eed219e in callLeak (/mnt/e/king-os/test/a.out+0x119e)
    #2 0x55614eed21be in main (/mnt/e/king-os/test/a.out+0x11be)
    #3 0x7f79633d70b2 in __libc_start_main (/lib/x86_64-linux-gnu/libc.so.6+0x270b2)

SUMMARY: AddressSanitizer: 10 byte(s) leaked in 1 allocation(s).
```

对于专门检测内存泄漏，也可使用 `-fsanitize=leak` 来检测，但 `-fsanitize=address` 也已经够用





### 8、使用Sanitizer检测数据竞争

`Sanitizer` 可以用来检查数据竞争的问题（编译时指定“`-fsanitize=thread -fPIE -pie`”）。不过好像只有当数据竞争发生时才会报告，不能未雨绸缪，感觉不是很实用



### 9、禁止某个函数被编译器优化掉

在该函数签名前加上函数属性`__attribute__ ((__used__))`



### 10、强制函数以 inline 形式调用

在该函数签名前加上函数属性`__attribute__ ((always_inline))`



### 11、保存临时文件

使用选项`-save-temps`可以保存gcc运行过程中生成的临时中间文件



### 12、打开\关闭警告信息

使用`-Wall`选项打开所有的警告信息，使用 `-w` 不生成任何警告信息



### 13、改变地址对齐

使用`-fpack-struct[=n]`选项（“`n`”需要为`2`的倍数）可以改变成员的地址对齐，当不指定 `n` 时默认没有填充字节，所有成员将一个挨着一个排在一起。（有时候这个编译选项会导致`ABI`的改变，所以使用时一定要谨慎）



### 14、添加头文件与库文件路径

`-l dir`:在**头文件**的搜索路径列表中添加 `dir` 目录

`-L dir`:在**库文件**（静态库，动态库）的搜索路径列表中添加 `dir` 目录



### 15、制作动态库与静态库

#### 静态库

使用 `ar` 命令，目标文件以 `lib` 开头，以 `.a` 为后缀，中间是静态库名称

```shell
$ ar -crv lib<filename>.a <file1> <file2> ...	# file1、file2 均是 .o 文件
```

需要使用时用 `-static` 指明静态链接，`-L` 指明静态库路径，`-l` 指明静态库名称

```shell
$ ar -crv libx.a f1.o f2.o
$ gcc main.c -static -L . -lx	# -L . 表明静态库在本目录下，-l 表明静态库名称为 x	(不是libx.a)
```



#### 动态库

直接使用 `gcc` 加特定参数生成动态库，目标文件以 `lib` 开头，以 `.so` 为后缀，中间是动态库名称

```shell
$gcc -fPIC -shared -o lib<filename>.so <file1>.c <file2>.c	# 直接由 .c 文件得来

# 使用时, -L 指定库路径
# 按照规则，动态链接库必须放在最后以完成符号解析
$ gcc main.c -L . -l<filename>	# 不用指定 -static，-l 默认就是动态链接
```

需要注意的是，linux可执行文件在运行的时候默认是去默认共享库文件寻找（默认没有当前目录），所以要将so文件添加到lib路径。可以用 `ldd` 用于打印程序所依赖的共享库存放位置。windows则相反，加载动态库的时候，默认是首先加载本地目录下的动态库



其实，linux也可以支持“加载当前目录的动态库”。只要设置合适的环境变量 `LD_LIBRARY_PATH` 就可以了

1. 临时修改，log out之后就失效：

   ​	在terminal中执行：`export LD_LIBRARY_PATH=./`

2. 让当前帐号以后都优先加载当前目录的动态库：

   ​	修改 ~/.bash_profile 在文件末尾加上两行： `LD_LIBRARY_PATH=./` 和 `export LD_LIBRARY_PATH` 

3. 让所有帐号从此都优先加载当前目录的动态库：

   ​	修改 /etc/profile 在文件末尾加上两行：`LD_LIBRARY_PATH=./`  和  `export LD_LIBRARY_PATH` 

4. 也可以配置 `ldconfig` 寻找路径：（**注意此方法只支持绝对路径**）

   1. 进入目录 `/etc/ld.so.conf.d/`
   2. 创建 `*.conf` 文件，文件名随意，扩展名必须为 `.conf`
   3. 在文件内部，添加 `.so` 文件的路径（只需要写路径即可），保存并退出
   4. `sudo ldconfig`





### 16、指定汇编格式

使用 `-masm=intel` 指定生成的汇编以 `intel` 格式保存

```shell
$gcc -S -masm=intel hello.c
```

同样的，反汇编的时候（`objdump`）使用 `-M intel` 表示反汇编成 `intel` 格式的汇编

```shell
$gcc -M intel -d hello.o
```





### 17、读取、剥离符号信息

使用 `readelf --syms <file>` 可以查看 ELF 文件的符号表

gcc 默认不会剥离二进制文件的符号信息。但在有些时候我们不需要这些符号信息或者不想让他人逆向破解我们的程序，就需要将有用的符号信息剥离。一方面可以减少文件大小，另一方面可以加大逆向难度。

```shell
$ strip --strip-all a.out	# --strip-all 表示剥离全部符号，strip 还可以有其他选择
```

但是即便符号被剥离后仍然保留有一些必要的动态依赖符号，用于解析程序运行时对动态库的引用。而 `.dynsym` 就是表达这个的





### 18、预处理

gcc 预处理以 `-E` 选项表示，也可以只用 `cpp` 命令手动调用.

预处理后的文件一般以 `.i` 结尾

```shell
$ gcc -E hello.c -o hello.i
$ cpp hello.c > hello.i	# cpp 默认输出为 stdout, > 重定向到 文件
```

预处理要做的事情包括

1. 展开所有的宏定义，包括 #`define，#if，#endif` 等，但是不包括 `#program`，因为后面编译器还要使用该信息
2. 将 `#include` 包含的文件内容全部复制到本文件对应地方，这个过程可以递归实现。所以为了避免循环 `include`，需要做好`include guard`，即 `#ifndef xxx #define xxx <content> #endif` 或者 `#pragma once`
3. 添加文件名和行号标识，以便编译器产生有用的调试信息



