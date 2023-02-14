## 参数注解

### @PathVariable

用于获取请求路径中的值

`xxx` 必须要和被指定的路径域一致

```java
@RequestMapping("/download/{xxx}")
@PathVariable String xxx;	// 
```



### @RequestParam

用于获得URL中的请求参数，如果参数变量名保持一致，该注解可以省略。不一致时需要手动传入要获取的参数名

```java
@RequestParam("file") String fileName,
```



### @RequestBody

用于获取请求体内部数据

```java
@RequestBody Map<String, Object> map
```



### @RequestHeader

获取请求头的参数

```java
@RequestHeader("token") String token
```

还有另一种方式是通过请求获得，在处理接口中加入参数 `HttpServletRequest req`

```java
String token = req.getHeader("token")
```



### @CookieValue

获取 cookie 中的参数

```java
@CookieValue("xxx") String xxx
```



