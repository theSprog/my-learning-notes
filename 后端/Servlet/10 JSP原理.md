### JSP

JSP 是 Java Server Pages 的缩写，JSP 也是 javaEE 的规范之一，每一个 web 服务器都会内置一个 JSP 翻译引擎

JSP 实际上就是一个 Servlet，只不过由服务器将 JSP 文件翻译为 Java 文件，然后再将此 Java 文件编译为 class 字节码交由 JVM 调用。

在访问 `index.jsp`  时，实际上就是执行 `index_jsp.class` 字节码，它来自 `index_jsp.java`

JSP 类实际上是 HttpJspBase，而 HttpJspBase 继承自 httpServlet，所以 index_jsp 类实际上就是一个 Servlet 类。他们的生命周期一样而且都是单例的

```java
// index_jsp.java
public final class index_jsp extends HttpJspBase
    
// HttpJspBase
public abstract class HttpJspBase extends HttpServlet
```



### 访问过程

JSP第一次访问时效率较低，因为只有当第一次访问 jsp 文件时，服务器才会创建对应的文件夹，把该 jsp 文件翻译为 *_jsp.java 文件并放在该文件夹下，之后再次访问时才会直接执行该文件，不用再次生成。所以第一次访问较慢。

<img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220915135706831.png" alt="image-20220915135706831" style="zoom:50%;" />

第一次比较麻烦：

- 要把 jsp 文件翻译生成 Java 源文件
- Java 源文件要编译生成 class 字节码文件
- 然后通过 class 去创建 servlet 对象
- 然后调用 servlet 对象的 init 方法
- 最后调用 servlet 对象的 service 方法



第二次就比较快：

- 因为第二次直接调用单例 servlet 对象的 service 方法即可



### 区别

JSP 和 Servlet 都是 Servlet，他们的区别是：职责不同

- JSP：展示数据
- Servlet：处理数据，和数据库打交道