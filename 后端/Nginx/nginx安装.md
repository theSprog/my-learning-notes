### 网站

https://nginx.org/en/download.html

存在预览版和稳定版，一般采用最新稳定版

```shell
$wget <url>
$tar -xvzf <package>
$cd <package>
```



### 前置条件

nginx使用了 epoll 技术，linux版本必须在 2.6 及以后

```shell
$uname -a
```



手动编译nginx必须需要以下三个函数库（不是二进制版本，而是dev版本）

- libpcre3-dev：pcre，用于解析正则表达式
- libz-dev：zip，用于压缩解压缩
- libssl-dev：openssl，用于网站加密通信，默认 nginx 不使用该库，但若要手动指定使用，则需要该库



### Nginx源码结构

解压 nginx 后会生成一定的源码目录，他们的作用分别是

- auto：编译时相关脚本，在可执行文件 configure 执行时要用到
  - cc：检查编译器
  - lib：检查库依赖
  - os：检查操作系统
  - type：检查平台
- conf：默认配置文件，配置 nginx 时需要用到
- contrib：一些脚本和工具，例如 vim 高亮工具
- html：错误页（50x.html）和欢迎页（index.html）
- man：帮助目录，可用 man 命令查看
- src：源码



### 编译和安装

查看配置选项，

```shell
$./configure --help

输出如下:
  --prefix=PATH                      set installation prefix
  --sbin-path=PATH                   set nginx binary pathname
  --modules-path=PATH                set modules path
  --conf-path=PATH                   set nginx.conf pathname
  --error-log-path=PATH              set error log pathname
  --pid-path=PATH                    set nginx.pid pathname
  --lock-path=PATH                   set nginx.lock pathname

  --user=USER                        set non-privileged user for
                                     worker processes
  --group=GROUP                      set non-privileged group for
                                     worker processes

  --build=NAME                       set build name
  --builddir=DIR                     set build directory

  --with-select_module               enable select module
  --without-select_module            disable select module
  ...
```

- --prefix： 即配置安装的根目录，默认是 `/usr/local/nginx`

- --sbin-path： 二进制可执行文件的目录，以 --prefix 为前缀

-  --conf-path： nginx.conf 的目录
- --with：指定需要的模块，不指定则默认不需要
- --without：指定不需要的模块，不指定则默认需要



```shell
$./configure
```

执行完后会生成 `Makefile` 文件和 `objs` 目录，该目录用于存放中间文件，该目录下的 `ngx_modules.c` 用于指定那个等会编译时那些模块会被编译进二进制文件中



#### 编译

```shell
$make
```

完成后在 `objs/src` 目录下会产生大量 `.o` 文件，是编译产生的中间文件，同时还会有一个 nginx 可执行文件，它就是编译的结果



#### 安装

```shell
$sudo make install
```

之所以要 `sudo` ，是因为它会创造 --prefix 目录并且在  --prefix 目录下创造其他下级目录，这一步可能需要权限

`install` 的本质就是把本目录下的各种必须的文件拷贝对应的文件中，例如 `objs/nginx` 拷贝进 `--prefix/sbin/` 目录下

连续多次安装会将原来`--prefix/sbin/` 目录下的 `nginx` 文件备份为 `nginx.old`,再将现在的 `nginx` 文件拷贝进此目录



### 启动与停止

```shell
$sudo nginx
$ps -ef | grep nginx

nginx: master process ./nginx
nginx: worker process

$sudo nginx -s stop
```

**进入 `nginx` 二进制的目录**后，用sudo模式启动，这是因为该进程需要打开 `error.log` 文件，而这一步可能没有权限

启动后 `nginx` 便会启动两个进程：一个 `master`，一个 `worker`

启动浏览器，向对应 `ip:port` 请求，应该就能看见 nginx 欢迎页面

![image-20220503224743396](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202205032247472.png)



### 建立软连接

```shell
$sudo ln -s /usr/local/nginx/sbin/nginx /usr/local/sbin/nginx
```

建立软连接之前不存在 `/usr/local/sbin/nginx`，建立后该文件存在，且指向原二进制文件 `nginx`