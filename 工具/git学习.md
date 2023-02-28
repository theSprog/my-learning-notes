## 前言

查看 git 所支持的命令

```bash
$git --help
```

查看某一命令的官方文档

```bash
$git <command> --help
```

PS: 一般只用 `git` 维护纯文本文件



创建git库：进入某个空文件夹(不一定需要空文件夹，但是建议每个project都单独使用空文件夹)

```bash
$git init
```

成功后git会在此目录下生成 `.git` 文件夹。`.git`包括暂存区和版本库



---

## git三大区域

git一共划分为三大区域: 工作区(Working Directory)，暂存区(Stage,又叫Index)，版本库(commit History)，还有一个远程仓库区，不过这不属于git

1. 工作区就是本地文件夹
2. 暂存区指通过`add`命令后文件到达区域
3. 版本库指`commit`后文件到达区域

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031633804.png)





---

## 三区转移命令

### 工作区与暂存区

#### 工作区 -> 暂存区

假设在本文件夹下新增一个`test.txt`文件

```bash
$git add test.txt
```

要将所有后缀为txt文件都添加，可以用 `git add *.txt`。要将所有文件都添加，直接用 `.` 或 `*`

```bash
$git add *.txt
$git add *
$git add .
```

- `git add .` 会把本地所有 untrack 的文件都加入暂存区，并且会根据`.gitignore`做过滤
- 但是`git add *` 会无视`.gitignore`把任何文件都加入



error&warning:

- 假若出现 `'...' does not have a commit checked out` 是因为在该仓库下还有其他 git 仓库，git 不允许仓库里面嵌套仓库

- 假若出现 `warning:LF will be replaced by CRLF in xxx` 代表 git 自动将 win 平台下的 CRLF 转换为 LF 换行符

  > 可以配置 git 是否进行自动转换
  >
  > ```shell
  > $git config --global core.autocrlf false/true
  > ```



#### 暂存区 -> 工作区

暂存区到工作区的命令随着版本不固定，当 `git add` 之后可以使用 `git status` 让 git 提示应该使用哪个命令去 `unstage`

```shell
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
```

上面表示，使用 `git restore --staged` 可以将文件撤回到工作区。

**注意**：这不仅可以用来将暂存区的文件撤回到工作区，也能够用于撤销对工作区的修改





### 暂存区到本地库

#### 暂存区 -> 本地库

```shell
$git commit -m "<message>"
```

其中`<message>`是本次提交所附带的注释，需要用双引号包裹注释内容



#### 本地库 -> 暂存区

见 **git基本命令/版本回退**





---

## git基本命令

### 用户签名

设置用户签名的方式有两种：

- 单个仓库设置

  > ```shell
  > git config user.name 用户名
  > git config user.email 邮箱
  > ```
  >
  > 配置好后信息存放在 `.git/config` 中

- 全局配置

  > ```shell
  > git config --global user.name 用户名
  > git config --global user.email 邮箱
  > ```
  >
  > 配置好后信息存放在 `~/.gitconfig` 中

用户名和邮箱可以不真实，只要符合格式即可。用以表示本客户端的名称，在分布式系统中表示某个操作是谁发出的，本账号和 github 无关。







### 查看git仓库的状态

```bash
$git status

On branch xxx	# 表示在 xxx 分支下
Your branch is up to date with 'origin/master'.

Changes not staged for commit:	# 目前相对暂存区而言不是最新的文件
	xxx
	
Changes to be committed:	# 尚未 commit 的文件
	xxx
	
Untracked files:	# 未被 git 管理的文件
	xxx
```

- 在工作区中新增的还未被 Git 管理的文件在`Untracked files`下

- 已经在暂存区备份但是在工作区修改后的文件在`modified`下。

  > `modified` 顾名思义就是修改过的意思。针对的就是已经在暂存区的文件最近又发生了改动的情况。
  >
  > 要想撤销修改，详情见 暂存区->工作区 这一节







### 查看历史版本

```bash
# 详细版本
$git log
```

显示结果从上到下依次是最近`commit`到最远`commit`。`commit`后面一串符号表示当时提交所生成的`commit_id`

```shell
# 简化版本
$git reflog
```

打印结果表示HEAD在不同`commit_id`之间跳跃的历史，用于版本回退

![](https://src-1259777572.cos.ap-chengdu.myqcloud.com/202112031634117.png)







### 版本回退

#### 通过 id 回退

将工作区回退到指定`commit_id`时的状态，有趣的是，虽然名字叫回退，但是通过`commit_id`也可以**回到未来**

```bash
$git reset --hard commit_id	# commit_id是想要去到的版本号
```

`commit_id`没有必要写全，只要写前几位，使得没有歧义就行了。`hard` 的含义是三个分区都回退到当时

PS：回退之后 `git log` 的结果也会回到当时的状态。



#### 相对 HEAD 回退

HEAD表示当前版本，在当前版本的基础上将工作区回退 n 次

```bash
$git reset --hard HEAD~n	# n是需要回退的次数
```

假如现在是第5次提交，回退3次， 则`n=3`。HEAD其实是一个指针，指向的是不同版本的结点。

可以通过查看 `.git/HEAD` 文件来查看当前的 `HEAD` 指针指向哪个文件。



#### --hard

这里的 --hard 参数可以修改，修改后命令有不同的含义

```bash
git reset –soft commit_xxid   	# 改了暂存区和工作区，版本库还是当前这个样子

git reset –mixed commit_xxid 	# 改了版本库和暂存区，工作区还是当前这个样子

git reset –hard commit_xxid  	# 改了版本库、暂存区和工作区，三个分区都变了
```





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







## git 分支操作

### 分支是什么

在版本控制中，可以同时推进多个任务，为每个任务设置一个主线的副本，每个团队在自己的副本上开发，等待开发完成后再合并回主线上。分支简单来讲就是主线的副本。

同一个主线可以衍生出多个分支，相互独立互不影响，因此很符合劳动分工理念。如果某个分支开发失败，直接删除此分支即可，不会对其他分支产生影响。



### 分支管理策略

首先，`master`分支应该是非常稳定的，也就是仅用来发布新版本，平时不能在上面干活；

那在哪干活呢？干活都在`dev`分支上，也就是说，`dev`分支是不稳定的，到某个时候，比如1.0版本发布时，再把`dev`分支合并到`master`上，在`master`分支发布1.0版本；

每个开发人员都在`dev`分支上干活，每个人都有自己的分支，时不时地往`dev`分支上合并就可以了。



### 分支操作

| 作用     | 命令                                              |
| -------- | ------------------------------------------------- |
| 创建分支 | git branch \<name>                                |
| 切换分支 | git checkout \<name> 或者是 git switch \<name>    |
| 查看分支 | git branch -v                                     |
| 删除分支 | 使用 `branch` 的 `-d `参数：git branch -d \<name> |

其他复杂操作：

- 合并指定分支到当前分支, 若指定分支相对于当前分支只是新增了东西，则合并是顺利的

  ```bash
  $git merge <name>	# 一般而言都是将 dev 分支合并到 master 分支
  
  # 上面那种写法可能会用Fast-forward模式，不会新建一个commit，这种写法会新建一个commit
  # --no-ff 表示禁用 Fast-forward
  $git merge --no-ff -m "merge with no-ff" dev	
  ```



### 分支冲突

合并分支时，两个分支在**同一个文件**有两套完全不同的修改。Gt无法替我们决定使用哪一个。必须人为决定新代码内容。

> 当出现 `merge conflict`  时 `git status` 的状态会出现 `both modified`，表示该文件存在两个修改。同时 branch 的状态也变成 `Merging`。

当 git 无法自动合并分支时，就必须首先解决冲突再合并。解决冲突就是把 git 合并失败的文件手动编辑为我们希望的内容，再提交。（吐槽一句：居然还要手动）

解决方法：

打开冲突文件，冲突处会出现如下字样

```shell
<<<<<<< HEAD
xxxxx1
xxxxx2
=============
yyyyy1
yyyyy2
>>>>>>> hot-fix
```

上面的内容表示 `xxxx` 和  `yyyy` 存在冲突，手动选定要保留的内容，同时删除其他 `git` 的标记，修改完后再次 `git add`。

例如可以将文件修改为如下形式同时删除 git 标记：

```
xxxxx1
yyyyy2
```

然后再 `git add .` 并且 `git commit` 提交，一切完备后分支状态也就不再是 `Merging` 了

![image-20230227220201648](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20230227220201648.png)



### bug修复策略

//TODO，没有大团队工程实践，暂弃，后补





---

## 团队协作

### push

push 命令用于从将本地的分支上传到远程的某一分支并与其合并。

```shell
$git push <远程主机名> <本地分支名>:<远程分支名>
```

如果本地分支名与远程分支名相同，则可以省略冒号。

例如：

```shell
$git push origin master
```

上面命令表示，将本地的 master 分支推送到远程的 master 分支上，并与其合并



### pull

pull = fetch + merge。从远程服务器获取到一个 branch 分支的更新到本地，并更新本地的某一分支，叫做pull。

```shell
$git pull <远程主机名> <远程分支名>:<本地分支名>
```

远程主机名可以通过 `git remote -v` 查看。如果远程分支是**与当前分支合并**，则冒号后面的部分可以省略。

例如：

```shell
$git pull origin master
```

上面命令表示，取回 origin 的 master 分支，再与本地的当前分支合并



### fetch

fetch 命令用于从远程获取代码库，该命令执行完后需要手动执行 `git merge` 远程分支到你所在的分支。







---

## 远端对接

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




### 查看远程库信息

```bash
$git remote -v
origin  git@gitee.com:theSprog/myblog.git (fetch)
origin  git@gitee.com:theSprog/myblog.git (push)
```



### 解绑远程库

解绑并不会真正删除远程库，要真正删除，必须手动到github上操作

```bash
$git remote rm <repoName>
```

