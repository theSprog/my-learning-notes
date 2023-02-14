### page 指令

有时我们需要在 jsp 中设置全局指令，例如页面编码等，此时需要在 jsp 文件第一行加上 page 指令，通过该指令设置响应的内容类型

```jsp
<%@page xxx %>
```



#### 设置编码

```jsp
<%--例如--%>
<%@page contentType="text/html;charset=UTF-8" %>
<%--或者--%>
<%@page pageEncoding="UTF-8" %>
```

#### 访问 JSP 时不生成 session 对象

默认访问 jsp 会创建 session 对象

```jsp
<%@page session="false" %>
```

但是这种写法也会使得 jsp 文件的 session 对象无法使用





### 插入 Java 程序

#### <% %> 标记

想要在 jsp 中插入 Java 语句，放在 `<% %>` 之中。在这个符号当中编写的被视为 java 语句，被翻译到 Servlet 类的 service方法内部。

```jsp
<%
	System.out.println("hello jsp"); 
%>
```

**注意：**

- `<!-- xxx -->` 是 html 的注释，这种注释也会被翻译进 java 文件。`<%-- xxx --%>`
- 在 `<% %>` 之中的语句被视为 `service` 方法内部的语句，必须符合 Java 规范，例如不可以在方法内部再次声明方法



向浏览器输出 Java 变量，可以直接使用 out 变量，这是 jsp 的九大内置变量之一

```jsp
<%
	String name = "myNmae";
	out.write("hello " + name);
%>
```

九大内置对象，这九大对象只能在 service 中使用，即 `<% %>` 中使用

```
pagecontext;
session;
application;
config;
out;
page;

request;
responose;

内部异常;
```



#### <%! %> 标记

要想在类体中插入 Java 语句，使用 `<%! %>` 格式，例如在此处定义成员变量等

```jsp
<%! 
    private int i;
	static {
        System.out.println("hello jsp");
    }

	public static void m1(){
		System.out.println("m1 method execute!");
    }
%>
```

然而这个语法很少用，不建议使用。

因为在service方法外面写静态变量和实例变量，都会存在线程安全问题：因为 JSP 就是 Servlet, Servlet是单例的，多线程并发的环境下，这个静态变量和实例变量一旦有修改操作，必然会存在并发安全问题。



#### <%= %> 标记

如果需要输出一个表达式可以使用简化的格式：`<%= %>`。该标记最终被翻译成了 `out.print();`。内部的表达式不必加分号

```jsp
<%= 1 + 2 %>

<%--相当于是--%>
<%
	out.print( 1 + 2 );
%>
```

