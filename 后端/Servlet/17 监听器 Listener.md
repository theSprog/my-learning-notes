### 什么是监听器？

- 监听器是Servlet规范中的一员。就像Filter一样。Filtert 也是 Servlet 规范中的一员。
- 在Servlet中，所有的监听器接口都是以`Listener`结尾。
- 监听器实际上是 Servlet 规范留给我们 Javaweb 程序员的特殊时机。特殊的时刻如果想执行这段代码，你需要想到使用对应的监听器。



### 实现 `Listener` 步骤

- 编写一个类实现 Listener 接口（以 `ServletContextListener` 为例，该监听器监听 `ServletContext` ）

  ```java
  pulic class ABC implements ServletContextListener{
      @Override
      public void contextInitialized(ServletContextEvent sce) {
          // 该方法在 ServletContext 被创建时调用
          // ...
      }
  
      @Override
      public void contextDestroyed(ServletContextEvent sce) {
          // 该方法在 ServletContext 被销毁时调用
          // ...
      }
  }
  ```

- 注册该类为 `Listener`。可以使用注解 `@WebListener`。也可以通过 `web.xml` 

  ```xml
  <listener>
  	<listener-class>xxx</listener-class>
  </listener>
  ```

- 当某个特殊的事件发生之后，由容器来自动调用

  - `ServletRequestListener` 是监听 request 请求对象的，对象创建和销毁时分别有方法执行

  - `HttpSessionListener` 是监听 `session` 对象的，对象创建和销毁时分别有方法执行

  - `HttpSessionBindingListener`  由用户类去实现，当该类的对象绑定到 session 对象上时（调用 `session.addAttribute()` 方法），会调用被绑定对象的监听方法

    - 注意，该对象不用注册监听器，不需要 `@WebListener` 之类的注解

    ```java
    public interface HttpSessionBindingListener extends EventListener {
        // 绑定数据时调用
        default public void valueBound(HttpSessionBindingEvent event) {}
        
        // 解绑数据时调用
        default public void valueUnbound(HttpSessionBindingEvent event) {}
    }
    ```

    

  



