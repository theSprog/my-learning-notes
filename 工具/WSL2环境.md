### 动机

写这篇的目的是记录某些难以安装的软件包的过程，方便以后再次回看.

**注意：在安装以下所有软件之前，首先保证你已经卸载了它们**



### WSL2

在 powershell 管理员模式中

```powershell
$wsl --install
```

重启生效



#### 换源

```shell
# 备份
$sudo cp /etc/apt/sources.list /etc/apt/sources.list.bak

# 授予可更改权限
$sudo chmod 777 /etc/apt/sources.list

# 更改
$sudo vim /etc/apt/sources.list 
```



#### ubuntu20 源

VIM命令`dG`删除光标之后所有行

```shell
deb http://mirrors.aliyun.com/ubuntu/ focal main restricted universe multiverse
deb-src http://mirrors.aliyun.com/ubuntu/ focal main restricted universe multiverse
 
deb http://mirrors.aliyun.com/ubuntu/ focal-security main restricted universe multiverse
deb-src http://mirrors.aliyun.com/ubuntu/ focal-security main restricted universe multiverse
 
deb http://mirrors.aliyun.com/ubuntu/ focal-updates main restricted universe multiverse
deb-src http://mirrors.aliyun.com/ubuntu/ focal-updates main restricted universe multiverse
 
deb http://mirrors.aliyun.com/ubuntu/ focal-proposed main restricted universe multiverse
deb-src http://mirrors.aliyun.com/ubuntu/ focal-proposed main restricted universe multiverse
 
deb http://mirrors.aliyun.com/ubuntu/ focal-backports main restricted universe multiverse
deb-src http://mirrors.aliyun.com/ubuntu/ focal-backports main restricted universe multiverse
```

刷新源

```shell
$sudo apt update
$sudo apt-get upgrade
$sudo  apt-get install -f
```



#### 其他必要安装

- ##### 安装 git、build-essential 和 zsh

```shell
$sudo apt install git
$sudo apt install build-essential
$sudo apt install zsh
```



- ##### 安装oh-my-zsh

  ```shell
  $sh -c "$(curl -fsSL https://gitee.com/shmhlsy/oh-my-zsh-install.sh/raw/master/install.sh)"
  ```

更换主题

```shell
$sudo vim ~/.zshrc

# 官网上 THEME 下有主题名称可选，将 ZSH_THEME 改为对应名称即可

# 生效
$source .zshrc
```

更现代的方式：

```shell
# 1. 安装 starship
$ curl -sS https://starship.rs/install.sh | sh

# 2. 由于我们使用的是zsh, 所以在 ~/.zshrc 下写入
eval "$(starship init zsh)"

# 3. 安装自动补全: zsh-autosuggestions
$ git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions

# 4. 在 ~/.zshrc 的 plugins 添加该插件
plugins=( 
    # other plugins...
    zsh-autosuggestions
)
```





- ##### 安装Rust

​		https://www.rust-lang.org/tools/install

- ##### 安装python

  ```shell
  $sudo apt install python-is-python3
  ```

- ##### 安装 cloc

  ```shell
  $sudo apt install cloc
  ```


- ##### 安装 nasm

  ```shell
  $sudo apt install nasm
  ```
  
- ##### 安装 ranger

  ```shell
  $sudo apt install ranger
  ```

- ##### 安装网络必备工具

  ```shell
  $sudo apt install net-tools
  ```

- ##### 安装 tree

  ```shell
  $sudo apt install tree
  ```

- ##### 安装 graphviz 和更高级的 d2

  ```shell
  $sudo apt install graphviz
  $curl -fsSL https://d2lang.com/install.sh | sh -s --
  ```

  





### Valgrind

- 首先进入官网下载页面，获得软件包的下载地址，然后用 `wget` 下载到本地

  ```shell
  $wget https://valgrind.org/downloads/<addr>
  ```

- 解压

  ```shell
  $tar -jxvf valgrind-<version>.tar.bz2 
  ```

- 安装，`--prefix` 建议不要写，默认即可

  ```shell
  $cd valgrind-3.16.0
  $./configure [--prefix=<somewhere>]
  $make install
  ```

- 删除安装包

  ```shell
  $rm <pkg>
  ```



### perf

`wsl2` 不会自带 `perf` 分析工具，手动安装如下

- 安装前置需要

  ```shell
  $ sudo aptitude install build-essential flex bison libssl-dev libelf-dev
  ```

- 下载源码（可以使用 `gitee` 镜像）

  ```shell
  $ sudo git clone https://gitee.com/mirrors/WSL2-Linux-Kernel.git
  ```

- 下载下来是一个文件夹，进入 `perf` 目录编译

  ```shell
  $ cd WSL2-Linux-Kernel/tools/perf
  $ make -j16
  ```

- 将其复制到可执行文件目录

  ```shell
  $ sudo cp perf /usr/local/bin
  ```

- 删除本地包

  ```shell
  $ cd ../../..
  $ rm -rf WSL2-Linux-Kernel
  ```

  



### libevent

http://libevent.org/

在官网上找到最新版下载地址。

```shell
tar -zxvf libevent-xxx-stable.tar.gz

cd libevent-xxx-stable/

./configure

make

sudo make install
```

安装后进入 `sample` 文件夹

```shell
cd sample

# 服务端
./hello-world

#客户端
netcat 127.0.0.1 9995
```

如果客户端收到 `hello world` 字符串，表示 libevent 在本机可以正常使用。





### llvm(clang)

```shell
$sudo apt update
$sudo apt install clang-13 llvm-13-dev
```

所有的相关组件后面都跟着 `-13`，标识版本。想要用 llvm 进行开发，必须安装 `dev`。单纯使用二进制文件则不用



#### 手动安装

由于自动安装的安装版本都不是最新的，所以可以选择手动安装替代

```shell
# 首先是下载源代码，git clone 默认安装最新版.
# 如果需要某一特定版本可以去 https://github.com/llvm/llvm-project/releases?page=1 查看
$ git clone https://github.com/llvm/llvm-project.git

# 将 llvm-project 放在一个合适的文件夹下，开始编译
$ cd llvm-project
# build 文件夹表示编译后的结果
$ mkdir build
$ cd build

# 在 llvm-project/llvm/ 下有一个 CMakeLists.txt, 这就是为什么我们需要指名这一特殊路径
# 如果要使用 make 构建，则指定 -G "Unix Makefiles" 表示生成 makefile 文件
# -DCMAKE_BUILD_TYPE=Release 表示以 release 模式构建
# -DCMAKE_INSTALL_PREFIX=~/11vm-project/build 表示二进制安装路径，我们将其放在源码的 build 之下
# -DBUILD_SHARED_LIBS=on 选择构建动态库
# -DLLVM_ENABLE_PROJECTS=clang 同时编译 clang
# -DLLVM_USE_LINKER=gold 可选
$ cmake ../llvm -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=~/11vm-project/build -DBUILD_SHARED_LIBS=on -DLLVM_ENABLE_PROJECTS=clang -DLLVM_USE_LINKER=gold

# 使用 cmake 构建出构建脚本（此处是makefile）后，使用 make 编译
$ make -j4
```

不过需要注意的是开发包 `llvm-13-dev` 仍然只能自动安装
