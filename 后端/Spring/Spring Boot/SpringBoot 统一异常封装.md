### @RestControllerAdvice

首先，`RestControllerAdvice `本质上是一个`Component`，因此也会被当成组建扫描。

一般是 `@RestControllerAdvice` 配合 `@ExceptionHandler` 实现全局异常处理

```java
@RestControllerAdvice
public class CommonExceptionHandler {
    
    // @ExceptionHandler 设定要拦截的异常类
    // 在参数中可以获取该异常，从而针对不同的异常返回前端不同的响应
	@ExceptionHandler(value = BindException.class)
    public CommonResponse validExceptionHandler(BindException e) {
		// xxx
    }
}
```





### 文件过大异常

springboot 文件过大会触发 `MaxUploadSizeExceededException` 异常

```java
@ExceptionHandler(MaxUploadSizeExceededException.class)
public CommonResponse fileSizeExceededHandler(MaxUploadSizeExceededException e) {
	//    xxx
}
```

如果没有处理此异常，springboot 会继续寻找，可能触发
