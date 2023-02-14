### 作用

我们可以把`Servlet`程序看做是一个最终要执行的目标。

我们可以使用过滤器 `Filter` 来添加过滤代码，这个过滤代码可以添加到`Servlet`执行之前，也可以添加到`Servlet`执行之后。

换句话说：`Filter`可以做到在`Servlet`这个目标程序执行之前过滤，也可以在目标程序执行之后过滤。

 `Filter` 可以使得 `Servlet` 中的重复代码得以集中，提高代码复用率



### 使用 Filter

- 编写 Java 类，实现 Filter 接口下所有方法

  ```java
  // 匹配所有路径
  @WebFilter("/*")
  public class MyFilter implements Filter {
      // 该方法在 Fliter 创建时被调用并且只执行一次
      @Override
      public void init(FilterConfig filterConfig)throws ServletException){
  		
      }
      
  	// 用户发送一次请求则执行一次，在这个方法中编写过滤规则
      @Override
      public void doFilter(ServletRequest request,ServletResponse response,FilterChain chain) 
          throws I0Exception,ServletException{
          //此处是目标程序执行之前过滤
          
          // 执行下一个 Filter，如果下一个不是过滤器，则执行对应的 servlet，
          // 要想请求传递下去必须加这句话
          chain.doFilter(request, response);
          
          //此处是目标程序执行之后过滤
      }
  
      // 该方法在 Fliter 销毁时调用并且只执行一次
      @Override
      public void destroy(){
         
      }  
  }
  ```

- 注意

  - Servlet 对象默认情况下，在服务器启动的时候是**不会新建**对象的。
  - Filter对象默认情况下，在服务器启动的时候**会新建**对象。
  - Servlet是单例的，Filter 也是单例的。

- 如果不指定 Filter 顺序，那么 Filter 执行顺序取决于类名在字典当中的顺序
  -  `FilterA` < `FilterB`，则 `FilterA` 先执行
  -  `Filter1` < `Filter2`，则 `Filter1` 先执行



### 责任链模式

责任链模式最大的特点就是在程序运行阶段能动态的调整程序的执行顺序

服务器实现 `FilterChain` 接口，在其中设定每一个 Filter 的前后顺序

```java
// 该接口由容器实现
public interface FilterChain {
    public void doFilter(ServletRequest request, ServletResponse response) throws IOException, ServletException;
}
```

Filter 对象可以使用 FilterChain 对象调用链中下一个 Filter 的 doFilter() 方法，若该 Filter 是链中最后一个过滤器，则调用目标资源的 service() 方法

换句话说容器实现的 FilterChain 伪码逻辑如下

```java
public TomcatFilterChain implements FilterChain {    
    @Override
    public void doFilter(ServletRequest request, ServletResponse response) throws IOException, ServletException {
        Filter next = getNext();
        if(next != null){	// 存在后继 Filter
            next.doFilter();
        }else{	// 已经没有下一个 Filter 了
            Servlet servlet = getServlet();
            servlet.service();
        }
    }
}
```

Tomcat 容器负责

- 管理好每一个 Filter 的前后继关系

- 将自己实现的 `TomcatFilterChain` 对象传入 `MyFilter` 中的 `doFilter` 方法的第三个参数

