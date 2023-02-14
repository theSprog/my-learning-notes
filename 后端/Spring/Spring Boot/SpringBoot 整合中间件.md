# RabbitMQ

由 erlang 语言开发，使用 AMQP 协议，在应用与应用之间通信

![image-20220925123940031](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220925123940031.png)

## 安装

### windows

安装 RabbitMQ 需要下载两项内容：

- RabbitMQ 安装包

  - 设置 `RABBITMQ_SERVER` 为安装包目录
  - 在 path 中添加 `sbin` 目录

- Erlang 运行环境

  - 设置 `ERLANG_HOME` 为安装包目录
  - 在 path 中添加 `bin` 目录的环境变量

  > 注意好 RabbitMQ 和 Erlang 运行版本的对应

所有前置条件准备好后，在 shell 中键入

```shell
$rabbitmq-plugins.bat enable rabbitmq_management
# 启动, 加 `-detached` 为后台运行模式
$rabbitmq-server.bat start -detached
```

访问 http://localhost:15672 查看，账号密码都是 `guest`

常用命令：

```shell
$rabbitmqctl status
$rabbitmqctl stop
```



### linux

访问官网，对不同的发行版本不同的安装策略



## 添加新用户

可以使用 web gui 添加，也可以使用命令行添加

使用 `add_user` 命令添加新用户，第一个参数是用户名，第二个是密码

```shell
$rabbitmqctl add_user test testpwd
```

设置该用户的角色

```shell
$rabbitmqctl set_user_tags test administrator
```

设置权限（假设所有权限）

```shell
$rabbitmqctl set_permissions -p / test ".*" ".*" ".*"
```

在 web gui 中即可查看所有的 `user`

![image-20220925140146005](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220925140146005.png)

## 添加新 virtual host

- 点击 Admin 进入管理设置
- 选择 virtual hosts 进行相应设置
- 输入 virtual host  名称
- 点击 Add virtual host 保存
- 点击对应的 virtual host，在该 host 下添加用户，点击 `set permission` 确认；点击 `clear` 清除用户

![image-20220925141202784](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220925141202784.png)





## springboot 整合

### 依赖配置

maven 引入

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-amqp</artifactId>
</dependency>
```

application 整合

```properties
# IP 地址
spring.rabbitmq.host=127.0.0.1
# 端口号，15672 只是web页面的端口，5672才是高级消息队列协议端口
spring.rabbitmq.port=5672

# 配置好虚拟主机，用户名密码
spring.rabbitmq.virtual-host=virtest
spring.rabbitmq.username=test
spring.rabbitmq.password=testpwd
```



### 创建 Exchange 和 Queue

五步：

- 定义交换机的名称  
- 定义队列的名称
- 创建交换机
- 创建队列
- 绑定队列和交换机（把队列绑定到交换机）

注意：`Exchange`和 `Queue` 均来自 `org.springframework.amqp.core`

```java
@Configuration
public class RabbitMQConfig {
    //定义交换机的名称
    public static final String EXCHANGE_MSG = "exchange_msg";
    //定义队列的名称
    public static final String QUEUE_MSG = "queue_msg";

    //创建交换机
    @Bean(EXCHANGE_MSG)
    public Exchange exchange(){
        return ExchangeBuilder
                .topicExchange(EXCHANGE_MSG)    // 交换机类型
                .durable(true)                  // 重启后原先数据是否保留
                .build();
    }

    //创建队列
    @Bean (QUEUE_MSG)
    public Queue queue(){
        return new Queue(QUEUE_MSG);
    }
    
    //绑定队列到交换机上
    @Bean
    public Binding binding(
            @Qualifier(QUEUE_MSG) Queue queue,
            @Qualifier(EXCHANGE_MSG) Exchange exchange) {
        return BindingBuilder
                .bind(queue)
                .to(exchange)
                .with("test.msg.*")
                .noargs();
    }
}
```



### 创建生产者和消费者

RabbitMQ 的路由规则

`test.msg.*` 中的 `*` 表示一个占位符，比如

- `test.msg.display` ：匹配
- `test.msg.do.display` ：不匹配

同理， `test.msg.*.*` 表示两个占位符

`test.msg.#`中的`#` 表示多个占位符，`test.msg`  也可以被匹配到



#### 生产者

生产者发送消息

```java
@GetMapping("producer")
    public String producer(){
        rabbitTemplate
            .convertAndSend(RabbitMQConfig.EXCHANGE_MSG, "test.msg.send", "This is a msg: " + new Data());
        rabbitTemplate
            .convertAndSend(RabbitMQConfig.EXCHANGE_MSG, "test.msg.delete", "This is a new msg: " + new Data());

        return "OK";
    }
```

访问该路由若干次，则向 rabbitMQ 发送消息

回到 web 管理页面，点击 `Queue` 可看到接收到消息![image-20220925150252569](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220925150252569.png)



#### 消费者

注意

-  `RabbitListener` 来自 `org.springframework.amqp.rabbit.annotation.RabbitListener;`
- `Message` 来自 `org.springframework.amqp.core.Message`;

```java
@Component
public class RabbitMQConsumer {
    // queues 表示监听的队列, 只有一个的时候可以直接写出
    @RabbitListener(queues = RabbitMQConfig.QUEUE_MSG)
    public void watchQueue(String payload, Message message) {
        System.out.println("payload: " + payload);
        System.out.println("message: " + message);
    }
}
```





# MongoDB

MongoDB 是一个 nosql 数据库，提供内存级别查询，不支持事务，提供了 GridFS 小文件存储方式（短视频，图片）



## 安装

在官网的 `product` 中选择 `community` 社区版本，下载安装，安装时选择 `Custom` 自定义安装路径。

可以选择 `MongoDB Compass`，它自带 GUI 系统，同时自带 `mongo shell`

mongdb6 不再自带 shell 系统，需要自己下载 https://www.mongodb.com/try/download/shell，使用 `mongosh.exe` 连接数据库



### 配置用户权限

开启认证：在配置文件中加入

```
security:
  authorization: enabled
```

使用 `mongosh` 连接数据库后：

```shell
$use admin
$db.createUser({user:"root",pwd:"root",roles:["root"]})

# 认证，否则会面可能会需要权限
$db.auth("root","root")

# 切换到某数据库，如果没有则创建
$use DATABASE_NAME

# 插入一条数据，否则 show 查询不到该数据库
$db.mongoTest.insertOne({"key":"this is a test key"})

# 查询数据库
$show dbs
```



## springboot 整合

maven 引入

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-data-mongodb</artifactId>
</dependency>
```

application 整合

```properties
# database 选择数据库名
spring.data.mongodb.database=mongoTest
# root:root 是 用户名:密码
spring.data.mongodb.uri=mongodb://root:root@localhost:27017
```



假设使用 JPA 方式

创建存储对象, `@Document("MyCollection")` 是指要在数据库中新增一个 Document，它属于 `MyCollection` 这个集合

```java
@Document("MyCollection")
@Data
@ToString
@NoArgsConstructor
@AllArgsConstructor
public class MongoTestMO {
    // 必须要有一个主键
    @Id
    private String key;
	
    // Field 表示映射到数据库的什么字段，没有则默认两字段相同
    @Field("name")
    private String name;

    @Field("sex")
    private String sex;
}
```

使用 JPA 方式完成对数据库的交互

```java
// 使用 JPA 方式进行数据层操作，继承之后就直接包含许多通用方法
// 第一个参数为操作的 Document, 第二个参数为主键的类型
public interface MongoDAO extends MongoRepository<MongoTestMO, String> {
}
```

使用时，`new`  Document 后选择对应的操作（例如这里的 `save`）

```java
@GetMapping("mongo")
public String mongo(){
    MongoTestMO mongoTestMO = new MongoTestMO("1002", "jack", "male");
    mongoDAO.save(mongoTestMO);
    return "mongo OK";
}
```

在 MongoDB Compass 刷新数据库即可看到结果

![image-20220925175534612](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220925175534612.png)





# MinIO

官网下载 MinIO 后，打开 MinIO

启动 MinIO，所有接受到的文件都存在 `<somewhere>` 文件夹下

```shell
$minio server <somewhere>
```

记录下登录用户名和密码，

启动后访问 http://localhost:9000，进入登录页面，输入密码。



进入管理页面后点击 `Buckets -> create Bucket` ，创建桶。在 `Buckets` 中管理所有 bucket，browse 可以进行上传下载，分享与预览服务

在 Buckets 中点击 `manage` ，将 `Access Policy` 设置为 `public`.

![image-20220925215115014](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220925215115014.png)

## springboot 整合

maven 引入，此处需要自己指定版本

```xml
<dependency>
    <groupId>io.minio</groupId>
    <artifactId>minio</artifactId>
    <version>8.4.3</version>
</dependency>

<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-configuration-processor</artifactId>
    <optional>true</optional>
</dependency>
```

properties

```properties
# minio 服务器地址
minio.endpoint=http://127.0.0.1:9000
# 文件地址 host
minio.fileHost=http://127.0.0.1:9000
# 存储桶名称
minio.bucketName=mybucket
# 用户名与密码
minio.accessKey=minioadmin
minio.secretKey=minioadmin
```



在 java 程序中创建：

- 属性程序

```java
@Data
@ConfigurationProperties(prefix = "minio")
@Component
public class MinioProp {
    private String endpoint;
    private String accessKey;
    private String secretKey;
    private String bucketName;
    private String fileHost;
}
```

- minio客户端配置文件

```java
@Configuration
public class MinioConfiguration {
    @Autowired
    private MinioProp minioProp;

    @Bean
    public MinioClient minioClient() throws Exception {
        String endpoint = minioProp.getEndpoint();
        String accessKey = minioProp.getAccessKey();
        String secretKey = minioProp.getSecretKey();
        String bucketName = minioProp.getBucketName();

        MinioClient minioClient = MinioClient
                .builder()
                .endpoint(endpoint)
                .credentials(accessKey, secretKey)
                .build();

        if(!minioClient.bucketExists(BucketExistsArgs.builder().bucket(bucketName).build())){
            minioClient.makeBucket(MakeBucketArgs.builder().bucket(bucketName).build());
        }

        return minioClient;
    }
}
```

- minio 使用程序

```java
@Autowired
private MinioClient minioClient;

@Autowired
private MinioProp minioProp;

@PostMapping("upload")
public String upload(MultipartFile file) {
    String filename = file.getOriginalFilename();
    try (InputStream in = file.getInputStream()) {
        LOG.info("开始上传文件");
        minioClient.putObject(PutObjectArgs.builder()
                              .bucket(minioProp.getBucketName())
                              .object(filename)
                              .stream(in, in.available(), -1)
                              .build());
        LOG.info("上传文件成功");
    } catch (Exception e) {
        LOG.error(e.getMessage());
        return "fail";
    }
    String url = minioProp.getFileHost()
        + "/"
        + minioProp.getBucketName()
        + "/"
        + filename;
    return "OK: " + url;
}
```

上传文件即可，成功后 `OK` 后紧跟该文件的 URL 地址。也可以去管理后台查看该文件是否存在

minio 还有很多其他的用法，详情请查看官方文档
