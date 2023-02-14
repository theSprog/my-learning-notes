### 实验

我们除了可以使用系统自带的动态链接库外，还可以自己制作动态链接库

例如：

```c
// share.c 此文件用于制作动态链接库
int sub(int a, int b)
{
   return a-b;
}
```

编译制作动态库：

```shell
$ gcc -fPIC -shared -o share.so share.c	# 将名称命名为 share.so
```



在另一处手动调用此函数：

```c
// test.c
#include<stdio.h>
#include<dlfcn.h>	// 要使用动态库必须导入该头文件

typedef int (* fn)(int, int);

int main(){
	void *handle; 	// 定义句柄，他是一个 void*
	int a = 5, b = 1;
	
    // dlopen 函数第一个参数代表动态库路径，第二个参数代表 flag
    // flag：分为这四种 
    // RTLD_NOW：在dlopen返回前，解析出全部没有定义的符号，解析不出来返回NULL。
    // RT_GLOBAL：动态库定义的符号可被其后打开的其他库解析。
    // RT_LOCAL：和上面相反，不能被其他库解析。默认。
    // RTLD_LAZY：暂缓决定，等有需要时再解出符号

	handle = dlopen ("./share.so", RTLD_LAZY);	// 使用懒加载方式
	fn sub = (fn)dlsym(handle, "sub"); 			// 在句柄里拿出函数 sub,为此我们需要知道 sub 函数的类型
	printf("a - b = %d\n", sub(a, b));
	dlclose(handle);							// 使用完后必须关闭句柄，否则造成资源泄露

	return 0;
}

// 另外 <dlfcn.h> 下还有一个函数 const char *dlerror();
// 当动态链接库操作函数执行失败时，调用dlerror可以返回出错信息，
// 若该函数返回值为NULL时表示操作函数执行成功。
```

编译运行：

```shell
$gcc -o test test.c -ldl
```

注意由于用到了动态链接器，我们需要 `libdl` 动态库(dlsym)，所以必须加上 `-ldl` 选项.



### 分析

我们调用了动态链接库里面的函数，我们怎么知道动态链接库里面的函数的地址呢？事实上，直到我们第一次调用这个函数，我们并不知道这个函数的地址，这个功能要做延迟绑定 lazy bind。

动态链接库中维护一个动态符号表，动态符号表 (.dynsym) 用来保存与动态链接相关的导入导出符号，不包括模块内部的符号。而 .symtab 则保存所有符号，包括 .dynsym 中的符号。

我们可以用 `readelf`  查看 `.dynsym` 表，

```shell
$ readelf --syms share.so	# 这种方式查看的是所有符号，也包括动态符号
$ readelf --dyn-syms share.so	# 这种方式只查看动态符号
# 输出如下
Symbol table '.dynsym' contains 6 entries:
   Num:    Value          Size Type    Bind   Vis      Ndx Name
     0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 0000000000000000     0 NOTYPE  WEAK   DEFAULT  UND __cxa_finalize
     2: 0000000000000000     0 NOTYPE  WEAK   DEFAULT  UND _ITM_registerTMCloneTable
     3: 0000000000000000     0 NOTYPE  WEAK   DEFAULT  UND _ITM_deregisterTMCloneTab
     4: 0000000000000000     0 NOTYPE  WEAK   DEFAULT  UND __gmon_start__
     5: 00000000000010f9    22 FUNC    GLOBAL DEFAULT   10 sub
```



而当我们反汇编查看 `share.so` 时恰好发现

```assembly
00000000000010f9 <sub>:
    10f9:       f3 0f 1e fa             endbr64
    10fd:       55                      push   %rbp
    10fe:       48 89 e5                mov    %rsp,%rbp
    1101:       89 7d fc                mov    %edi,-0x4(%rbp)
    1104:       89 75 f8                mov    %esi,-0x8(%rbp)
    1107:       8b 45 fc                mov    -0x4(%rbp),%eax
    110a:       2b 45 f8                sub    -0x8(%rbp),%eax
    110d:       5d                      pop    %rbp
    110e:       c3                      retq
```

由此可见，只要保存了动态符号表，当其他程序通过函数名称调用函数时，可以通过符号表`Name` 字段反查 `Value` 字段进而找到该函数的入口地址，完成函数调用