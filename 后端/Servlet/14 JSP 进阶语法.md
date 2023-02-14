### JSP 的指令

#### page 指令

- 设置是否开启会话

  ```jsp
  <%@page session="true|false"%>
  ```

  默认为 true，如果设置为 false 在当前 jsp 中无法使用 session 对象



- 设置响应的内容类型

  ```jsp
  <%@page contentType="text/json"%>
  ```

  

- 设置响应的字符集

  ```jsp
  <%@page pageEncoding="UTF-8"%>
  相当于
  <%@page contentType="xxx;charset=UTF-8"%>
  ```

   

- Java 导包

  ```jsp
  <%@page import="java.util.*"%>
  ```

   

- 设置当前页面出错时 errorPage，例如空指针异常时跳转页面

  ```jsp
  <%@page errorPage="/error.jsp"%>
  ```

   

- 如果当前页面是错误页，向程序员反馈信息

  ```jsp
  <%@page isErrorPage="true"%>
  
  <%
  	exception.printStackTrace();
  %>
  ```

  `exception` 是九大内置对象之一，它的类型是 `java.lang.Throwable` ，可以直接使用（只有当 `isErrorPage` 设置为 `true` 才能使用）

   
  
- 是否忽略 EL 表达式，默认是 `false`

  ```jsp
  <%@page isELIgnored="true"%>
  ```

  



### 九大内置对象

- javax.servlet.jsp.PageContext **pageContext**

- javax.servlet.http.HttpServletRequest **request**

- javax.servlet.http.HttpSession **session**

- javax.servlet.ServletContext **application**

  - 以上四个作用域，大小分别为 pageContext < request < session < application
  - 以上这四个域都有 `setAttribute`、`getAttribute`、`removeAttribute `方法

  

- java.lang.Throwable **exception**

  

- javax.servlet.ServletConfig **config**

  

- java.lang.Object **page**，

  - 其实就是 `this` 指针，指向 Servlet

  

- javax.servlet.jsp.JspWriter **out**

- javax.servlet.http.HttpServletResponse **response**

  