### Http 协议

#### HTTP的请求协议包括：4部分

- 请求行
  - 请求方式，如 GET、POST 等
    - GET 请求只能发送字符串，且发送到字符串长度有限，不同的浏览器规定不同
    - GET 无法发送大量数据
    - POST 可以发送任意信息，包括图片视频等，理论上没有数据量限制
    - GET 支持缓存，POST 不支持缓存。只要发送gt请求，浏览器做的第一件事都是先从本地浏览器缓存中找，找不到的时候才会去服务器上获取。
  - URI（统一资源标识符）
    - 与 URL 区别：URL 包含 URI，URI 只代表资源的名称，无法用它定位资源
  - 协议版本号

- 请求头

- 空白行
  - 用于分隔请求头和请求体

- 请求体
  - 向服务器发送的具体信息，用 post 请求时会在此写入内容

具体报文格式

```
// 请求行
GET /test HTTP/1.1
// 请求头
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
Accept-Encoding: gzip, deflate, br
Accept-Language: zh-CN,zh;q=0.9,en;q=0.8
Connection: keep-alive
Cookie: username-localhost-8888="2|1:0|10:1660924176|23:username-localhost-8888|44:OGU1NTY0NWFjZWI5NGQyYjgyNmY3NjljNmNiNWNjMzU=|87caaff9fba6310d052b966728a79a73f9ae5d44576173bf3d81e8bf0d950b00"; freePromorunningtmr=; isfreeretainend=; discount_free_trigger=; JSESSIONID=ADDB531A74538ECA9EF9B8AB7FAC5AAB
Host: localhost:8080
Sec-Fetch-Dest: document
Sec-Fetch-Mode: navigate
Sec-Fetch-Site: none
Sec-Fetch-User: ?1
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36
sec-ch-ua: "Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "Windows"
// 空白行

// 请求体，如果以 post 做请求方式此处会呈现内容
...
```



#### HTTP的响应协议包括：4部分

- 状态行
  - 协议版本号（如 HTTP/1.1）
  - 状态码
    - 200： 表示请求成功
    - 404：资源不存在
    - 405：前后端请求与处理方式不一致，如前端用 get 请求，后端用 post 处理
    - 500：服务器内部错误
  - 状态描述信息
    - ok 表示成功
- 响应头
- 空白行
  - 用于分隔响应头和响应体
- 响应体
  - 本质上是一串长字符串，由浏览器解析

具体报文格式

```
// 状态行
HTTP/1.1 200 ok
// 响应头
Content-Length: 53						
Date: Tue, 13 Sep 2022 07:30:08 GMT
Keep-Alive: timeout=20
Connection: keep-alive
// 空白行

// 响应体
...
```



### HttpServlet

这个 Servlet 是 HTTP 协议专用的 Servlet，对应的专用请求对象是 HttpServletRequest 和 HttpServletResponse，Web 服务器将请求中的所有内容解析，并将其封装到 HttpServletRequest 中，程序员只需要调用对应方法即可取得对应内容



HttpServlet 是一个抽象类，继承了 GenericServlet。

```java
public abstract class HttpServlet extends GenericServlet {
	
}
```

同时 HttpServlet 重写了 `service` 方法，他会在内部将请求和响应类型转换，并调用自己的 `service` 方法

```java
@Override
public void service(ServletRequest req, ServletResponse res)
    throws ServletException, IOException
{
    HttpServletRequest  request;
    HttpServletResponse response;

    if (!(req instanceof HttpServletRequest &&
          res instanceof HttpServletResponse)) {
        // 如果不是 HTTP 协议，抛出异常
        throw new ServletException("non-HTTP request or response");
    }

    request = (HttpServletRequest) req;
    response = (HttpServletResponse) res;

    service(request, response);
}
```

再在自定义的 `service` 中分别就请求方法分别处理。如对于 GET 分派给 `doGet` 处理，在该类中提供了一个默认的 `doGet` 实现，它仅仅实现了对方法的判断。

该方法是期望子类去重写的，换言之，如果子类不重写，就会调用到 HttpServlet 的同名方法，从而抛出异常，代表后端服务器未实现该方法的处理

```java
protected void doGet(HttpServletRequest req, HttpServletResponse resp)
    throws ServletException, IOException
{
    String protocol = req.getProtocol();
    String msg = lStrings.getString("http.method_get_not_supported");
    if (protocol.endsWith("1.1")) {
        // 返回 405 错误
        resp.sendError(HttpServletResponse.SC_METHOD_NOT_ALLOWED, msg);
    } else {
        resp.sendError(HttpServletResponse.SC_BAD_REQUEST, msg);
    }
}
```





### HttpServletRequest

HttpServletRequest 是一个接口，是 Servlet 规范的一员，父接口是 ServletRequest，由服务器实现，并由服务器传入对应的 `service` 接口中。使得 webapp 得以获取前端发送的各种参数

每一次请求都会创建一个 ServletRequest，该对象只在当前对象有效。

获取前端用户提交的数据，整个数据是以一个 `Map<String,String[]>` 的形式存在：

- `Map<String,String[]> getParameterMap)` 这个是获取Map
- `Enumeration<String> getParameterNames()` 这个是获取Map集合中所有的key
- `String[] getParameterValues(String name)` 根据 key 获取Map集合的 value，key 不存在则返回 `null`
- `String getParameter(String name)` 获取 value 这个一维数组当中的**第一个**元素。这个方法最常用。（建议确定该 value 只有一个元素才使用此方法）

```java
Map<String,String[]> name 存储String; value 存储String[]
    
name			value
________________________
username		{"abc"}
userpwd			{"111"}
other			{"s","d","tt"}
```



HttpServletRequest  又称为请求域对象，与此相关的还有应用域（ServletContext），同样他还有对应的存取方法

```java
void setAttribute(String name, Object o);
void removeAttribute(String name);
Object getAttribute(String name);
```

请求对象创建时，请求域对象被创建；请求对象被销毁时请求域对象被销毁。不同的请求对象请求域不同。

如果想要将两个请求串联，就需要用到请求转发机制



#### 请求转发

转发被视为一次请求

```java
//第一步：获取请求转发器对象，参数是另一个 Servlet 的请求路径
RequestDispatcher dispatcher = request.getRequestDispatcher ("/b");
//第二步：调用转发器的 forward() 方法完成跳转/转发
dispatcher.forward(request,response);

//第一步和第二步代码可以联合在一起:
request.getRequestDispatcher ("/b").forward(request,response);
```

此时可以在另一个 `Servlet` 中取得上一个 `Servlet` 设置的对象（setAttribute），换言之这种方式使得两个 Servlet 得以通信

转发的下一个资源不一定是一个Servlet 请求路径：只要是 Tomcat 服务器当中的合法资源，都是可以转发的。例如：html、jsp



#### 常用方法

```java
// 获取客户端 IP 
String remoteAddr = req.getRemoteAddr();

// 设置请求体的编码方式,如果不存在会抛出 UnsupportedEncodingException 异常，
// 这种一般用于解决 POST 方法乱码
void setCharacterEncoding("UTF-8");

// 获取方法
String method = req.getMethod();

// 获取请求URI(不带项目名)
String URI = req.getRequestURI();

// 获取 Servlet Path(带项目名)
String servletPath = req.getServletPath();
```



对于 GET 请求编码方式：在 Tomcat_home/conf/server.xml 中的 Connector 标签中，加入 `URIEncoding="xxx"`, 不写自 Tomcat8 之后默认`"UTF-8"`。

```xml
<Connector URIEncoding="UTF-8"/>
```



### HttpServletResponse

常用方法

```java
// 同样的，响应也有相应的设置方式
void setContentType("text/html;charset=UTF-8");
```

