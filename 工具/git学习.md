---
title: git学习
date: 2021-10-16 10:43:50
tags: git
---

git 基本命令记录

<!-- more -->

<!-- toc -->



## 前言

查看 git 所支持的命令

```bash
$git --help
```

查看某一命令的官方文档

```bash
$git <command> --help
```



## git三大区域

git一共划分为三大区域: 工作区(Working Directory)，暂存区(Stage,又叫Index)，版本库(commit History)，还有一个远程仓库区，不过这不属于git

1. 工作区就是本地文件夹
2. 暂存区指通过`add`命令后文件到达区域
3. 版本库指`commit`后文件到达区域

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031633804.png)

## git基本命令

### 创建git库

进入某个空文件夹(不一定需要空文件夹，但是建议每个project都单独使用空文件夹)

```bash
$git init
```

成功后git会在此目录下生成 `.git` 文件夹

`.git`包括暂存区和版本库



### 将文件暂存到暂存区

假设在本文件夹下新增一个`test.txt`文件

```bash
$git add test.txt
```

要将所有后缀为txt文件都添加，可以用 *.txt

要将所有文件都添加，直接用 . 或 *

```bash
$git add *.txt
$git add *
$git add .
```

`git add .` 会把本地所有untrack的文件都加入暂存区，并且会根据`.gitignore`做过滤，但是`git add *` 会忽略`.gitignore`把任何文件都加入

##### error:

- 假若出现 `'...' does not have a commit checked out` 是因为在该仓库下还有其他 git 仓库，不允许仓库里面还有仓库



PS: 一般只用 `git` 维护纯文本文件



### 将暂存文件提交到版本库(history)

```bash
$git commit -m <message>
```

其中`<message>`是本次提交所附带的注释，用双引号包裹注释内容



### 查看working tree中文件的状态

```bash
$git status
```

在工作区中新增的文件在`Untracked files`下，修改的文件在`modified`下

这里会做两次对比，工作区 vs 暂存区 以及 暂存区 vs 版本库



### diff查看修改

```bash
$git diff [<file>]
```

结果之中`--`代表删除的内容，`++`代表新增的内容

`<file>`参数是可选的，不填的话默认检查所有被修改文件

给diff不同的参数会比较同一文件在不同分区的修改

```bash
$git diff 			# 工作区 <-> 暂存区
$git diff HEAD		# 工作区 <-> 版本库
$git diff --cached	# 暂存区 <-> 版本库
```



### 查看每次commit日志

```bash
$git log
```

显示结果从上到下依次是最近`commit`到最远`commit`

`commit`后面一串符号表示当时提交所生成的`commit_id`



### 回退

HEAD表示当前版本，在当前版本的基础上将工作区回退 n 次

```bash
$git reset --hard HEAD~n	# n是需要回退的次数
```

假如现在是第5次提交，回退3次， 则`n=3`

HEAD其实是一个指针，指向的是不同版本的结点



将工作区回退到指定`commit_id`时的状态,

有趣的是，哪怕现在在很靠之前的状态，通过`commit_id`也可以”回到未来“

```bash
$git reset --hard commit_id	# commit_id是想要回到的状态
```

`commit_id`没有必要写全，只要写前几位，使得没有歧义就行了

PS：回退之后log的结果也会回到当时的状态



这里的 --hard 参数可以修改，修改后命令有不同的含义

```bash
git reset –soft commit_xxid   	# 改了暂存区和工作区，版本库还是当前这个样子

git reset –mixed commit_xxid 	# 改了版本库和暂存区，工作区还是当前这个样子

git reset –hard commit_xxid  	# 改了版本库、暂存区和工作区，三个分区都变了
```



### 查看版本变更历史

```bash
$git reflog
```

打印结果表示HEAD在不同`commit_id`之间跳跃的历史，用于”回到未来“

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031634117.png)



### 撤销修改

撤销不同于回退，回退是在不同的版本间跳跃

- 修改了某文件，还没有`add`，但是想撤销本次修改。相当于用暂存区来还原工作区

   ```bash
   $git checkout -- <file>
   ```

- 修改了某文件，有`add`，但没有`commit`，但是想撤销本次修改。相当于用版本库来还原暂存区，再用暂存区还原工作区

   ```bash
   $git reset HEAD <file>	
   $git checkout -- <file>
   ```

- 修改文件都已经`commit`了，使用回退`commit_id`吧



### git和远程仓库对接(以github为例)

- 新建一个SSH key, 双引号内填邮箱。完成后主目录下会生成两个文件，一个私钥 `id_rsa`, 一个公钥 `id_rsa.pub`

   ```bash
   $ssh-keygen -t rsa -C "<youremail>@example.com"
   ```

- 登录github，打开settings，填上任意title，并把`id_rsa.pub`内的内容复制到Key文本框里，之后点击`add key`

   ![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031634253.png)

- 将git关联到某一个github仓库, 将命令中的  `yourName` 替换为你的名字，  `yourRepoName`替换为你想关联的仓库名

- 这些信息一般可以在仓库的 SSH 信息处获得

   ```bash
   $ git remote add origin git@github.com:<yourName>/<yourRepoName>.git
   ```

   完成后，远程仓库的别名就叫`origin`了，你也可以将它改成别的名字，但这个名字是默认叫法

- 推送到远端仓库, 第一次推送建议加 `-u`， 可以将远端分支初始化，并将本地分支和远端分支相关联，以后再推送就不用 `-u` 了。另外，加 `-f` 可以强推，但是有风险，不推荐

   ```bash
   $git push [-u] origin master [-f]
   ```

- 第一次往github推送会有一个警告

   ```bash
   The authenticity of host 'github.com (xx.xx.xx.xx)' can't be established.
   RSA key fingerprint is xx.xx.xx.xx.xx.
   Are you sure you want to continue connecting (yes/no)?
   ```

   输入yes即可

- 查看远程库信息

   ```bash
   $git remote -v
   origin  git@gitee.com:theSprog/myblog.git (fetch)
   origin  git@gitee.com:theSprog/myblog.git (push)
   ```

- 解绑远程库。解绑并不会真正删除远程库，要真正删除，必须手动到github上操作

   ```bash
   $git remote rm <repoName>
   ```

   

从远端clone到本地, 建议ssh协议，速度快

```bash
$git clone git@gitee.com:<yourName>/<yourRepoName>.git
```



从远程服务器获取到一个branch分支的更新到本地，并更新本地库，叫做pull。

pull = fetch + merge

```bash
$git pull git@gitee.com:theSprog/myblog.git
```



### 创建分支、合并分支

- 创建分支，使用`branch`命令

   ```bash
   $git branch <name>
   ```

- 切换分支，使用 `checkout` 或 `switch` 命令

   ```bash
   $git checkout <name> 	# $git checkout -b <name> 参数表示创建并切换
   $git switch <name> 		# $git switch -c <name> 参数表示创建并切换
   ```

- 查看有多少分支，当前处于哪个分支

   ```bash
   $git branch
   ```

- 合并指定分支到当前分支, 若指定分支相对于当前分支只是新增了东西，则合并是顺利的

   ```bash
   $git merge <name>	# 一般而言都是将 dev 分支合并到 master 分支
   
   # 上面那种写法可能会用Fast-forward模式，不会新建一个commit，这种写法会新建一个commit
   # --no-ff 表示禁用 Fast-forward
   $git merge --no-ff -m "merge with no-ff" dev	
   ```

- 删除分支, 使用 `branch` 的 `-d `参数

   ```bash
   $git branch -d <name>
   ```

- 当git无法自动合并分支时，就必须首先解决冲突。解决冲突就是把git合并失败的文件手动编辑为我们希望的内容，再提交。（吐槽一句：居然还要手动）



### 分支管理策略

首先，`master`分支应该是非常稳定的，也就是仅用来发布新版本，平时不能在上面干活；

那在哪干活呢？干活都在`dev`分支上，也就是说，`dev`分支是不稳定的，到某个时候，比如1.0版本发布时，再把`dev`分支合并到`master`上，在`master`分支发布1.0版本；

开发人员每个人都在`dev`分支上干活，每个人都有自己的分支，时不时地往`dev`分支上合并就可以了。



### bug修复策略

//TODO，没有大团队工程实践，暂弃，后补
