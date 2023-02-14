### 步骤

- 第一步：在webapps目录下新建一个目录，起名随意(一般是项目名称)。

- 第二步：在webapp目录下新建一个目录：**WEB-INF**

  - 注意：这个目录的名字是 Servlet 规范中规定的，必须全部大写，必须一模一样。

- 第三步：在 WEB-INF 目录下新建一个目录：**classes**

  - 这个目录的名字必须是全部小写的 classes。这也是 Servlet 规范中规定的。
  - 另外这个目录下一定存放的是 Java 程序编译之后的 class 文件（这里存放的是字节码文件）。

- 第四步：在 WEB-INF 目录下新建一个目录：**lib**

  - 这个目录不是必须的。但如果一个 webapp 需要第三方的 jar 包的话，这个 jar 包要放到这个 lib 目录下，
  - 这个目录的名字必须是全部小写的lib。这也是 Servlet 规范中规定的。

- 第五步：在 WEB-INF 目录下新建一个文件：**web.xml**

  - 这个文件是必须的，这个文件名必须叫做 web.xml。
  - 这个文件所在位置必须是这里。
  - 这个 web.xml 文件就是一个配置文件，在这个配置文件中描述了 **请求路径** 和 **Servlet类** 之间的对照关系。

- 第六步：编写 JavaWeb 程序，该程序实现了 Servlet 接口

  - 版本说明：

    Tomcat 10 及以上的 Servlet 规范已经从 Javax 迁移到 Jakarta，换句话说要使 WebApp 运行在 Tomcat10 中，需要实现的 Servlet 接口应来自 Jakarta

    ```java
    import jakarta.servlet.http.HttpServlet;
    ...
    ```

  - 编译时需要使用 Tomcat 提供的 jar 包，在 Tomcat 目录的 lib 文件夹下的 servlet-api.jar，将其加入 classpath 中，以使 `javac` 编译通过

- 第七步：将编译生成的字节码文件夹及文件放在 WEB-INF/classes 下

- 第八步：在 web.xml 中配置必要信息，使得 请求路径 和 Servlet 类 关联到一起（注册 Servlet 类）

  ```xml
  <!--servlet描述信息-->
  <!--两处servlet-name应该保持一致-->
  <servlet>
      <servlet-name></servlet-name>
      <servlet-class></servlet-class>
  </servlet>
  
  <!--servlet映射信息-->
  <servlet-mapping>
      <servlet-name></servlet-name>
      <!--url以"/"开始-->
      <!--浏览器上的请求路径不能随便写，必须和url-pattern一致-->
      <url-pattern></url-pattern>
  </servlet-mapping>
  ```



假设 JavaWeb 编写如下

```java
import javax.servlet.*;
import java.io.IOException;

public class TestServlet implements Servlet {
    @Override
    public void init(ServletConfig config) throws ServletException {

    }

    @Override
    public ServletConfig getServletConfig() {
        return null;
    }
	
    // service 是被调用的方法
    @Override
    public void service(ServletRequest req, ServletResponse res) throws ServletException, IOException {
        System.out.println("My Servlet Service");
    }

    @Override
    public String getServletInfo() {
        return null;
    }

    @Override
    public void destroy() {

    }
}
```

### ServletResponse

使用 `getWriter` 获取一个 PrintWriter，用于从服务器向浏览器发送数据，不需要用户手动关闭，由 Tomcat 负责

```java
@Override
public void service(ServletRequest req, ServletResponse res) throws ServletException, IOException {
    res.setContentType("text/html");
    PrintWriter out = res.getWriter();
    out.print("<h1> this is a test </h1>");
}
```





### IDEA 支持

- new project

- new module

- 让 module 变成 javaEE 模块（在创建的模块上右击：Add Frameworks Support -> webApplication，它会自动生成合法目录和文件）

  - 必要时需要自己手动在 WEB-INF 目录下新建 lib 文件夹

- 设置 Servlet 依赖路径：File -> Project Structure -> Module -> Dependency -> ＋，选择 jar 路径

  - 主要 + `servlet-api` 和 `jsp-api` 

  <img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220911205831813.png" alt="image-20220911205831813" style="zoom: 50%;" />



### 设置默认页面

```xml
<welcome-file-list>
	<welcome-file>xxx</welcome-file>
    <welcome-file>yyy</welcome-file>
</welcome-file-list>
```

当访问某个服务器时，不指定文件便默认导向欢迎页。该文件默认从 webapp 根目录开始查找，

**注意：路径不需要以 `/` 开始**

一个 webapp 可以设置多个欢迎页，越靠上优先级越高，上一个找不到找下一个。默认根目录下欢迎页 `index.html`（这是在 CATALINA_HOME/conf/web.xml 中可以进行全局配置）

```xml
<welcome-file-list>
    <welcome-file>index.html</welcome-file>
    <welcome-file>index.htm</welcome-file>
    <welcome-file>index.jsp</welcome-file>
</welcome-file-list>
```





### WEB-INF

放在 WEB-INF 下的资源属于服务器敏感资源，无法通过浏览器配置路径访问到，所以静态资源一定要 WEB-INF 之外访问
