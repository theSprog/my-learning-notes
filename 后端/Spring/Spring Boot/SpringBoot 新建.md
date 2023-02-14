### IDEA创建

- #### File -> New -> Project

  <img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220816132608504.png" alt="image-20220816132608504" style="zoom:50%;" />



- #### Spring Initializer -> 配置选项 -> next

- #### （本质上也是从 start.spring.io 创建）

  <img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220816132704877.png" alt="image-20220816132704877" style="zoom:50%;" />

- #### 勾选需要的插件

- #### （一般需要SpringBoot DevTools 和 Spring Web）

- #### Finish

  <img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220816132909270.png" alt="image-20220816132909270" style="zoom:50%;" />



#### 一般而言，我们的项目如果前后端分离，则可以将 main/resources 下的 static 和 templates 删除

<img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220816133325421.png" alt="image-20220816133325421" style="zoom:50%;" />





### 注意点

1. #### SpringBoot 默认嵌入 tomcat , 默认端口号 8080

2. #### 当然也可以用传统的方式打包成 war 包，放入单独的 tomcat 中运行

3. #### 由于有maven-wrapper所以可以不用提前下载 maven, 由它去下载 maven

4. #### 在 settings -> File Encodings 中将所有编码改为 UTF-8

5. #### (可选)将 maven 选为本地 maven home 目录，settings file 和 repository 全都 override（使用内置的即可）

6. #### (可选) 在 resources 下新增 banner.txt 可以定制启动图案

#### 