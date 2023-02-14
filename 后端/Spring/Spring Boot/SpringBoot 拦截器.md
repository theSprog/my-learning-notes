### Interceptor

拦截器(Interceptor) 同 Filter 过滤器一样，它俩都是面向切面编程——AOP 的具体实现（AOP切面编程只是一种编程思想而已）。

你可以使用 Interceptor 来执行某些任务，例如在 Controller 处理请求之前编写日志，添加或更新配置…

在 Spring中，当请求发送到 Controller 时，在被Controller处理之前，它必须经过 Interceptors（0或多个）。

Spring Interceptor是一个非常类似于Servlet Filter 的概念





### 使用

使用拦截器只需要实现 `HandlerInterceptor` 即可。

每一个拦截方法的返回值具有是否放行的意义

- true：请求放行
- false：请求被拦截，可以继续访问

```java
public class MyInterceptor implements HandlerInterceptor {
    // 拦截请求，在进入 controller 之前
    @Override
    public boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler) throws Exception {
       // xxx
    }
    
     // 拦截请求，在进入 controller 之后
    @Override
    public boolean postHandle(HttpServletRequest request, HttpServletResponse response, Object handler) throws Exception {
       // xxx
    }
    
    @Override
    public boolean afterCompletion(HttpServletRequest request, HttpServletResponse response, Object handler) throws Exception {
       // xxx
    }
}
```



实现拦截器之后，注册拦截器

```java
@Configuration
public class WebConfig implements WebMvcConfigurer {
    @Override
    public void addInterceptors(InterceptorRegistry registry) {
        // addPathPatterns 增加需要拦截的请求
        // excludePathPatterns 排除需要拦截的请求
        registry.addInterceptor(new MyInterceptor()).addPathPatterns("/admin/*").excludePathPatterns("/admin/login");
    }
}
```

