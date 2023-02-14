---
title: shell学习
date: 2021-10-16 19:22:05
tags: linux
---

linux环境下shell脚本的学习

<!-- more -->

<!-- toc -->



### shell脚本的执行

- 通过chmod来运行，前提是cd到相应目录。这种方法需要在脚本第一行加上 `#! 解释器路径`， 例如 `#! /bin/sh`

  ```bash
  $ chmod +x ./shellName.sh		# 授予执行权限
  $ ./shellName.sh				# 执行脚本
  ```

- 解释器运行, 这种方式不需要在头部加上解释器路径

  ```bash
  $ /bin/sh shellName.sh
  $ /bin/bash shellName.sh # bash是sh的增强版本
  ```

### 用户输入

通过read读取用户输入，存储到某一个变量中

```shell
$ read s	
hello
$ echo $s	#echo默认会换行
hello
```

### 变量

#### 定义变量

基本变量只有两种: 字符串和数字(数组是它两的复合)

变量定义时不需要加`$`, 使用时需要加`$`。且定义式等号两边不能有空格

```shell
#! /usr/bin/bash
name="bob"
echo $name
name="ada"
echo $name
```

要将变量定义为只读的，只需在定义完后加上一句 readonly

```shell
#! /usr/bin/bash
name="bob"
readonly name
echo $name
name="ada"	# this will be error
```

变量使用时可以加`{}`, 也可以不加，但推荐复杂使用加上，用来告知解释器变量的边界



### 字符串处理

shell的串下标是从 `1` 开始的

#### 字符串的删除

删除规则:

| 语法                | 语义                                 |
| ------------------- | ------------------------------------ |
| ${变量名#匹配规则}  | 从变量**头部**开始匹配，最**短**删除 |
| ${变量名##匹配规则} | 从变量**头部**开始匹配，最**长**删除 |
| ${变量名%匹配规则}  | 从变量**尾部**开始匹配，最**短**删除 |
| ${变量名%%匹配规则} | 从变量**尾部**开始匹配，最**长**删除 |

```shell
#! /usr/bin/bash

variable="I love you, Do you love me, too?"

var=${variable#*ov}
echo $var	# --> e you, Do you love me, too?

var=${variable##*ov}
echo $var	# --> e me, too?

var=${variable%ov*}
echo $var	# --> I love you, Do you l

var=${variable%%ov*}
echo $var	# --> I l
```



#### 字符串的替换

替换规则

| 语法                         | 语义                                       |
| ---------------------------- | ------------------------------------------ |
| ${变量名/旧字符串/新字符串}  | 用新字符串替换变量名内旧字符串，只替换首个 |
| ${变量名//旧字符串/新字符串} | 用新字符串替换变量名内旧字符串，替换全部   |

```shell
#! /usr/bin/bash

variable="I love you, Do you love me, too?"

var=${variable/love/hate}
echo $var	# --> I hate you, Do you love me, too?

var=${variable//love/hate}
echo $var	# --> I hate you, Do you hate me, too?
```



#### 字符串的长度

- 通过 `${#string}`
- 通过 expr length "${string}", 也可以通过反引号(`)将结果引起来

```shell
#! /usr/bin/bash

variable="I love you, Do you love me, too?"

var=${#variable}
echo $var					#--> 32
expr length "${variable}"	#--> 32
```



#### 字串处理

| 要求                                   | 途径                                                         |
| -------------------------------------- | ------------------------------------------------------------ |
| 去掉头部n个字符                        | ${string:n}                                                  |
| 抽取从所给串某一位置开始一定长度的子串 | ${string:position:length}                                    |
| 抽取尾部若干个串                       | ${string:(-position)} or \${string: -position}(冒号后有空格) |

```shell
#! /usr/bin/bash

variable="helloworld"

var=${variable:5}
echo $var		# --> world

var=${variable:5:2}
echo $var		# --> wo

var=${variable:(-4):2}
echo $var		# --> or

var=${variable:(-3)}
echo $var		# --> rld
```



### 流程控制

#### if

- if-then-fi

  ```shell
  if [ 10 == 10 ]		#条件式里的空格是必须的，否则会报错
  then
     	echo "10 等于 10"
  fi
  
  if (( 10 == 10 ))		#条件式也可用这样的形式，内层括号是必不可少的
  then
     	echo "10 等于 10"
  fi
  ```

  

- if-then-else-fi

  ```shell
  if [ 10 == 20 ]
  then
     	echo "10 等于 20"
  else
  	echo "10 不等于 20"
  fi
  ```

- if-then-elif-then-[elif-then]-else-fi

  ```shell
  if [ 10 == 20 ]
  then
     	echo "10 等于 20"
  elif [ 10 == 10 ]
  then
  	echo "10 等于 10"
  else
  	echo "懵了"
  fi
  ```



#### for

- for-in-do-done

  ```shell
  for loop in 1 2 3 4 5
  do
      echo "The value is: $loop"
  done
  ```



#### while

- while-do-done

  ```shell
  int=1
  while(( $int<=5 ))	#必须将表达式放入()内部, 否则会报错
  do
      echo $int
      let "int+=2"	#let 命令用于执行其后的表达式
      				#int=`expr $int + 2`, 注意表达式操作数之间必须要分开
  done
  ```

- 死循环

  ```shell
  while true
  do
      command
  done
  
  #或者是这样
  while :		#此处有一个冒号
  do
      command
  done
  ```

#### until

- until-do-done

  ```shell
  a=0
  
  until (( $a > 10 ))	#until [ $a == 10 ]。 同样，中括号与表达式之间必需要有距离
  do
     echo $a
     a=`expr $a + 1`	#通过反引号来存储 expr 得出的值
  done
  ```

  

#### case

case-in-esac用于多选择分支，每个分支用变量加右括号`)`表示匹配规则，每个匹配块内用 `;;` 表示结束，esac(即case反过来)表示结束

```shell
echo '输入 1 到 4 之间的数字:'
echo '你输入的数字为:'
read aNum
case $aNum in
    1)  echo '你选择了 1'
    ;;
    2)  echo '你选择了 2'
    ;;
    3)  echo '你选择了 3'
    ;;
    4)  echo '你选择了 4'
    ;;
    *)  echo '你没有输入 1 到 4 之间的数字'
    ;;
esac
```

#### break与continue

与其它语言一样，break用于跳出循环，continue用于回到循环初始处
