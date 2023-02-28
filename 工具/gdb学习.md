### 基本常识

要想程序可被调试，编译时需要加上 `-g` 选项，例如 `gcc -g`



### 基本命令

#### 调试某个程序

- gdb + 程序名



#### 运行与退出

- r：运行。r 后面可以带参数，被视为程序的入口参数
- start：运行到main入口，然后stop
- starti：运行到第一条指令，然后stop
- c:  继续运行，直到下一个断点
- finish:  跳出当前函数
- until:跳出循环
- q：退出
- fin/finish：跳出函数
- return \<expr>：跳出函数并且设置返回一个表达式。也可以单独使用 return



#### 查看源文件

- l	// 一次显示10行，多次执行显示多行



#### 打断点

- b + 函数名	// 在函数处打断点，
  - // 如果有函数重载那么会同时在多个重载函数上打断点。
  - // 如果需要指定重载函数，要在函数签名带上参数类型信息。比如：
  - //  `f()` 和 `f(int x)` ，要在 `f(int x)` 打断点：`b f(int)`
- b + 行号	   // 在行号处打断点
- b + 行号/函数 if cond    // 若cond成立就加断点



#### 查看断点

- info + b



#### 删除断点

- clear N	// 删除第N行的断点
- delete N (简写 `d N`)  // 删除N号断点
- delete   // 删除所有断点



#### 查看函数栈情况

- bt    // 这也可以用来查看当前执行到哪一行，bt 即 backtrace



#### 单步执行和单条执行

- n    // 单步执行，若有函数会直接把函数当成一条语句
- s    // 单条执行，若有函数会直接进入函数内部



#### 打印表达式的值

- p + 表达式    // 这里的表达式可以是变量
- **技巧**：p 可以调用一个表达式，而我们可用通过表达式的副作用来修改运行时的程序，从而动态影响程序的运行



#### 查看内存

`x` 可用于查看内存，后跟 `/<num>` 表示要查看多少单位内存。一单位 = 4字节（不一定，与指令集有关）

```shell
$x/16 0x12345678
```



#### 反汇编

- gdb反汇编可用`disassemble` / `disas` 命令
- disas \<Function>[, +length]：指定反汇编函数，指定 length 则只反汇编 length 字节，注意 `+`  不可省略。长度末端卡在一条指令中间时会反汇编该指令，确保指令的完整性
- disas \<Addr>[, +length]：仅指定地址时，gdb会自动查找该地址属于那个函数，进而反汇编函数。指定 length 则只反汇编 length 字节
- disas \<start>, \<end>：指定要反汇编的起始地址和结束地址。长度末端卡在一条指令中间时会反汇编该指令，确保指令的完整性
- show disassembly-flavor：显示当前汇编输出格式，默认是 AT&T 格式
- set disassembly-flavor intel/att：设置汇编格式为 intel/att
- x/i \<function>：x也是查看命令，默认是查看内存。加入 `/i` 后以指令形式查看内存（如果该内存无法翻译为指令则显示 `bad` ）。`i` 前面还可以添加数字，以查看若干条指令





### 技巧命令

#### shell + 命令用于在gdb中执行shell里的命令

- e.g. `shell ls`

​	

#### 内存映射

- 当程序运行起来后，使用 `i proc mappings`查看进程的内存映射信息，`i` 是 `info` 的缩写
- 也可以在程序运行起来后在 `gdb` 执行 `shell` 命令：`shell cat /proc/<进程id>/maps`
- cat是从文件第一行打印到文件最后一行，所以打印的内存映像是倒过来的，我们一般习惯正着的内存映像。所以使用 `tac`：`shell tac/proc/<进程id>/maps`



#### 日志功能

- set logging on	// 这会将输出复制到 gdb.txt 中

#### watchpoint和catchpoint

- watch + 变量	// 直到变量改变才程序停止， `info + watch` 查看watchpoint信息（简写为 wa）
  - 同样的，该 watch 也能后缀 if 表达式
- catch + event   // 用捕捉断点监控某一事件的发生，等同于在程序中该事件发生的位置打普通断点。
  - 例如：`catch throw Exception` 	// 指定捕获`throw Exception`事件
  - 常见的 event 包括：throw、catch、load、unload

#### 调试多进程

- 查看进程信息，`info inferiors`
- 默认多进程时调试的是父进程，显式指定调试父进程用 `set follow-fork-mode parent`
- 指定调试子进程用 `set follow-fork-mode child`
- 设置调试模式：即调试当前进程时其他进程是否运行,`set detach-on-fork [on|off]`
  - 默认是 on，即调试当前进程时其他进程继续运行
  - 设置为 off，则调试当前进程时其他进程被挂起
- 切换到某个进程 `inferior <进程ID>`

#### 调试多线程

- shell模式下，使用命令 `ps -aL | grep <程序名称>` 查看程序内的所有线程，第一行即主线程
- shell模式下，使用命令 `pstree -p <主线程id>`， 查看线程派生树
- 查看线程信息，`info threads`
- 切换线程， `thread <线程id>`
- 设置调试模式：即调试当前线程时其他线程是否运行，`set scheduler-locking [on|off]`
  - 默认是 off，即调试当前线程时其他线程继续运行
  - 设置为 on，则调试当前线程时其他线程被挂起
- 指定某线程执行gdb命令：`thread apply <线程id> <command>`
- 指定全部线程执行gdb命令：`thread apply all <command>`

#### layout 可视化显示

- 显示源代码窗口：layout src
- 显示汇编代码窗口：layout asm
- 显示寄存器窗口：layout regs
- 显示源代码和汇编代码：layout split
- 切换到下/上一个layout：layout next/prev
- 切换到某个layout：focus cmd/src/asm/regs/next/prev
- 刷新所有窗口：refresh
- 更新源代码窗口和当前执行点：update
- 调整name窗口的高度：winheight name +/- line
- 退出 layout：Ctrl + x，再按a，回到传统模式，即回到执行layout之前的调试窗口

#### 时间旅行

记录下当前和之后状态机状态，方便需要向后/向前执行

- record full：开始记录
- record stop：结束记录
- reverse-step(rs)：单步跳过（step over）的回溯
- reverse-next(rn): 单步执行（step next）的回溯
- reverse-stepi(rsi)：单步调试（step in）的回溯

#### .gdbinit脚本

- 在使用gdb调试程序的时候，有时候需要设定多个断点，重复执行某些操作，而这些操作写起来比较麻烦，这个时候就应该想起来用gdb命令脚本了，它能够很好的完成这些工作。

- gdb脚本的名称叫做 `.gdbinit` .在里面写上初始时需要的配置。有两种方式来启动脚本

  - 启动 gdb 时

    gdb在启动的时候，会在**当前目录**下查找`.gdbinit`这个文件，并把它的内容作为gdb命令进行解释。适合于单调试目标时。

  - gdb运行期间

    可以在gdb内使用 `source <script-file>` 来解释gdb命令脚本script-file。适合于多调试目标时

- 一个好用的 gdb 脚本

  ```shell
  #文件路径： ~/.gdbinit
  
  # 加上这句允许 gdb 在自己启动时的目录查找 .gdbinit 文件
  set auto-load safe-path /
  
  # 保存历史命令
  # 这个文件会越来越大，注意及时清理 
  set history filename ./.gdb_history
  set history save on
   
  # 记录执行gdb的过程
  # 这个文件会越来越大，注意及时清理 
  set logging file ./.log.txt
  set logging on
   
   
  # 退出时不显示提示信息
  set confirm off
  
  #有时当gdb输出信息较多时，gdb会暂停输出
  #并会打印“---Type <return> to continue, or q <return> to quit---”这样的提示信息
  #用下面的命令将其禁止
  set pagination off
   
   
  # 打印数组的索引下标
  set print array-indexes on
  ```

  

### 跨语言调试

#### 调试 python(建议用pdb)





### 远程调试

假如服务器存在一个程序出现 bug，我们不可能在服务器上调试程序，通常的做法是用 `gdbserver` 监听某个端口，然后在本地调试远程程序。



#### 远程

远程服务器上运行命令：

```shell
$gdbserver <ip:port> <program> <argument>

# 例如：用参数 foo.txt 调试 vim
$gdbserver localhost:9527 vim foo.txt
```

`ip` 指代被调试机器的IP地址（其实目前该功能已经被忽略，随便填都行），`port` 可以服务机上任意选择，只要不与其他程序冲突，后跟被调试程序和参数。



#### 本地

开启 `gdb` 后，使用命令

```shell
$gdb
$target remote <ip:port>
```



#### vscode

新建 `.vscode` 文件夹后放入如下配置，关键在于我所注释的那些参数

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Launch",
            "type": "cppdbg",
            "request": "launch",
            "program": "xxxx",	// 被调试程序的名称
            "args": [],	// 参数
            "cwd": "xxx/xxx",	//被调试程序的目录
            "environment": [],
            "miDebuggerPath": "/usr/bin/gdb",	// 调试工具
            "miDebuggerServerAddress": "ip:port",	// IP:端口号
            "useExtendedRemote": true,	// 使用 extended-remote 可以在本地退出后保持连接不关闭
        }
    ]
}
```

然后点击调试即可。使用 `extended-remote` 后服务端无法再 `ctrl+c` 关闭，只能用 `ps -a` 然后 `kill` 进程了。
