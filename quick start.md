---
title: Quick Start
data: 
---
Welcome to [Hexo](https://hexo.io/)! This is your very first post. Check [documentation](https://hexo.io/docs/) for more info. If you get any problems when using Hexo, you can find the answer in [troubleshooting](https://hexo.io/docs/troubleshooting.html) or you can ask me on [GitHub](https://github.com/hexojs/hexo/issues).



<!--more--> 

<!-- toc -->

## Quick Start



### Create a new post

``` bash
$ hexo [new | n] "My New Post"
```

More info: [Writing](https://hexo.io/docs/writing.html)

### Run server

``` bash
$ hexo [server | s]
```

More info: [Server](https://hexo.io/docs/server.html)

### Generate static files

``` bash
$ hexo [generate | g]
```

More info: [Generating](https://hexo.io/docs/generating.html)

### Deploy to remote sites

``` bash
$ hexo [deploy | d]
```

More info: [Deployment](https://hexo.io/docs/one-command-deployment.html)

### Delete database and public folder

```bash
$hexo clean		#每次修改配置文件，安装插件等都需要clean一下
```

### Only display head

```bas
<!--more-->
```

### Support Latex

[如何在Hexo中支持Latex](https://blog.csdn.net/weixin_44191286/article/details/102702479)

### Latex formula

```latex
$$
	this is a formula
$$
```

eg:

```latex
$$
h_{\theta}(x) = \theta + \theta_{1}x
$$
```

And this above will be render to this:
$$
h_{\theta}(x) = \theta + \theta_{1}x
$$
format \``` latex ``` is unnecessary, inline math is **two $**



### latex math symbols

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202203041615532.jpg)

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202203041615331.jpg)

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202203041616024.jpg)

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202203041616284.jpg)





### Use tcyun_COS

[使用腾讯COS作为图床并在Typora通过PicGo-Core使用](https://blog.csdn.net/guo_ridgepole/article/details/108257277)

shortcut: `ctrl + shift + I`

This is a test:

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202112031635923.jpeg)



### Hexo-toc

```bash
npm install hexo-toc --save
```

接着在_config.yml里面配置

```
toc:
  maxDepth: 3
```

