### ServletConfig是什么

是一个接口，是 Servlet 规范的一员。Tomcat 服务器实现了该接口，并且在 `init` 时作为参数传入 Servlet 中。

一个 Servlet 对象会关联一个 ServletConfig，不同的 Servlet 对象关联不同的 ServletConfig。ServletConfig对象中包装的信息是 web.xml 文件中 `<servlet></servlet>` 标签的配置信息

```java
ServletConfig servletConfig = this.getServletConfig();
```



假设 `web.xml` 如下

```xml
<servlet>
    <servlet-name>test</servlet-name>
    <servlet-class>org.example.TestServlet</servlet-class>
    <init-param>
        <param-name>driver</param-name>
        <param-value>com.mysql.cj.jdbc.driver</param-value>
    </init-param>
</servlet>
```



#### getServletName 

获取 servlet-name 内容

```java
String servletName = servletConfig.getServletName();	// test
```





#### getInitParameter 

获取 init-param 中 param-name 对应的 param-value，不存在则返回 null。传入参数为 param-name。

```java
String driver = servletConfig.getInitParameter("driver");	// com.mysql.cj.jdbc.driver
```

可以直接通过 `GenericServlet` 父类的 `getInitParameter` 代替 `ServletConfig` 去取 InitParameter，这本质只是一层封装而已

```java
String driver = this.getInitParameter("driver")
```





#### getInitParameterNames 

获取 init-param 中所有 param-name，无参

```java
Enumeration<String> initParameterNames = servletConfig.getInitParameterNames();
while (initParameterNames.hasMoreElements()){
    String name = initParameterNames.nextElement();
    String val = servletConfig.getInitParameter(nextElement);
    out.print("<h1>" + name + "=" + val + "</h1>");
}
```

可以直接通过 `GenericServlet` 父类的 `getInitParameters` 代替 `ServletConfig` 去取 InitParameters，这本质也只是一层封装而已

```java
Enumeration<String> initParameterNames = this.getInitParameterNames()
```

