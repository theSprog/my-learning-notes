### 会话

会话对应的英语单词：session。

会话是两个或多个通信设备之间，或计算机与用户之间**临时的、交互式的**信息交换。会话在某个时间点建立，然后在稍后的某个时间点被结束。session 最重要的作用是保存会话信息（因为 HTTP 协议是无状态的协议）

一个 session 包含多次请求（request）



### 获取Session对象

在 Java 规范中 session 对象对应的类名是 `HttpSession`

```java
public HttpSession getSession();
public HttpSession getSession(boolean create);
```

这行代码很神奇：每一个用户获取 Session 都是获取的自己的 Session。如果没有获取到则新建一个对象并返回。`getSession(false)`  若获取不到当前请求的 Session 则不会创建，返回 `null`。

每一个 Session 对象都有一个会话期，一旦超过就会将此 Session 对象销毁。

一般而言用户登录时会创建会话（` getSession()`），之后的私人相关业务都要 session 对象（` getSession(false)`），没有则视为非法操作。



### session的实现原理

在web服务器中有一个session列表。类似于map集合。这个map集合的 key 存储的是 sessionID，这个map集合的value存储的是对应的 session 对象



第一次 session 请求的时候：

服务器会创建一个新的 session 对象，同时给session对象生成一个 ID，然后 web 服务器会将 session 的 ID发送给浏览器，浏览器将 session 的 ID保存在浏览器的缓存中。

第二次 session 请求的时候：

会自动将浏览器缓存中的 sessionID 自动发送给服务器，服务器获取到 sessionID, 然后从session 列表中查找到对应的 session 对象。



第一次请求的**响应头**中携带着 cookie 对象，cookie 中包含 sessionID。之后的请求的**请求头** cookie 中包含 sessionID，而响应体却不包含



### session 销毁

- 超时销毁：超过一定时间自动销毁，可以在 `web.xml` 中配置 session 超时时间，单位是分钟

  ```xml
  <!--session的超时时长是30分钟。-->
  <!--如果30分钟过去了，session对象仍然没有被访问，session对象会被销毁-->
  <session-config>
  	<session-timeout>30</session-timeout>
  </session-config>
  ```

  

- 手动销毁：如网站设置的 `logout`

  ```java
  session.invalidate();
  ```

  



### Cookie

#### 生成

cookie 是由服务器端生成的



#### 保存

cookie 最终保存在浏览器客户端中

- 可以保存在运行内存中（浏览器进程关闭就消失）
- 也可以保存在硬盘文件中，一般浏览器提供的 cookie 清除就是将硬盘的 cookie 清除



#### 作用

cookie 和 session 都是为了保存会话的状态

- cookie 将会话状态保存在浏览器客户端上
- session 将会话状态保存在服务器端上

为什么需要 cookie 和 session，是因为 HTTP 是无状态协议，所以由客户端和服务端来提供状态机制

HTTP协议中规定：任何一个cookie都是由name和value组成的。name和value都是**字符串类型**的。



### Java Cookie

- Java 中提供一个类以键值对的形式专门表示 cookie：

  - ```java
    new Cookie("cookieName", "cookieValue")
    ```

  - HTTP 是这样规定的：当访问某一路径返回 cookie 时，浏览器保存此cookie，下次再次访问该路径则浏览器自动在请求中添加 cookie，并发送给服务端

  - cookie 可以设置路径，之后浏览器请求该路径与该路径的子路径时都会发送cookie

    ```java
    cookie.setPath("/servlet")
    ```

    

- 将 cookie 数据发送给浏览器：

  - ```java
    response.addCookie(cookie);
    response.addCookie(cookie2);   
    ```

- 服务端接收 cookies

  - ```java
    Cookie[] cookies = request.getCookies()
    ```

  - 没有 cookie 时返回 `null`，而不是一个空数组

  

- `setMaxAge()`：设置 cookie 有效期，单位是秒（second）。
  - 没有设置有效时间则默认存储到浏览器运行内存中。
  - 只要设置过期时间**大于 0**（哪怕是 1），cookie 都会存储到磁盘中
  - 设置过期时间**小于 0**，表示该 cookie 不会被存储到硬盘中，和不调用 `setMaxAge` 是同一个效果
  - `setMaxAge(0)` 表示删除浏览器上同名 cookie



### Cookie 禁用

Cookie 禁用是指服务器发送的 cookie 浏览器拒收，而不是服务器不发送 cookie 了。

这会导致一个问题：每一次访问都被视为一次新的请求，从而可能在服务端新生成Session对象。而服务端暂时不会销毁这些对象，客户端又不使用 cookie 访问这些对象，相当于这些对象 "内存泄露" 了。当然会话超时这些对象仍会被回收

#### URL重写机制

如果禁用了 cookie，如何实现cookie机制？

在URL之后加上参数 `;jsessionid=xxx`。URL重写机制会提高开发者的成本。开发人员在编写任何请求路径的时候，后面都要添加一个 `jsessionid`，给开发带来了很大的难度和成本。所以大部分的网站都是这样设计的：如果禁用cookie，网站直接罢工

