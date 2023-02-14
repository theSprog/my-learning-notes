---
title: maven学习
date: 2021-10-16 19:22:51
tags:java, maven
---

Java项目构建工具maven的学习

<!-- more -->

<!-- toc -->



## maven基础

### maven本质与作用

maven是一个Java项目管理工具，将项目的开发的管理抽象为一个项目对象模型（project object model）,即 POM，一个项目就是一个对象。

maven具体的用处有

- 项目构建：提供标准的项目，跨平台的项目构建方式

- 依赖管理：管理项目依赖的资源，避免版本间冲突

- 统一开发结构：提供标准的项目结构

  <img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031636135.png" style="zoom:67%;" />

maven的下载地址为[maven下载页](http://maven.apache.org/download.cgi), 使用maven需要配置`JAVA_HOME`和`MAVEN_HOME`，测试安装是否成功用`mvn -v` 



### maven基本概念：仓库与坐标

#### 仓库

- 仓库定义：是用于存储jar包的地方
- 仓库分类：
  - 本地仓库：自己电脑上存储资源的仓库
  - 私服：团队或公司存储资源的仓库
  - 中央仓库：maven团队维护，存储互联网上所有jar包资源

#### 坐标

- 坐标定义：用于定位某个资源在仓库中的位置
- 坐标构成
  - groupId：定义jar包隶属于的组织（通常是域名反写：例如 org.apache)
  - artifactId：定义jar包的项目名称（通常是模块名称：例如CRM）
  - version：定义项目的版本号
  - packaging(可选)：项目打包方式
- jar包查询网站：[https://mvnrepository.com/](https://mvnrepository.com/)

#### 仓库配置

##### 本地仓库配置

maven默认本地仓库在`${user.home}\.m2\repository`下，但我们可以自定义本地仓库的位置

1. 进入maven安装目录，打开`conf\settings.xml`配置文件
2. 寻找  <localRepository>/path/to/local/repo</localRepository>，将尖括号中的内容替换为你想设置的本地仓库地址

##### 镜像仓库配置

maven是从中央仓库去取本地不存在的jar包，而默认的中央仓库在国外，我们可以配置镜像中央仓库，当访问国外的仓库时，会默认访问镜像仓库(用[阿里maven](https://developer.aliyun.com/mvn/guide)为例)

```xml
<!-- conf\settings.xml -->
<!-- 在<mirrors></mirrors>标签中添加 mirror 子节 -->

<mirror>
    <!-- id是镜像的唯一标识, 必须唯一 -->
    <id>aliyunmaven</id>
    <!-- 对哪种仓库进行镜像,配置为central(中央),表示要访问central时去访问下面配置的url -->
    <mirrorOf>central</mirrorOf>
    <!-- name无所谓 -->
    <name>aliyun</name>
    <url>https://maven.aliyun.com/repository/public</url>
</mirror>
```

##### 下载依赖源码和文档

```xml
<!-- conf\settings.xml -->

<!-- profiles -->
<profiles>
<profile>
    <id>downloadSources</id>
    <properties>
        <downloadSources>true</downloadSources>
        <downloadJavadocs>true</downloadJavadocs>           
    </properties>
</profile>
</profiles>

<!-- activeProfiles -->
<activeProfiles>
   <!-- downloadSources 对应 profile id 标签 --> 
  <activeProfile>downloadSources</activeProfile>
</activeProfiles>
```



### maven工程结构

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031636137.png" style="zoom:67%;" />

在src同层目录下应该有一个pom.xml来为此工程建模

java目录下是正规的代码源程序，包的根目录在这个文件夹下，包从这里开始查找



在java工程的目录中( 即java-project这个文件夹下 )执行maven命令

```shell
$mvn compile	# 会自动去下载该工程依赖的包, 下载到之前 maven 配置的仓库里，同时生成一个 target 文件夹在src同级目录上

$mvn clean		# 清理 target 文件夹

$mvn test		# 启动测试，并打印结果。生成的报告在 target/surefile-reports 下

$mvn package	# 将源程序打包，将打包的结果放在 target 目录下

$mvn install	# 将源程序安装到 maven 本地仓库中
```



### idea配置maven

进入idea的设置中，搜索maven，将 `maven home directory` 和 `User settings file` 改成 maven 文件夹所在位置和setting.xml所在位置(一般是 `%maven_home%/conf/settings.xml` 这个位置)，有需要的话，也可以将 `Local repository` 改成自己需要的位置

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031637498.png)



### maven依赖

在项目的 `pom.xml` 中添加依赖

```xml
<dependencies>
    <dependency>
        <groupId>此处填依赖隶属的组织</groupId>
        <artifactId>此处填依赖的包名</artifactId>
        <version>版本号</version>
    </dependency>
    
    <dependency>
        ...
    </dependency>
    
    ...
</dependencies>
```

#### 依赖传递

依赖可以传递，所以分为两种依赖：

- 直接依赖：在pom.xml中直接写明的自己要依赖的包
- 间接依赖：本项目所依赖的包所依赖的包也是本项目的依赖，即可以被本项目所使用

​	

#### 依赖冲突

当依赖发生冲突时

- 路径优先：依赖路径越短的包覆盖长路径的包

- 声明优先：当资源在相同层级时，配置顺序靠后的覆盖靠前的

- 可选依赖：指对外隐藏当前项目的依赖

  ```xml
  <optional>true</optional>	开启隐藏
  ```

- 排除依赖：指主动地断开依赖关系

  ```xml
  在dependency标签内加上:
  
  <exclusions>
      <exclusion>
          <groupId>...</groupId>
          <artifactId>...</artifactId>
          不需要写版本
      </exclusion>
  </exclusions>
  ```



#### 依赖范围

在scope标签下有五种：compile、provided、runtime、system、test。他们的作业范围有三种：主程序有效，测试程序有效，打包有效

<img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/img/Snipaste_2021-10-22_13-06-13.png" style="zoom:67%;" />

system跟provided类似，但是区别是你必须在该`scope`下面再写一个`<systemPath></systemPath>` 显式地指明jar包的本地路径，同时打包不会打进包里（不推荐使用）



### 插件

插件可以增强maven的功能，插件可以在项目生命周期的某个阶段干某些事，具体写法是在**pom.xml**内加入如下配置。

```xml
<build>
    <plugins>
        <plugin>
            <groupId>...</groupId>
            <artifactId>...</artifactId>
            <version>...</version>
            <executions>
                <execution>
                    <goals>
                        <goal>...</goal>	<!--打包成的目标-->
                        <goal>...</goal>
                    </goals>
                    <phase>...</phase>	<!--在哪个阶段打包-->
                </execution>
            </executions>
        </plugin>
    </plugins>
</build>
```





## maven高级

### 分模块开发与设计原则

- 职责清晰：模块中包含且仅包含当前模块所需要的类和配置信息
- 依赖完善：当前模块所依赖额模块需要通过maven依赖的形式来引入，而被依赖的模块又必须先 mvn install 的方法打包到本地仓库，供其他模块使用
- 统一汇总：在 web.xml 中需要加载所有的必须配置文件



### 聚合

聚合即通过一个模块来管理其他模块的构建，即同时执行生命周期命令

我们需要定义一个模块，它本身是一个模块，而它的作用就是用于管理其他模块。只需要在被用于管理的模块的 pom.xml 文件中如下配置

```xml
<!-- 定义该模块用于管理，如果没有定义 packaging，默认打 jar 包 -->
<packaging>pom</packaging>

<!-- 管理的工程列表 -->
<modules>
    <!-- 具体的工程名称 -->
    <module></module>
    <module></module>
    <module></module>
    <module></module>
</modules>
```



### 继承

在管理模块中直接声明所有模块要用的依赖包(和插件包)，所有子模块的依赖直接去父模块取，保持了不同模块的所用依赖版本的统一，利于管理。要让父模块管理所有依赖，直接在该模块内部加上 `dependencyManagement` 标签即可

```xml
<!-- 声明本模块用于管理所有依赖 -->
<dependencyManagement>
    <dependencies>
        ...
    </dependencies>
</dependencyManagement>

<pluginManagement>
    <plugins>
        ...
    </plugins>
</pluginManagement>
```

而在所有子模块中需要加上这个标签

```xml
<!-- 定义本模块的父模块 -->
<parent>
    <groupId>...</groupId>
    <artifactId>...</artifactId>
    <version>...</version>
    <!-- 填写父模块的 pom.xml 文件位置 -->
    <relativePath>...</relativePath>
</parent>
```

这样在子模块中只需要写自己依赖的模块名称，而不用再写版本号，直接使用父模块统一的版本。不是说父模块写好了依赖子模块就不用配置了，是父模块将所有依赖写好，子模块按需调用

```xml
<groupId>...</groupId>
<artifactId>...</artifactId>
```



### 属性与资源配置

相当于 pom 文件内的变量, 将某些公共属性抽取出来，使用时直接使用，修改时也方便。

属性分为五种:

- 自定义属性：类似于定于变量。自定义的变量在其他 文件中也能使用，调用格式也是 `${}`，前提是配置了资源文件对应信息

  ```xml
  <properties>
      <...>...</...>
      <!-- 例如，定义spring版本 -->
      <spring.version>...</spring.version>
      <jdbc.username>abc</jdbc.username>
      <jdbc.password>123456</jdbc.password>
  </properties>
  
  <!-- 使用时用 ${} 格式 -->
  <version>${spring.version}</version>
  
  <!-- 在pom.xml中配置资源文件对应信息，一般配在父模块的pom.xml中 -->
  <resources>
      <resource>
          <!-- directory是从当前 pom.xml 的级别开始查找的 -->
          <directory>...</directory>
          <!-- 若设为false则对应文件不会解析 ${} -->
          <filtering>true</filtering>
      </resource>
  </resources>
  <!-- 在pom.xml中配置测试文件对应信息，一般也配在父模块的pom.xml中 -->
  <testResources>
      <testResource>
          <!-- directory是从当前 pom.xml 的级别开始查找的 -->
          <directory>...</directory>
          <filtering>true</filtering>
      </testResource>
  </testResources>
  
  
  <!-- 其他文件也能使用，例如数据库配置 jdbc.properties -->
  jdbc.username = ${jdbc.username}
  jdbc.password = ${jdbc.password}
  
  ```

- Setting属性：使用 Maven 配置文件 settings.xml 中的标签属性，用于动态配置

  ```xml
  ${settings.localRepository}
  ```

- 内置属性：属于Maven内置属性，可以快速调用

  ```xml
  ${basedir}
  ${version}
  ```

- Java系统属性

- 环境变量属性

  ```xml
  <!-- 系统属性调用方式示例 -->
  ${user.home}
  
  <!-- 环境变量调用方式示例 -->
  ${env.JAVA_HOME}
  
  <!-- 属性查询方式 -->
  $mvn help:system
  <!-- 显示结果有两个分区：系统属性和环境变量 -->
  ```

  

### 版本管理

版本分为两种：SNAPSHOT(快照版本，也叫未完成版本)，RELEASE(发布版本)。当然这只是惯例，而非要求。不同的企业用的命名方式不一样

#### 版本号约定：

约定规范：<主版本>.<次版本>.<增量版本>.<里程碑版本>

- 主版本：表示项目架构有重大变化，例如 spring4 到 spring5
- 次版本：表示有重大版本添加，或者系统地修复漏洞
- 增量版本：表示有紧急漏洞的修复
- 里程碑版本：即一个RELEASE版本的发布

示例：spring: 5.1.9.RELEASE



### 多环境兼容

在本地开发时一种环境，在项目测试时可能又是另一种环境，具体部署到线上时又是另一套环境。如果每次环境迁移时都去改参数，会很麻烦。maven支持多环境的配置

```xml
<!-- 创建多环境 -->
<profiles>
    <!-- 定义具体的生产环境 -->
    <profile>
        <!-- 每个环境都有唯一的id, 自己命名 -->
        <id>...</id>	<!-- 例如 <id>pro_env</id> -->
        <properties>
            ...
        </properties>
        
        <!-- 设置默认的 install 环境 -->
        <activation>
            <activeByDefault>true</activeByDefault>
        </activation>
    </profile>
    
    <!-- 定义具体的开发环境 -->
    <profile>
        <id>...</id>	<!-- 例如 <id>dev_env</id> -->
        <properties>
            ...
        </properties>
    </profile>
</profiles>
```

具体打包时使用的环境需要参数 -P <id\>, 如果不设置，会去取默认的环境

```shell
$ mvn install -P id	

#例如
$ mvn install -P pro_env
```



### 跳过测试

只需要在测试命令之后的那些命令加入 `-D skipTests` 即可跳过所有测试

```shell
# 例如
$ mvn install -D skipTests
```

也可以通过插件的形式在 pom.xml 中配置跳过测试

```xml
<plugins>
    <plugin>
        <!-- 使用 maven-surefire-plugin 插件 -->
        <artifactId>maven-surefire-plugin</artifactId>
        <version>...</version>
        <configuration>
            <!-- 跳过所有测试 -->
            <skipTests>true</skipTests>
            
            <!-- 一般不会用上面这种，只会跳过或测试某几个测试用例 -->
            <includes>
                <include>...</include>
            </includes>
            
            <excludes>
                <exclude>...</exclude>
            </excludes>
        </configuration>
    </plugin>
</plugins>
```

也可以通过idea自带的maven组件的界面操作来跳过测试

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031637585.png)



### 私服

#### nexus服务器安装与启动

使用nexus搭建maven私服，[nexus下载页面](https://help.sonatype.com/repomanager3/download)可以下载不同操作系统所对应的产品。下载解压后会有两个目录

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031637370.png)

其中上面一个目录是nexus的服务器，下面一个目录是工作空间，nexus运行起来后所管理的依赖全都放在这个目录下，运行服务器：`nexus /run <服务器名称>`

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031638250.png)

服务器启动后会看到如下界面

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031638369.png)

访问服务器端口**8081**即可

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031638689.png)

想要修改默认端口号，进入 etc/nexus-default.properties 文件，修改 `application-port` 属性即可。

修改服务器相关配置，进入 bin/nexus.vmoptions 修改即可

管理员登录：管理员用户名为 **admin**, 密码放在所说明的文件下。登陆后需要自己设置一个密码

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031638156.png)



#### 私服仓库的分类

- 宿主仓库：给团队自己维护的仓库，保存无法从中央仓库获取的资源，如
  - 自主研发
  - 第三方非开源的库，例如 oracle 的驱动
- 代理仓库：代理远程仓库，通过nexus访问其他远程仓库，例如中央仓库
- 仓库组
  - 将若干个仓库(例如宿主仓库或代理仓库)组成一个组，简化配置
  - 仓库组里面没有资源，只是一种设计



#### 上传组件

##### 手动上传组件

点击 **齿轮图标** -> Repositories -> Create repository -> maven2(hosted) -> 填写仓库唯一Id，点击确定

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031638129.png)

将刚刚创建的仓库加入到群组中：点击群组 -> 将所要操作的组加入**Members**中

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031638040.png)

上传资源到刚才的仓库中：点击方块图标 -> Browse -> 仓库名 -> Upload Component -> 填写必要信息 -> Upload 即可

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031639173.png)



#### 本地访问私服

需要在本地仓库中的 `settings.xml` 中配置 servers 标签

```xml
<servers>
    <server>
      <!-- id任意定义,但最好和私服的仓库名称一致，比如之前的test_repo -->
      <id>deploymentRepo</id>		
      <username>repouser</username>
      <password>repopwd</password>
    </server>
</servers>
```

之后配置 mirror 镜像，其中 url 可直接从 Browse 页面 copy

```xml
<mirrors>
    <mirror>
        <!-- id 必须唯一 -->c
        <id>...</id>
        <!-- 对那种仓库进行镜像，此处配置为全部 -->
        <mirrorOf>*</mirrorOf>
        <!-- url 可以在 Browse 每一个项后，有一个copy按钮, 复制即可 -->
        <url>...</url>
    </mirror>
</mirrors>
```

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031639861.png)



#### 本地发布到私服

在项目的 pom.xml 文件中配置发布的私服信息

```xml
<distributionManagement>
    <!-- release版本发布的仓库 -->
    <repository>
        <!-- 此处填写发布的服务器仓库的id, 不是之前镜像的id -->
        <id>repo_test</id>
        <!-- url 可以在 Browse 每一个项后，有一个copy按钮, 复制即可 -->
        <url>...</url>
    </repository>
    
    <!-- snapshots版本发布的东西 -->
    <snapshotRepository>
        <!-- 此处填写发布的服务器仓库的id, 不是之前镜像的id -->
        <id>...</id>
        <!-- url 可以在 Browse 每一个项后，有一个copy按钮, 复制即可 -->
        <url>...</url>
    </snapshotRepository>
</distributionManagement>
```

发布命令：

```shell
$ mvn deploy
```



发布过程：

1. 当要发布对应版本的资源时，去访问对应的 url。
2. 由于没有用户名和密码，又回到 settings.xml, 根据 id 匹配对应的 server。
3. 拿到用户名和密码后再次登录，发布版本
