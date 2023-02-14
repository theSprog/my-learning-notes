### 前言

相关网站：

- 文档：https://docs.docker.com/
- dockerHub：https://hub.docker.com/
- Dockerfile文档：
  - 原版：https://docs.docker.com/engine/reference/builder/， 
  - 中文版：http://www.dockerinfo.net/document

- 



### 安装(ubuntu下)

注：本安装来自 2021/12/3 ，一切安装原则上应该以官方文档为准

- 卸载旧 docker：

  ```shell
  sudo apt-get remove docker docker-engine docker.io containerd runc
  ```

- 安装前准备

  ```shell
  sudo apt-get update
  
  sudo apt-get install ca-certificates curl gnupg lsb-release
  
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
  
  echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
    $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
  ```

- 安装docker引擎

  ```shell
  sudo apt-get update
  
  sudo apt-get install docker-ce docker-ce-cli containerd.io
  ```

- 将本用户添加进docker组，之后执行docker命令就不必加 `sudo`

  ```shell
  sudo usermod -aG docker <your_name>		# 完成这一步后退出重进一下 shell
  ```

- 启动

  ```shell
  sudo service docker start
  ```

- 查看是否成功

  ```shell
  docker version	
  docker run hello-world 
  # 不报异常即是正常
  
  docker images	# 查看镜像
  ```

- 设置docker在每次启动shell的时候启动

  ```shell
  sudo systemctl enable docker	#一直有效，除非重启宿主机
  ```




### 卸载

```shell
# 删除docker
sudo apt-get purge docker-ce docker-ce-cli containerd.io
 
# 删除配置和容器
sudo rm -rf /var/lib/docker
sudo rm -rf /var/lib/containerd
```



### Docker常用命令

#### 帮助信息

```shell
docker version
docker info

docker 命令 --help	# 查看命令信息
```

命令文档地址：https://docs.docker.com/reference/



#### 镜像命令

- ##### docker images

```shell
$ docker images	# 列出所有镜像

REPOSITORY    TAG       IMAGE ID       CREATED        SIZE
hello-world   latest    feb5d9fea6a5   2 months ago   13.3kB

# 内涵
REPOSITORY	:	镜像的仓库源
TAG			:	镜像的标签
IMAGE ID	:	镜像ID
CREATED		:	镜像的创建时间
SIZE		:	镜像大小

#可选项
-a	列出所有
-q	只列出ID
```



- ##### docker search

```shell
$ docker search <REPOSITORY>

#可选项
--filter=STARS=<num>	# 只列出 STARS >= num 的 images
```



- ##### docker pull

```shell
$ docker pull <REPOSITORY>	# 下载镜像，默认拉取最新版

$ docker pull <REPOSITORY>:tag	# 拉取指定版本的镜像，tag即版本号, 写全了也叫 image
# e.g. docker pull mysql:5.7

$ docker pull <owner>/<REPOSITORY>:tag	# 这种方式拉取的并非是官方的镜像，而是第三方用户的
```



- ##### docker rmi

```shell
# 指定id比较方便，若指定名称，默认删除的是 tag 为 latest 的images
$ docker rmi [<IMAGE ID>|<image>, ...]	# 删除指定id 或 指定名称的镜像, 可指定多项

$ docker rmi -f $(docker images -aq) # 删除所有镜像
```





#### 容器命令

只有现有镜像才能再创建容器

- ##### docker run

```shell
$ docker run <image>

# 可选项
--name=<NAME>		# 如果有一个镜像创建多个容器， 每个容器有不同的命名
-d					# 后台方式运行
-it					# 交互方式运行，从容器中退回用 exit
-[p|P]				# 指定端口/随即指定端口
	-p ip:主机端口:容器端口	
	-p 主机端口:容器端口	(常用， 例如 8080:8080)
--memory 			# 限制内存大小，例如 --memory=200M
--cpu-shares		# 设置CPU权重，假设第一个容器设置为10，第二个为5，则第一个占用CPU的时间会是第二个的2倍
--network			# 设置本容器要连接的网络，不指定的话默认连接 docker0
-e					# 设置容器内环境变量
```

假设有镜像 centos，用此镜像创建容器

```shell
$ docker run -it centos /bin/bash	# /bin/bash 即交互方式
$ exit # 从容器中退回，如果不是后台方式运行的话，容器终止
$ ctrl + p + q	# 从容器中退回，但容器不终止
```



- ##### docker ps

```shell
$ docker ps # 查看正在运行的容器

# 可选项
-a			# 查看运行过的容器，以从新到旧的顺序排列(包括正在运行的)
-n			# 只显示前 n 个值
```



- ##### docker rm	

```shell
$ docker rm <容器ID>	# 删除容器，但不能删除正在运行的容器

#可选项
-f		# 可以删除正在运行的容器

$ docker rm -f $(docker ps -aq)	# 删除所有容器（包括正在运行的）
$ docker rm $(docker ps -f "status=exited" -q) # 删除已经退出的容器
```



- ##### 启动与停止容器

```shell
$ docker start <容器ID>	# start 是启动一个暂停的，run 是创建一个全新的（从镜像）
$ docker restart <容器ID>	# restart 将某个暂停的容器重新启动，即从头开始运行
$ docker stop <容器ID>
$ docker kill <容器ID>	# 加强版的 stop
```



- ##### 对容器执行某个命令

```shell
$ docker exec <command>

# 打印容器IP地址
$ docker exec ip a
```



- ##### 进入正在运行的程序

```shell
$ docker ps		# 查看正在运行的容器

# 方式1
# 进入容器后打开一个新的终端，i:interactive 交互式的，t:tty 终端
$ docker exec -it 容器ID <shell_name>	# shell_name 指定交互方式，例如 /bin/bash

#方式2
# 进入容器正在执行的终端
$ docker attach 容器ID	
```



- ##### 从容器中将数据拷到宿主机上

```shell
$ docker cp 容器ID:路径 宿主路径
```



- ##### 查看容器运行资源

```shell
$ docker stats
```



- ##### 将容器保存起来，方便下次直接使用

```shell
$ docker commit <容器ID> <名称>:<TAG>	# 创建一个 <名称>:<TAG> 的 image
# 一般不会通过这种方式来创建 image, 而是通过 DockerFile
```



- ##### 查看docker底层信息

```shell
$ docker inspect <容器ID>
```



- ##### 登录dockerHub，并且发布自己制作的镜像

```shell
$ docker login

# push 镜像到 dockerHub
# 自己发布的镜像一定要以自己的用户名开头
$ docker push <用户名/镜像名称>[:TAG]
```







### 数据持久化

每一个container都包含 container layer 和 image layers（ReadOnly），所以容器的数据都保存在 container layers 层，但是这种保存方式使得只要容器一被删除，容器中保存的数据也被删除。所以需要专门的持久化方式，使得容器和数据分离



#### 容器数据卷（Volume）

作用：使得容器和数据分离，容器将所有的数据直接挂载到宿主机上，即宿主机目录和容器目录双向绑定，指向同一块空间



- ##### 使用数据卷，指定路径挂载（bind mouting）

```shell
# 允许挂载多个目录，即多次使用 -v
$ docker run -v <宿主机目录>:<容器内目录> 镜像ID
```



- ##### 具名挂载和匿名挂载

```shell
# 匿名挂载不指定宿主机目录
$ docker run -v <容器内目录> 镜像ID

# 具名挂载
$ docker run -v <卷名>:<容器内目录> 镜像ID	# 卷名是给数据卷起个名字，由 docker 统一管理。允许多个容器共同映射到同一个卷
```

拓展：在容器内目录的后面还可以在拼接 `ro`，`rw` 来指定容器权限

```shell
# 容器只能读卷内数据，不能写
$ docker run -v <卷名>:<容器内目录>:ro 镜像ID

# 容器能读写卷内数据
$ docker run -v <卷名>:<容器内目录>:ro 镜像ID
```



- ##### 查看所有 volume 情况

```shell
$ docker volume ls
```

- ##### 查看 volume 具体信息（如挂载的具体路径）

```shell
$ docker volume inspect <卷名>
```

所有的docker容器内的卷，没有指定目录的情况下（即具名和匿名挂载）都在 `/var/lib/docker/volumes/` 下，卷名的 `_data` 目录下。一般情况下都是使用具名挂载



#### 基于plugin的数据卷

例如 NAS ，aws。

//todo 待补充





### DockerFile

- ##### DockerFile语法

```dockerfile
# 开头都是 FROM，即在某个父镜像上继续加层
# '#' 开头的行是注释
FROM <image>:<tag>	# 不写 tag 默认 latest， 也可以写 FROM scratch，即直接基于宿主操作系统

# 接下来的标签出现顺序不固定


LABEL <key>=<value>		# 定义元数据，如作者，日期等 # 可定义多个 LABEL

# 每次 RUN 都会生成新的一层，为了避免无用分层，合并多条命令成一行
# && 用于连接多条命令， \ 用于换行
# 例如 yum update && install -y vim \
#						python-dev
RUN <command>	


# 相当于 cd 命令， 没有目录会创建目录并进入
# 不要使用 RUN cd，尽量使用绝对目录
WORKDIR <dir>

# 将本地文件复制进容器内
# 添加远程文件使用 curl 和 wget
ADD <本地文件> <容器内目录>	# 若本地文件是压缩包，则 ADD 命令会自动解压
COPY <本地文件> <容器内目录> # COPY 命令不会自动解压

# 相当于定义常量
# 例如 ENV MYSQL_VERSION 5.7
ENV <key> <value>

VOLUME 

# 对外暴露端口号
EXPOSE <port_num>

# 设置容器执行后的默认执行的命令和参数
# 如果定义了多个 CMD，只有最后一个会被执行
# 如果 docker run 指定了其他命令，则 CMD 命令会被忽略，CMD 相当于就是默认命令
# 例如：CMD python app.py
CMD <command>

# 设置容器启动时的命令
# 让容器以服务的形式运行，并且不会被忽略，一定会被执行
# 最佳实践：写一个脚本作为 entrypoint
# 例如：ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
ENTRYPOINT <command>

# 常见写法：ENTRYPOINT + CMD，运行ENTRYPOINT命令，并且用CMD接受参数
# 例如：
# 
# ENTRYPOINT ["/usr/bin/stress"]
# CMD []

```



- ##### 构建镜像命令

```shell
$ docker build -t <镜像名称> <Dockerfile所在目录>
```

在构建镜像时，先会去基于 `<os_name>:<tag>`  生成一个临时容器，再在这个容器里面执行各种命令，最后将此容器打包镜像，将临时容器删除



- ##### 构建时debug

```shell
# 每构建一层就会生成一个镜像，我们可以从失败处 run -it 镜像，从而发现为什么接下来的操作失败
```



- 设定编码字符集

  docker 自带的字符编码是 POSIX，在容器内部查看： `locale` 命令

  如果想环境自带 `UTF-8` 支持的话需要手动在 dockerfile 中加上

```dockerfile
ENV LANG en_ZW.utf8
```





### Docker网络

netns(network namespace)是在linux中提供网络虚拟化的一个项目，使用netns网络空间虚拟化可以在本地**虚拟化出多个网络环境**，使用netns创建的网络空间独立于当前系统的网络空间，其中的网络设备以及iptables规则等都是独立的，就好像进入了另外一个网络一样

Linux网络空间：

```shell
# 列出所有网络空间
$ ip netns list

# 添加网络空间
$ ip netns add <namespace>

# 删除网络空间
$ ip netns delete <namespace>

# 进入网络执行命令
$ ip netns exec <name> <command>

# 添加一对veth网卡，使得两个空间可以互通，veth设备是成对出现的
$ ip link add name <veth_name1> type veth peer name <veth_name2>

# 查看veth网卡，也可以进入某个网络空间查看网卡信息
$ ip link
$ ip netns exec <namespace> ip link

# 将veth网卡添加进各自的网络空间
$ ip link set <veth_name> netns <namespace>

# 为网卡分配ip地址
# <ip_addr> 例如 192.168.1.1/24
$ ip netns exec <namespace> ip addr add <ip_addr> dev <veth_name>

# 进入网络空间，将veth网卡启动
$ ip netns exec <namespace> ip link set dev <veth_name> up

# 查看网络空间对应veth网卡的ip地址，验证是否启动成功
$ ip netns exec <namespace> ip a
```



在创建一个容器时也会创建一个独立的网络空间，每一个容器都会连接到 `docker01` 网络空间上，从而达到容器间互联

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112061534426.png" style="zoom: 50%;" />

并且由 `docker0` 通过NAT技术与物理网卡相连接，从而访问外部 Internet

查看网络：

```shell
$ docker network ls

NETWORK ID     NAME      DRIVER    SCOPE
bc6e79660eef   bridge    bridge    local	# docker0
22aa52c218e5   host      host      local	# 直接使用宿主网络
39bd74abe5ba   none      null      local	# 无网络，只能通过 exec -it 方式访问，其他容器无法访问
```

创建一个网络：

```shell
$ docker network create <NETWORK>

# 可选项
-d 设置连接方式，默认 bridge
```



- ##### 连接

通过在运行容器时指定 `--link` 参数，使得本容器与另外一个容器连接，从而本容器访问被连接的容器只需要直接使用 `name` 即可，然而反过来却不行，即 `--link` 是单向的

如果有两个容器连接到用户自己创建的网络上，则默认这两个容器是互联的，即可以互相通过网络连接



- ##### 多机器通信

//todo

