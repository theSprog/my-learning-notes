### 一般构建

- 进入官网 https://start.spring.io/，勾选构建工具，所选语言，Spring Boot 版本
- 点击 GENERATE，下载压缩包
- 解压后使用 IDE 打开

![image-20220907162002860](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220907162002860.png)

如果需要 actuator 功能需要引入依赖

```xml
<dependency>  
    <groupId>org.springframework.boot</groupId>  
    <artifactId>spring-boot-starter-web</artifactId>  
</dependency>

<dependency>  
    <groupId>org.springframework.boot</groupId>  
    <artifactId>spring-boot-starter-actuator</artifactId>  
</dependency>  
```





### 最简单的使用

若要将某个类用作controller，只需要在其上加上注解 `@RestfulController` 即可

```java
@RestController
@SpringBootApplication
public class DemoApplication {

	public static void main(String[] args) {
		SpringApplication.run(DemoApplication.class, args);
	}
	
    // 将类上注解 @RestController 即可把该类用作 Controller 类
	@GetMapping("/hello")
	public String hello() {
		return "hello";
	}
}
```



打包（跳过测试）。或者使用 maven 插件图形化打包

```shell
$mvn clean package '-Dmaven.test.skip=true'
```



打包后存在两个包，一个是原始的自己写的内容的包，较小；另一个是`maven`将所有依赖捆绑到一起的包，较大，可运行。

```
demo-0.0.1-SNAPSHOT.jar				// 捆绑包（可运行）
demo-0.0.1-SNAPSHOT.jar.original	// 原始包
```

通过 `java -jar xxx.jar` 可直接运行捆绑包