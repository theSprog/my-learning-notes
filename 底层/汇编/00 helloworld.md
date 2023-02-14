### Hello World

```assembly
section .text
	global _start2
_start2:
	mov rax, 1		; write(
	mov rdi, 1		;	stdout
	mov rsi, msg_len;	msg
	mov rdx, 12		;	12
	syscall			;)

	mov rax, 60		; exit(
	mov rdi, 0		; 0
	syscall			;)

section .data
	msg db "hello world", 10	; 10 是换行
	msg_len equ 12
	
section .bss
```

一份汇编代码主要包含三个部分 `.text`，`.data`， `.bss`



#### .text

`.text` 主要是代码段，用于存放可执行代码。`global` 是导出符号，汇编中符号默认是本地符号，不会导出，将某个符号声明为 `global` 后即可将其导出到符号表，用于后面的链接。

当参数少于7个时， 参数从左到右放入寄存器: `rdi`, `rsi`, `rdx`, `rcx`, `r8`, `r9`。而上面的 `rax` 其实是存放系统调用号，可以理解为于函数入口点。其中 `1` 代表 `write`， `60` 代表 `exit`（仅在 `linux` 环境下）



#### .data

`.data` 存放已初始化数据，格式是 `<name> <type> <value>`。 `<name>`  与 `<type>` 之间不必有冒号，定义的符号默认是 `local` 属性的，能够在符号中查到这个符号。符号的使用就是地址，即 `msg` 指向 `h`，是 `hello world` 的第一个字节。

| type | size |
| ---- | ---- |
| db   | 8    |
| dw   | 16   |
| dd   | 32   |
| dq   | 64   |

在该段内还可以使用 `equ` 伪指令，它不是在内存中的一个值，而会被汇编器展开为常量，不需要重定位（符号类型`ABS`）。

当然并不一定必须要在这一段才能使用该伪指令，事实上任意一个段内都能使用。



#### .bss

`.bss` 保存未初始化的变量，这种变量的使用格式是 `<name> <type> <len>`，其中的 `len` 代表要定义多少个前面这种类型的空间。例如

```assembly
dArr resb 20
```

上面这个指令定义了一个的数组，数组长度为 20，单个元素大小为 8.

| type | size |
| ---- | ---- |
| resb | 8    |
| resw | 16   |
| resd | 32   |
| resq | 64   |



### 编译与链接

首先将汇编文件编译为可重定位文件

```shell
$ nasm -g -f elf64 -o hello.o hello.asm -l hello.lst
```

`-f` 指定文件格式，`-l` 产生临时文件，用于展示源代码和二进制指令的对应关系和内存地址



然后将其链接

```shell
$ ld -e _start2 -o hello hello.o
```

`-e` 指定入口地址，一般来讲如果不指定的话默认 `_start`是入口，如果 `.o` 文件中没有这个符号的话链接器会显示符号未定义并停止工作，如果我们手动指定入口的话，链接器就不会报错

```shell
$ nm ./hello

00000000006000e4 D __bss_start
00000000006000e4 D _edata
00000000006000e8 D _end
00000000004000b0 T _start2
00000000006000d8 d msg
000000000000000c a msg_len
```

上面可以看到，我们定义的 `_start2`、`msg`和 `msg_len`都会被记录，而且类型是 **全局Text**、**局部data**和**abs**



