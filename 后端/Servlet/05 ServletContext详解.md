### 获取 ServletContext 对象

1. 通过 ServletConfig 对象获取

```java
ServletContext servletContext = servletConfig.getServletContext();
```

2. 直接使用父类 GenericServlet 提供的 getServletContext

```java
ServletContext servletContext = this.getServletContext();
```



### ServletContext 是什么

是一个接口，是 Servlet 规范的一员，由WEB服务器实现该接口。在WEB服务器启动时创建，对于一个 webapp 而言，只有一个 ServletContext 对象。而且在服务器关闭时销毁。所有 Servlet 共享同一个 ServletContext 对象

一个 ServletContext 对象通常对应一个 web.xml 对象



### 常用方法

#### 获取上下文初始化参数

与 ServletConfig 一样，具有 `getInitParameter` 与 `getInitParameterNames` 两个方法，但他们是用来获取全局上下文信息的。ServletConfig 是用来获取特定 Servlet 信息的

假设 web.xml 如下，这些配置只能通过 ServletContext 对象来获取

```xml
<context-param>
    <param-name>abc</param-name>
    <param-value>def</param-value>
</context-param>
```

```java
Enumeration<String> initParameterNames = servletContext.getInitParameterNames();
while (initParameterNames.hasMoreElements()){
    String name = initParameterNames.nextElement();
    String val = servletContext.getInitParameter(name);
    out.println(name + " = " + val);
}
```



#### 获取项目根路径

`getContextPath` 返回webapp项目根路径，如果项目部署在根目录（也就是 [http://localhost](http://localhost:8080/)）的话返回 `""`

```java
String contextPath = servletContext.getContextPath();
out.println("contextPath = " + contextPath);
```

idea 中可以在 tomcat 配置的 deployment 的 Application context 中修改



#### 获取文件绝对路径

getRealPath需要一个字符串路径参数，webapp 是文件的起始路径（即 `/`），返回该文件的绝对路径。换句话说就是将相对路径改为绝对路径

```java
String realPath = servletContext.getRealPath("/WEB-INF/web.xml");
// 或者
String realPath = servletContext.getRealPath("WEB-INF/web.xml");
out.println("realPath = " + realPath);
```



#### 日志

log 可以记录日志，该日志会记录到 TOMCAT_HOME/logs 文件夹下。

```java
servletContext.log("hello you");
```

对于IDEA，会将日志记录到 CATALINA_BASE/logs 文件夹下（服务器控制台获取 CATALINA_BASE ）

Tomcat 每次生成多个日志：

![image-20220912172659959](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220912172659959.png)

其中 

- catalina.xxx.log 是启动日志，

-  localhost.xxx.log 是具体服务日志，

- localhost_access_log.xxx.log 是访问日志，记录访问信息



#### 手动设置 Attribute

这种方法可以不在 web.xml 中提前定死所有属性，而是在运行时动态增减

```java
// 增
setAttribute(String name, Object val);
// 删
removeAttribute(String name);

// 查
getAttribute(String name);
```

ServletContext 对象还有另一个名字：应用域（后面还有其他域，例如：请求域、会话域），如果所有的用户共享一份数据，并且这个数据**很少的被修改**（线程安全），并且这个**数据量很少**，可以将这些数据放到 ServletContext 这个应用域中

因为应用域相当于一个缓存，放到缓存中的数据，下次在用的时候，不需要从数据库中再次获取，大大提升执行效率。